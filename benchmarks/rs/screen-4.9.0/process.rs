use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn crypt(
        __key: *const libc::c_char,
        __salt: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn bcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn rindex(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut DefaultEsc: libc::c_int;
    static mut DefaultMetaEsc: libc::c_int;
    fn SetCanvasWindow(_: *mut canvas, _: *mut win);
    fn RethinkDisplayViewports() -> libc::c_int;
    fn RecreateCanvasChain();
    fn ResizeCanvas(_: *mut canvas);
    fn FindCanvas(_: libc::c_int, _: libc::c_int) -> *mut canvas;
    fn AddCanvas(_: libc::c_int) -> libc::c_int;
    fn RemCanvas();
    fn OneCanvas();
    fn RethinkViewportOffsets(_: *mut canvas);
    fn CountCanvasPerp(_: *mut canvas) -> libc::c_int;
    fn EqualizeCanvas(_: *mut canvas, _: libc::c_int);
    fn LoadLayout(_: *mut layout, _: *mut canvas);
    fn NewLayout(_: *mut libc::c_char, _: libc::c_int);
    fn SaveLayout(_: *mut libc::c_char, _: *mut canvas);
    fn ShowLayouts(_: libc::c_int);
    fn FindLayout(_: *mut libc::c_char) -> *mut layout;
    fn UpdateLayoutCanvas(_: *mut canvas, _: *mut win);
    fn CreateLayout(_: *mut libc::c_char, _: libc::c_int) -> *mut layout;
    fn RemoveLayout(_: *mut layout);
    fn LayoutDumpCanvas(_: *mut canvas, _: *mut libc::c_char) -> libc::c_int;
    fn RenameLayout(_: *mut layout, _: *const libc::c_char);
    fn RenumberLayout(_: *mut layout, _: libc::c_int) -> libc::c_int;
    fn WindowChangeNumber(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    static mut strnomem: [libc::c_char; 0];
    fn Detach(_: libc::c_int);
    fn Hangup();
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn QueryMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Dummy(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Finit(_: libc::c_int);
    fn MakeNewEnv();
    fn MakeWinMsg(
        _: *mut libc::c_char,
        _: *mut win,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn AddWinMsgRend(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn setbacktick(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    );
    fn ResetAnsiState(_: *mut win);
    fn ResetCharsets(_: *mut win);
    fn WriteString(_: *mut win, _: *mut libc::c_char, _: libc::c_int);
    fn ChangeAKA(_: *mut win, _: *mut libc::c_char, _: libc::c_int);
    fn SetCharsets(_: *mut win, _: *mut libc::c_char);
    fn GetAnsiStatus(_: *mut win, _: *mut libc::c_char) -> libc::c_int;
    fn WBell(_: *mut win, _: libc::c_int);
    fn WindowChanged(_: *mut win, _: libc::c_int);
    fn RcLine(_: *mut libc::c_char, _: libc::c_int);
    fn WriteFile(_: *mut acluser, _: *mut libc::c_char, _: libc::c_int);
    fn ReadFile(_: *mut libc::c_char, _: *mut libc::c_int) -> *mut libc::c_char;
    fn KillBuffers();
    fn RunBlanker(_: *mut *mut libc::c_char);
    fn do_source(_: *mut libc::c_char);
    fn SetTTY(_: libc::c_int, _: *mut mode);
    fn SetFlow(_: libc::c_int);
    fn SendBreak(_: *mut win, _: libc::c_int, _: libc::c_int);
    fn TtyGrabConsole(
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn TtyGetModemStatus(_: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char;
    fn GetHistory() -> libc::c_int;
    fn MarkRoutine();
    fn MakePaster(_: *mut paster, _: *mut libc::c_char, _: libc::c_int, _: libc::c_int);
    fn FreePaster(_: *mut paster);
    fn inp_setprompt(_: *mut libc::c_char, _: *mut libc::c_char);
    fn Input(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut libc::c_char) -> (),
        >,
        _: *mut libc::c_char,
        _: libc::c_int,
    );
    fn display_help(_: *mut libc::c_char, _: *mut action);
    fn display_copyright();
    fn display_displays();
    fn display_bindkey(_: *mut libc::c_char, _: *mut action);
    fn MakeWindow(_: *mut NewWindow) -> libc::c_int;
    fn FreeWindow(_: *mut win);
    fn winexec(_: *mut *mut libc::c_char) -> libc::c_int;
    fn FreePseudowin(_: *mut win);
    fn DoStartLog(_: *mut win, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn zmodem_abort(_: *mut win, _: *mut display);
    fn SlotToggle(_: libc::c_int);
    fn MakeTermcap(_: libc::c_int) -> *mut libc::c_char;
    fn gettermcapstring(_: *mut libc::c_char) -> *mut libc::c_char;
    fn remap(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn CheckEscape();
    fn ClearAll();
    fn Redisplay(_: libc::c_int);
    fn RedisplayDisplays(_: libc::c_int);
    fn ShowHStatus(_: *mut libc::c_char);
    fn RefreshHStatus();
    fn CursorVisibility(_: libc::c_int);
    fn MouseMode(_: libc::c_int);
    fn RemoveStatus();
    fn ResizeDisplay(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn ResetIdle();
    fn ChangeWindowSize(
        _: *mut win,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn ChangeScreenSize(_: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn xrealloc(_: *mut libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn ResizeLayersToCanvases();
    fn ResizeLayer(_: *mut layer, _: libc::c_int, _: libc::c_int, _: *mut display);
    fn MayResizeLayer(_: *mut layer) -> libc::c_int;
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn SetTimeout(_: *mut event, _: libc::c_int);
    fn chsock() -> libc::c_int;
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn SaveStrn(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn Filename(_: *mut libc::c_char) -> *mut libc::c_char;
    fn AddXChars(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn xsetenv(_: *mut libc::c_char, _: *mut libc::c_char);
    fn AclCheckPermWin(_: *mut acluser, _: libc::c_int, _: *mut win) -> libc::c_int;
    fn AclCheckPermCmd(_: *mut acluser, _: libc::c_int, _: *mut comm) -> libc::c_int;
    fn AclUmask(
        _: *mut acluser,
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn UsersAcl(
        _: *mut acluser,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn AclWinSwap(_: libc::c_int, _: libc::c_int);
    fn DoSu(
        _: *mut *mut acluser,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn AclLinkUser(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int;
    fn UserFreeCopyBuffer(_: *mut acluser) -> libc::c_int;
    fn FindUserPtr(_: *mut libc::c_char) -> *mut *mut acluser;
    fn UserDel(_: *mut libc::c_char, _: *mut *mut acluser) -> libc::c_int;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn FromUtf8(_: libc::c_int, _: *mut libc::c_int) -> libc::c_int;
    fn ToUtf8(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn LoadFontTranslation(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    fn WinSwitchEncoding(_: *mut win, _: libc::c_int);
    fn FindEncoding(_: *mut libc::c_char) -> libc::c_int;
    fn EncodingName(_: libc::c_int) -> *mut libc::c_char;
    fn EncodingDefFont(_: libc::c_int) -> libc::c_int;
    fn RecodeBuf(
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn logfclose(_: *mut logfile) -> libc::c_int;
    fn display_windows(onblank: libc::c_int, order: libc::c_int, group: *mut win);
    static mut comms: [comm; 0];
    static mut rc_name: *mut libc::c_char;
    static mut home: *mut libc::c_char;
    static mut BellString: *mut libc::c_char;
    static mut ActivityString: *mut libc::c_char;
    static mut ShellProg: *mut libc::c_char;
    static mut ShellArgs: [*mut libc::c_char; 0];
    static mut hstatusstring: *mut libc::c_char;
    static mut captionstring: *mut libc::c_char;
    static mut timestring: *mut libc::c_char;
    static mut wliststr: *mut libc::c_char;
    static mut wlisttit: *mut libc::c_char;
    static mut captionalways: libc::c_int;
    static mut queryflag: libc::c_int;
    static mut hardcopydir: *mut libc::c_char;
    static mut screenlogfile: *mut libc::c_char;
    static mut logtstamp_string: *mut libc::c_char;
    static mut log_flush: libc::c_int;
    static mut logtstamp_on: libc::c_int;
    static mut logtstamp_after: libc::c_int;
    static mut VisualBellString: *mut libc::c_char;
    static mut VBellWait: libc::c_int;
    static mut MsgWait: libc::c_int;
    static mut MsgMinWait: libc::c_int;
    static mut SilenceWait: libc::c_int;
    static mut SockPath: [libc::c_char; 0];
    static mut SockName: *mut libc::c_char;
    static mut auto_detach: libc::c_int;
    static mut use_altscreen: libc::c_int;
    static mut iflag: libc::c_int;
    static mut maxwin: libc::c_int;
    static mut focusminwidth: libc::c_int;
    static mut focusminheight: libc::c_int;
    static mut use_hardstatus: libc::c_int;
    static mut visual_bell: libc::c_int;
    static mut attr2color: [[libc::c_int; 4]; 0];
    static mut nattr2color: libc::c_int;
    static mut hardstatusemu: libc::c_int;
    static mut printcmd: *mut libc::c_char;
    static mut default_startup: libc::c_int;
    static mut defobuflimit: libc::c_int;
    static mut defnonblock: libc::c_int;
    static mut defmousetrack: libc::c_int;
    static mut ZombieKey_destroy: libc::c_int;
    static mut ZombieKey_resurrect: libc::c_int;
    static mut ZombieKey_onerror: libc::c_int;
    static mut defautonuke: libc::c_int;
    static mut separate_sids: libc::c_int;
    static mut nwin_default: NewWindow;
    static mut nwin_undef: NewWindow;
    static mut join_with_cr: libc::c_int;
    static mut compacthist: libc::c_int;
    static mut search_ic: libc::c_int;
    static mut pastefont: libc::c_int;
    static mut mark_key_tab: [libc::c_uchar; 0];
    static mut BufferFile: *mut libc::c_char;
    static mut PowDetachString: *mut libc::c_char;
    static mut EffectiveAclUser: *mut acluser;
    static mut term: [term; 0];
    static mut kmapdef: [*mut libc::c_char; 0];
    static mut kmapadef: [*mut libc::c_char; 0];
    static mut kmapmdef: [*mut libc::c_char; 0];
    static mut mchar_so: mchar;
    static mut renditions: [libc::c_int; 0];
    static mut VerboseCreate: libc::c_int;
    static mut screenencodings: *mut libc::c_char;
    static mut cjkwidth: libc::c_int;
    static mut flayer: *mut layer;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut fore: *mut win;
    static mut console_window: *mut win;
    static mut windows: *mut win;
    static mut users: *mut acluser;
    static mut layouts: *mut layout;
    static mut layout_attach: *mut layout;
    static mut layout_last_marker: layout;
    static mut screenterm: [libc::c_char; 0];
    static mut HostName: [libc::c_char; 0];
    static mut version: [libc::c_char; 0];
    static mut WinLf: LayFuncs;
    static mut MarkLf: LayFuncs;
    static Z0width: libc::c_int;
    static Z1width: libc::c_int;
    static mut nethackflag: libc::c_int;
    static mut wtab: *mut *mut win;
    static mut multi: *mut libc::c_char;
    static mut maxusercount: libc::c_int;
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
    pub ut_tv: C2RustUnnamed,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub struct comm {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub userbits: [AclBits; 1],
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
    pub l_mouseevent: C2RustUnnamed_1,
    pub l_pause: C2RustUnnamed_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub struct C2RustUnnamed_1 {
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
    pub w_alt: C2RustUnnamed_2,
    pub w_destroyev: event,
    pub w_exitstatus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct term {
    pub tcname: *mut libc::c_char,
    pub type_0: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const NUM_RENDS: C2RustUnnamed_3 = 3;
pub const REND_SILENCE: C2RustUnnamed_3 = 2;
pub const REND_MONITOR: C2RustUnnamed_3 = 1;
pub const REND_BELL: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmap_ext {
    pub str_0: *mut libc::c_char,
    pub fl: libc::c_int,
    pub um: action,
    pub dm: action,
    pub mm: action,
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
pub struct inputsu {
    pub up: *mut *mut acluser,
    pub name: [libc::c_char; 24],
    pub pw1: [libc::c_char; 130],
    pub pw2: [libc::c_char; 130],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct digraph {
    pub d: [libc::c_uchar; 2],
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kclass {
    pub next: *mut kclass,
    pub name: *mut libc::c_char,
    pub ktab: [action; 338],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut NullStr: [libc::c_char; 1] = unsafe {
    *::std::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0")
};
pub static mut plop_tab: [plop; 256] = [plop {
    buf: 0 as *const libc::c_char as *mut libc::c_char,
    len: 0,
    enc: 0,
}; 256];
pub static mut TtyMode: libc::c_int = 0o620 as libc::c_int;
pub static mut hardcopy_append: libc::c_int = 0 as libc::c_int;
pub static mut all_norefresh: libc::c_int = 0 as libc::c_int;
pub static mut zmodem_mode: libc::c_int = 0 as libc::c_int;
pub static mut zmodem_sendcmd: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut zmodem_recvcmd: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut zmodes: [*mut libc::c_char; 4] = [
    b"off\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"auto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"catch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pass\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut idletimo: libc::c_int = 0;
pub static mut idleaction: action = action {
    nr: 0,
    args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    argl: 0 as *const libc::c_int as *mut libc::c_int,
    quiet: 0,
};
pub static mut blankerprg: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut ktab: [action; 338] = [action {
    nr: 0,
    args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    argl: 0 as *const libc::c_int as *mut libc::c_int,
    quiet: 0,
}; 338];
pub static mut kclasses: *mut kclass = 0 as *const kclass as *mut kclass;
pub static mut umtab: [action; 104] = [action {
    nr: 0,
    args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    argl: 0 as *const libc::c_int as *mut libc::c_int,
    quiet: 0,
}; 104];
pub static mut dmtab: [action; 104] = [action {
    nr: 0,
    args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    argl: 0 as *const libc::c_int as *mut libc::c_int,
    quiet: 0,
}; 104];
pub static mut mmtab: [action; 104] = [action {
    nr: 0,
    args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    argl: 0 as *const libc::c_int as *mut libc::c_int,
    quiet: 0,
}; 104];
pub static mut kmap_exts: *mut kmap_ext = 0 as *const kmap_ext as *mut kmap_ext;
pub static mut kmap_extn: libc::c_int = 0;
static mut maptimeout: libc::c_int = 300 as libc::c_int;
static mut digraphs: [digraph; 513] = [
    {
        let mut init = digraph {
            d: [' ' as i32 as libc::c_uchar, ' ' as i32 as libc::c_uchar],
            value: 160 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['N' as i32 as libc::c_uchar, 'S' as i32 as libc::c_uchar],
            value: 160 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['~' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 161 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['!' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 161 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['!' as i32 as libc::c_uchar, 'I' as i32 as libc::c_uchar],
            value: 161 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['c' as i32 as libc::c_uchar, '|' as i32 as libc::c_uchar],
            value: 162 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['c' as i32 as libc::c_uchar, 't' as i32 as libc::c_uchar],
            value: 162 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['$' as i32 as libc::c_uchar, '$' as i32 as libc::c_uchar],
            value: 163 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['P' as i32 as libc::c_uchar, 'd' as i32 as libc::c_uchar],
            value: 163 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, 'x' as i32 as libc::c_uchar],
            value: 164 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['C' as i32 as libc::c_uchar, 'u' as i32 as libc::c_uchar],
            value: 164 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['C' as i32 as libc::c_uchar, 'u' as i32 as libc::c_uchar],
            value: 164 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, 'u' as i32 as libc::c_uchar],
            value: 164 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['Y' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 165 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['Y' as i32 as libc::c_uchar, 'e' as i32 as libc::c_uchar],
            value: 165 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['|' as i32 as libc::c_uchar, '|' as i32 as libc::c_uchar],
            value: 166 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['B' as i32 as libc::c_uchar, 'B' as i32 as libc::c_uchar],
            value: 166 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['p' as i32 as libc::c_uchar, 'a' as i32 as libc::c_uchar],
            value: 167 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['S' as i32 as libc::c_uchar, 'E' as i32 as libc::c_uchar],
            value: 167 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 168 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['\'' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 168 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['c' as i32 as libc::c_uchar, 'O' as i32 as libc::c_uchar],
            value: 169 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['C' as i32 as libc::c_uchar, 'o' as i32 as libc::c_uchar],
            value: 169 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 170 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['<' as i32 as libc::c_uchar, '<' as i32 as libc::c_uchar],
            value: 171 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['-' as i32 as libc::c_uchar, ',' as i32 as libc::c_uchar],
            value: 172 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['N' as i32 as libc::c_uchar, 'O' as i32 as libc::c_uchar],
            value: 172 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['-' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 173 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['r' as i32 as libc::c_uchar, 'O' as i32 as libc::c_uchar],
            value: 174 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['R' as i32 as libc::c_uchar, 'g' as i32 as libc::c_uchar],
            value: 174 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['-' as i32 as libc::c_uchar, '=' as i32 as libc::c_uchar],
            value: 175 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['\'' as i32 as libc::c_uchar, 'm' as i32 as libc::c_uchar],
            value: 175 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['~' as i32 as libc::c_uchar, 'o' as i32 as libc::c_uchar],
            value: 176 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['D' as i32 as libc::c_uchar, 'G' as i32 as libc::c_uchar],
            value: 176 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['+' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 177 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['2' as i32 as libc::c_uchar, '2' as i32 as libc::c_uchar],
            value: 178 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['2' as i32 as libc::c_uchar, 'S' as i32 as libc::c_uchar],
            value: 178 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['3' as i32 as libc::c_uchar, '3' as i32 as libc::c_uchar],
            value: 179 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['3' as i32 as libc::c_uchar, 'S' as i32 as libc::c_uchar],
            value: 179 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['\'' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 180 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['j' as i32 as libc::c_uchar, 'u' as i32 as libc::c_uchar],
            value: 181 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['M' as i32 as libc::c_uchar, 'y' as i32 as libc::c_uchar],
            value: 181 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['p' as i32 as libc::c_uchar, 'p' as i32 as libc::c_uchar],
            value: 182 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['P' as i32 as libc::c_uchar, 'I' as i32 as libc::c_uchar],
            value: 182 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['~' as i32 as libc::c_uchar, '.' as i32 as libc::c_uchar],
            value: 183 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['.' as i32 as libc::c_uchar, 'M' as i32 as libc::c_uchar],
            value: 183 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: [',' as i32 as libc::c_uchar, ',' as i32 as libc::c_uchar],
            value: 184 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['\'' as i32 as libc::c_uchar, ',' as i32 as libc::c_uchar],
            value: 184 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['1' as i32 as libc::c_uchar, '1' as i32 as libc::c_uchar],
            value: 185 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['1' as i32 as libc::c_uchar, 'S' as i32 as libc::c_uchar],
            value: 185 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 186 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['>' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 187 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['1' as i32 as libc::c_uchar, '4' as i32 as libc::c_uchar],
            value: 188 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['1' as i32 as libc::c_uchar, '2' as i32 as libc::c_uchar],
            value: 189 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['3' as i32 as libc::c_uchar, '4' as i32 as libc::c_uchar],
            value: 190 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['~' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 191 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['?' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 191 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['?' as i32 as libc::c_uchar, 'I' as i32 as libc::c_uchar],
            value: 191 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 192 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 192 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 193 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 194 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 194 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 195 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 195 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 196 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 196 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, '@' as i32 as libc::c_uchar],
            value: 197 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, 'A' as i32 as libc::c_uchar],
            value: 197 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['A' as i32 as libc::c_uchar, 'E' as i32 as libc::c_uchar],
            value: 198 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['C' as i32 as libc::c_uchar, ',' as i32 as libc::c_uchar],
            value: 199 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 201 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 202 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 202 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 203 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['E' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 203 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 204 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 204 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 205 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 206 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 206 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 207 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 207 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['D' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 208 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['N' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 209 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['N' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 209 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 210 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 210 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 211 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 212 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 212 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 213 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 213 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 214 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 214 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['/' as i32 as libc::c_uchar, '\\' as i32 as libc::c_uchar],
            value: 215 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['*' as i32 as libc::c_uchar, 'x' as i32 as libc::c_uchar],
            value: 215 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['O' as i32 as libc::c_uchar, '/' as i32 as libc::c_uchar],
            value: 216 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 217 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 217 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 218 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 219 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 219 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['U' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['Y' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 221 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['I' as i32 as libc::c_uchar, 'p' as i32 as libc::c_uchar],
            value: 222 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['T' as i32 as libc::c_uchar, 'H' as i32 as libc::c_uchar],
            value: 222 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['s' as i32 as libc::c_uchar, 's' as i32 as libc::c_uchar],
            value: 223 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['s' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 223 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 224 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 224 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 225 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 226 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 226 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 227 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 227 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 228 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 228 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, 'a' as i32 as libc::c_uchar],
            value: 229 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['a' as i32 as libc::c_uchar, 'e' as i32 as libc::c_uchar],
            value: 230 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['c' as i32 as libc::c_uchar, ',' as i32 as libc::c_uchar],
            value: 231 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 232 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 232 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 233 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 234 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 234 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 235 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['e' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 235 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 236 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 236 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 237 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 238 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 238 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 239 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 239 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['d' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 240 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['n' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 241 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['n' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 241 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 242 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 242 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 243 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 244 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 244 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 245 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '?' as i32 as libc::c_uchar],
            value: 245 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 246 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 246 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: [':' as i32 as libc::c_uchar, '-' as i32 as libc::c_uchar],
            value: 247 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['o' as i32 as libc::c_uchar, '/' as i32 as libc::c_uchar],
            value: 248 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, '`' as i32 as libc::c_uchar],
            value: 249 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, '!' as i32 as libc::c_uchar],
            value: 249 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 250 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, '^' as i32 as libc::c_uchar],
            value: 251 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, '>' as i32 as libc::c_uchar],
            value: 251 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 252 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['u' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 252 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['y' as i32 as libc::c_uchar, '\'' as i32 as libc::c_uchar],
            value: 253 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['i' as i32 as libc::c_uchar, 'p' as i32 as libc::c_uchar],
            value: 254 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['t' as i32 as libc::c_uchar, 'h' as i32 as libc::c_uchar],
            value: 254 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['y' as i32 as libc::c_uchar, '"' as i32 as libc::c_uchar],
            value: 255 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['y' as i32 as libc::c_uchar, ':' as i32 as libc::c_uchar],
            value: 255 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '[' as i32 as libc::c_uchar],
            value: 196 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '\\' as i32 as libc::c_uchar],
            value: 214 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, ']' as i32 as libc::c_uchar],
            value: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '{' as i32 as libc::c_uchar],
            value: 228 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '|' as i32 as libc::c_uchar],
            value: 246 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '}' as i32 as libc::c_uchar],
            value: 252 as libc::c_int,
        };
        init
    },
    {
        let mut init = digraph {
            d: ['"' as i32 as libc::c_uchar, '~' as i32 as libc::c_uchar],
            value: 223 as libc::c_int,
        };
        init
    },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
    digraph { d: [0; 2], value: 0 },
];
static mut resizeprompts: [*mut libc::c_char; 8] = [
    b"resize # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -h # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -v # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -b # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -l # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -l -h # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -l -v # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resize -l -b # lines: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn parse_input_int(
    mut buf: *const libc::c_char,
    mut len: libc::c_int,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if len >= 1 as libc::c_int
        && (*buf as libc::c_int == 'U' as i32
            && *buf.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
            || *buf as libc::c_int == '0' as i32
                && (*buf.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                    || *buf.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'X' as i32))
    {
        x = 0 as libc::c_int;
        i = 2 as libc::c_int;
        while i < len {
            if *buf.offset(i as isize) as libc::c_int >= '0' as i32
                && *buf.offset(i as isize) as libc::c_int <= '9' as i32
            {
                x = x * 16 as libc::c_int
                    | *buf.offset(i as isize) as libc::c_int - '0' as i32;
            } else if *buf.offset(i as isize) as libc::c_int >= 'a' as i32
                && *buf.offset(i as isize) as libc::c_int <= 'f' as i32
            {
                x = x * 16 as libc::c_int
                    | *buf.offset(i as isize) as libc::c_int
                        - ('a' as i32 - 10 as libc::c_int);
            } else if *buf.offset(i as isize) as libc::c_int >= 'A' as i32
                && *buf.offset(i as isize) as libc::c_int <= 'F' as i32
            {
                x = x * 16 as libc::c_int
                    | *buf.offset(i as isize) as libc::c_int
                        - ('A' as i32 - 10 as libc::c_int);
            } else {
                return 0 as libc::c_int
            }
            i += 1;
            i;
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        x = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < len {
            if (*buf.offset(i as isize) as libc::c_int) < '0' as i32
                || *buf.offset(i as isize) as libc::c_int > '7' as i32
            {
                return 0 as libc::c_int;
            }
            x = x * 8 as libc::c_int
                | *buf.offset(i as isize) as libc::c_int - '0' as i32;
            i += 1;
            i;
        }
    } else {
        return 0 as libc::c_int
    }
    *val = x;
    return 1 as libc::c_int;
}
pub static mut noargs: [*mut libc::c_char; 1] = [0 as *const libc::c_char
    as *mut libc::c_char; 1];
pub static mut enter_window_name_mode: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn InitKeytab() {
    let mut i: libc::c_uint = 0;
    let mut argarr: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[action; 338]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<action>() as libc::c_ulong)
    {
        ktab[i as usize].nr = -(1 as libc::c_int);
        ktab[i as usize].args = noargs.as_mut_ptr();
        ktab[i as usize].argl = 0 as *mut libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (188 as libc::c_int - 106 as libc::c_int
            + (188 as libc::c_int - 166 as libc::c_int)) as libc::c_uint
    {
        umtab[i as usize].nr = -(1 as libc::c_int);
        umtab[i as usize].args = noargs.as_mut_ptr();
        umtab[i as usize].argl = 0 as *mut libc::c_int;
        dmtab[i as usize].nr = -(1 as libc::c_int);
        dmtab[i as usize].args = noargs.as_mut_ptr();
        dmtab[i as usize].argl = 0 as *mut libc::c_int;
        mmtab[i as usize].nr = -(1 as libc::c_int);
        mmtab[i as usize].args = noargs.as_mut_ptr();
        mmtab[i as usize].argl = 0 as *mut libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    argarr[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 82 as libc::c_int as libc::c_uint {
        if !(i.wrapping_add(106 as libc::c_int as libc::c_uint)
            < 106 as libc::c_int as libc::c_uint)
        {
            if !(i.wrapping_add(106 as libc::c_int as libc::c_uint)
                >= (106 as libc::c_int + (188 as libc::c_int - 106 as libc::c_int))
                    as libc::c_uint)
            {
                if !(*kmapdef.as_mut_ptr().offset(i as isize)).is_null() {
                    argarr[0 as libc::c_int
                        as usize] = *kmapdef.as_mut_ptr().offset(i as isize);
                    SaveAction(
                        dmtab
                            .as_mut_ptr()
                            .offset(i as isize)
                            .offset((106 as libc::c_int - 106 as libc::c_int) as isize),
                        160 as libc::c_int,
                        argarr.as_mut_ptr(),
                        0 as *mut libc::c_int,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 22 as libc::c_int as libc::c_uint {
        if !(i.wrapping_add(166 as libc::c_int as libc::c_uint)
            < 166 as libc::c_int as libc::c_uint)
        {
            if !(i.wrapping_add(166 as libc::c_int as libc::c_uint)
                >= (166 as libc::c_int + (188 as libc::c_int - 166 as libc::c_int))
                    as libc::c_uint)
            {
                if !(*kmapadef.as_mut_ptr().offset(i as isize)).is_null() {
                    argarr[0 as libc::c_int
                        as usize] = *kmapadef.as_mut_ptr().offset(i as isize);
                    SaveAction(
                        dmtab
                            .as_mut_ptr()
                            .offset(i as isize)
                            .offset(
                                (166 as libc::c_int - 166 as libc::c_int
                                    + (188 as libc::c_int - 106 as libc::c_int)) as isize,
                            ),
                        160 as libc::c_int,
                        argarr.as_mut_ptr(),
                        0 as *mut libc::c_int,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 64 as libc::c_int as libc::c_uint {
        if !(i.wrapping_add(106 as libc::c_int as libc::c_uint)
            < 106 as libc::c_int as libc::c_uint)
        {
            if !(i.wrapping_add(106 as libc::c_int as libc::c_uint)
                >= (106 as libc::c_int + (188 as libc::c_int - 106 as libc::c_int))
                    as libc::c_uint)
            {
                if !(*kmapmdef.as_mut_ptr().offset(i as isize)).is_null() {
                    argarr[0 as libc::c_int
                        as usize] = *kmapmdef.as_mut_ptr().offset(i as isize);
                    argarr[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
                    SaveAction(
                        mmtab
                            .as_mut_ptr()
                            .offset(i as isize)
                            .offset((106 as libc::c_int - 106 as libc::c_int) as isize),
                        160 as libc::c_int,
                        argarr.as_mut_ptr(),
                        0 as *mut libc::c_int,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    ktab['h' as i32 as usize].nr = 84 as libc::c_int;
    ktab[('z' as i32 & 0o37 as libc::c_int) as usize].nr = 162 as libc::c_int;
    ktab['z' as i32 as usize].nr = ktab[('z' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('c' as i32 & 0o37 as libc::c_int) as usize].nr = 143 as libc::c_int;
    ktab['c' as i32 as usize].nr = ktab[('c' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('n' as i32 & 0o37 as libc::c_int) as usize].nr = 117 as libc::c_int;
    ktab['n' as i32 as usize].nr = ktab[('n' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[(' ' as i32 & 0o37 as libc::c_int) as usize].nr = ktab['n' as i32 as usize].nr;
    ktab[' ' as i32 as usize].nr = ktab[(' ' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab['N' as i32 as usize].nr = 119 as libc::c_int;
    ktab[('p' as i32 & 0o37 as libc::c_int) as usize].nr = 130 as libc::c_int;
    ktab['p' as i32 as usize].nr = ktab[('p' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[0o177 as libc::c_int as usize].nr = ktab['p' as i32 as usize].nr;
    ktab[('h' as i32 & 0o37 as libc::c_int) as usize]
        .nr = ktab[0o177 as libc::c_int as usize].nr;
    ktab[('k' as i32 & 0o37 as libc::c_int) as usize].nr = 96 as libc::c_int;
    ktab['k' as i32 as usize].nr = ktab[('k' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('l' as i32 & 0o37 as libc::c_int) as usize].nr = 136 as libc::c_int;
    ktab['l' as i32 as usize].nr = ktab[('l' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('w' as i32 & 0o37 as libc::c_int) as usize].nr = 181 as libc::c_int;
    ktab['w' as i32 as usize].nr = ktab[('w' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab['v' as i32 as usize].nr = 177 as libc::c_int;
    ktab[('v' as i32 & 0o37 as libc::c_int) as usize].nr = 68 as libc::c_int;
    ktab[('q' as i32 & 0o37 as libc::c_int) as usize].nr = 186 as libc::c_int;
    ktab['q' as i32 as usize].nr = ktab[('q' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('s' as i32 & 0o37 as libc::c_int) as usize].nr = 185 as libc::c_int;
    ktab['s' as i32 as usize].nr = ktab[('s' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('t' as i32 & 0o37 as libc::c_int) as usize].nr = 167 as libc::c_int;
    ktab['t' as i32 as usize].nr = ktab[('t' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('i' as i32 & 0o37 as libc::c_int) as usize].nr = 94 as libc::c_int;
    ktab['i' as i32 as usize].nr = ktab[('i' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('m' as i32 & 0o37 as libc::c_int) as usize].nr = 97 as libc::c_int;
    ktab['m' as i32 as usize].nr = ktab[('m' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab['A' as i32 as usize].nr = 168 as libc::c_int;
    ktab['L' as i32 as usize].nr = 103 as libc::c_int;
    ktab[',' as i32 as usize].nr = 99 as libc::c_int;
    ktab['W' as i32 as usize].nr = 179 as libc::c_int;
    ktab['.' as i32 as usize].nr = 71 as libc::c_int;
    ktab[('\\' as i32 & 0o37 as libc::c_int) as usize].nr = 133 as libc::c_int;
    ktab[('d' as i32 & 0o37 as libc::c_int) as usize].nr = 67 as libc::c_int;
    ktab['d' as i32 as usize].nr = ktab[('d' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab['D' as i32 as usize].nr = 128 as libc::c_int;
    ktab[('r' as i32 & 0o37 as libc::c_int) as usize].nr = 182 as libc::c_int;
    ktab['r' as i32 as usize].nr = ktab[('r' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('f' as i32 & 0o37 as libc::c_int) as usize].nr = 79 as libc::c_int;
    ktab['f' as i32 as usize].nr = ktab[('f' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab['C' as i32 as usize].nr = 32 as libc::c_int;
    ktab['Z' as i32 as usize].nr = 141 as libc::c_int;
    ktab['H' as i32 as usize].nr = 101 as libc::c_int;
    ktab['M' as i32 as usize].nr = 111 as libc::c_int;
    ktab['?' as i32 as usize].nr = 89 as libc::c_int;
    ktab['*' as i32 as usize].nr = 70 as libc::c_int;
    let mut args: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    args[0 as libc::c_int
        as usize] = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    args[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    SaveAction(
        ktab.as_mut_ptr().offset('-' as i32 as isize),
        145 as libc::c_int,
        args.as_mut_ptr(),
        0 as *mut libc::c_int,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i
        < (if maxwin != 0 && maxwin < 10 as libc::c_int {
            maxwin
        } else {
            10 as libc::c_int
        }) as libc::c_uint
    {
        let mut args_0: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
        let mut arg1: [libc::c_char; 10] = [0; 10];
        args_0[0 as libc::c_int as usize] = arg1.as_mut_ptr();
        args_0[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
        sprintf(arg1.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, i);
        SaveAction(
            ktab.as_mut_ptr().offset('0' as i32 as isize).offset(i as isize),
            145 as libc::c_int,
            args_0.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    ktab['\'' as i32 as usize].nr = 145 as libc::c_int;
    let mut args_1: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    args_1[0 as libc::c_int
        as usize] = b"-b\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    args_1[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    SaveAction(
        ktab.as_mut_ptr().offset('"' as i32 as isize),
        180 as libc::c_int,
        args_1.as_mut_ptr(),
        0 as *mut libc::c_int,
    );
    ktab[('G' as i32 & 0o37 as libc::c_int) as usize].nr = 173 as libc::c_int;
    ktab[':' as i32 as usize].nr = 34 as libc::c_int;
    ktab[('[' as i32 & 0o37 as libc::c_int) as usize].nr = 38 as libc::c_int;
    ktab['[' as i32 as usize].nr = ktab[('[' as i32 & 0o37 as libc::c_int) as usize].nr;
    let mut args_2: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    args_2[0 as libc::c_int
        as usize] = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    args_2[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    SaveAction(
        ktab.as_mut_ptr().offset(']' as i32 as isize),
        125 as libc::c_int,
        args_2.as_mut_ptr(),
        0 as *mut libc::c_int,
    );
    SaveAction(
        ktab.as_mut_ptr().offset((']' as i32 & 0o37 as libc::c_int) as isize),
        125 as libc::c_int,
        args_2.as_mut_ptr(),
        0 as *mut libc::c_int,
    );
    ktab['{' as i32 as usize].nr = 90 as libc::c_int;
    ktab['}' as i32 as usize].nr = 90 as libc::c_int;
    ktab['>' as i32 as usize].nr = 183 as libc::c_int;
    ktab['<' as i32 as usize].nr = 134 as libc::c_int;
    ktab['=' as i32 as usize].nr = 139 as libc::c_int;
    ktab['D' as i32 as usize].nr = 128 as libc::c_int;
    ktab[('x' as i32 & 0o37 as libc::c_int) as usize].nr = 100 as libc::c_int;
    ktab['x' as i32 as usize].nr = ktab[('x' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab[('b' as i32 & 0o37 as libc::c_int) as usize].nr = 21 as libc::c_int;
    ktab['b' as i32 as usize].nr = ktab[('b' as i32 & 0o37 as libc::c_int) as usize].nr;
    ktab['B' as i32 as usize].nr = 127 as libc::c_int;
    ktab['_' as i32 as usize].nr = 151 as libc::c_int;
    ktab['S' as i32 as usize].nr = 158 as libc::c_int;
    ktab['Q' as i32 as usize].nr = 121 as libc::c_int;
    ktab['X' as i32 as usize].nr = 138 as libc::c_int;
    ktab['F' as i32 as usize].nr = 78 as libc::c_int;
    ktab['\t' as i32 as usize].nr = 80 as libc::c_int;
    let mut args_3: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    args_3[0 as libc::c_int
        as usize] = b"prev\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    args_3[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    SaveAction(
        ktab
            .as_mut_ptr()
            .offset(140 as libc::c_int as isize)
            .offset(-(106 as libc::c_int as isize))
            .offset(256 as libc::c_int as isize),
        80 as libc::c_int,
        args_3.as_mut_ptr(),
        0 as *mut libc::c_int,
    );
    let mut args_4: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    args_4[0 as libc::c_int
        as usize] = b"-v\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    args_4[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    SaveAction(
        ktab.as_mut_ptr().offset('|' as i32 as isize),
        158 as libc::c_int,
        args_4.as_mut_ptr(),
        0 as *mut libc::c_int,
    );
    if DefaultEsc >= 0 as libc::c_int {
        ClearAction(&mut *ktab.as_mut_ptr().offset(DefaultEsc as isize));
        ktab[DefaultEsc as usize].nr = 122 as libc::c_int;
    }
    if DefaultMetaEsc >= 0 as libc::c_int {
        ClearAction(&mut *ktab.as_mut_ptr().offset(DefaultMetaEsc as isize));
        ktab[DefaultMetaEsc as usize].nr = 110 as libc::c_int;
    }
    idleaction.nr = 19 as libc::c_int;
    idleaction.args = noargs.as_mut_ptr();
    idleaction.argl = 0 as *mut libc::c_int;
}
unsafe extern "C" fn FindKtab(
    mut class: *mut libc::c_char,
    mut create: libc::c_int,
) -> *mut action {
    let mut kp: *mut kclass = 0 as *mut kclass;
    let mut kpp: *mut *mut kclass = 0 as *mut *mut kclass;
    let mut i: libc::c_int = 0;
    if class.is_null() {
        return ktab.as_mut_ptr();
    }
    kpp = &mut kclasses;
    loop {
        kp = *kpp;
        if kp.is_null() {
            break;
        }
        if strcmp((*kp).name, class) == 0 {
            break;
        }
        kpp = &mut (*kp).next;
    }
    if kp.is_null() {
        if create == 0 {
            return 0 as *mut action;
        }
        if strlen(class) > 80 as libc::c_int as libc::c_ulong {
            Msg(
                0 as libc::c_int,
                b"Command class name too long.\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut action;
        }
        kp = malloc(::std::mem::size_of::<kclass>() as libc::c_ulong) as *mut kclass;
        if kp.is_null() {
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
            return 0 as *mut action;
        }
        (*kp).name = SaveStr(class);
        i = 0 as libc::c_int;
        while i
            < (::std::mem::size_of::<[action; 338]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<action>() as libc::c_ulong)
                as libc::c_int
        {
            (*kp).ktab[i as usize].nr = -(1 as libc::c_int);
            (*kp).ktab[i as usize].args = noargs.as_mut_ptr();
            (*kp).ktab[i as usize].argl = 0 as *mut libc::c_int;
            (*kp).ktab[i as usize].quiet = 0 as libc::c_int;
            i += 1;
            i;
        }
        (*kp).next = 0 as *mut kclass;
        *kpp = kp;
    }
    return ((*kp).ktab).as_mut_ptr();
}
unsafe extern "C" fn ClearAction(mut act: *mut action) {
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*act).nr == -(1 as libc::c_int) {
        return;
    }
    (*act).nr = -(1 as libc::c_int);
    if (*act).args == noargs.as_mut_ptr() {
        return;
    }
    p = (*act).args;
    while !(*p).is_null() {
        free(*p as *mut libc::c_void);
        p = p.offset(1);
        p;
    }
    free((*act).args as *mut libc::c_char as *mut libc::c_void);
    (*act).args = noargs.as_mut_ptr();
    (*act).argl = 0 as *mut libc::c_int;
}
pub unsafe extern "C" fn ProcessInput(
    mut ibuf: *mut libc::c_char,
    mut ilen: libc::c_int,
) {
    let mut ch: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if display.is_null() || ilen == 0 as libc::c_int {
        return;
    }
    if (*display).d_seql != 0 {
        evdeq(&mut (*display).d_mapev);
    }
    slen = ilen;
    s = ibuf as *mut libc::c_uchar;
    loop {
        let fresh0 = ilen;
        ilen = ilen - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        ch = *fresh1 as libc::c_int;
        if (*display).d_dontmap != 0 || (*display).d_nseqs == 0 {
            (*display).d_dontmap = 0 as libc::c_int;
        } else {
            loop {
                if *(*display).d_seqp as libc::c_int != ch {
                    l = *((*display).d_seqp)
                        .offset(
                            (*((*display).d_seqp)
                                .offset((-(*display).d_seql - 1 as libc::c_int) as isize)
                                as libc::c_int + 1 as libc::c_int) as isize,
                        ) as libc::c_int;
                    if l != 0 {
                        (*display)
                            .d_seqp = ((*display).d_seqp)
                            .offset((l * 2 as libc::c_int + 4 as libc::c_int) as isize);
                    } else {
                        (*display).d_mapdefault = 0 as libc::c_int;
                        l = (*display).d_seql;
                        p = ((*display).d_seqp as *mut libc::c_char)
                            .offset(-(l as isize));
                        (*display).d_seql = 0 as libc::c_int;
                        (*display)
                            .d_seqp = ((*display).d_kmaps)
                            .offset(3 as libc::c_int as isize);
                        if l == 0 as libc::c_int {
                            break;
                        }
                        q = (*display).d_seqh;
                        if !q.is_null() {
                            (*display).d_seqh = 0 as *mut libc::c_uchar;
                            i = (*q.offset(0 as libc::c_int as isize) as libc::c_int)
                                << 8 as libc::c_int
                                | *q.offset(1 as libc::c_int as isize) as libc::c_int;
                            i &= !(0x4000 as libc::c_int);
                            if StuffKey(i) != 0 {
                                ProcessInput2(
                                    (q as *mut libc::c_char).offset(3 as libc::c_int as isize),
                                    *q.offset(2 as libc::c_int as isize) as libc::c_int,
                                );
                            }
                            if display.is_null() {
                                return;
                            }
                            l -= *q.offset(2 as libc::c_int as isize) as libc::c_int;
                            p = p
                                .offset(
                                    *q.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                                );
                        } else {
                            (*display).d_dontmap = 1 as libc::c_int;
                        }
                        ProcessInput(p, l);
                        if display.is_null() {
                            return;
                        }
                        evdeq(&mut (*display).d_mapev);
                    }
                } else {
                    let fresh2 = (*display).d_seql;
                    (*display).d_seql = (*display).d_seql + 1;
                    if fresh2 == 0 as libc::c_int {
                        slen -= ilen + 1 as libc::c_int;
                        if slen != 0 {
                            ProcessInput2(ibuf, slen);
                        }
                        if display.is_null() {
                            return;
                        }
                        (*display).d_seqh = 0 as *mut libc::c_uchar;
                    }
                    ibuf = s as *mut libc::c_char;
                    slen = ilen;
                    (*display).d_seqp = ((*display).d_seqp).offset(1);
                    (*display).d_seqp;
                    l = (*display).d_seql;
                    if !(l
                        == *((*display).d_seqp).offset((-l - 1 as libc::c_int) as isize)
                            as libc::c_int)
                    {
                        break;
                    }
                    if *((*display).d_seqp).offset(l as isize) as libc::c_int != l {
                        q = ((*display).d_seqp)
                            .offset(1 as libc::c_int as isize)
                            .offset(l as isize);
                        if ((*display).d_kmaps).offset((*display).d_nseqs as isize) > q
                            && *q.offset(2 as libc::c_int as isize) as libc::c_int > l
                            && bcmp(
                                ((*display).d_seqp).offset(-(l as isize))
                                    as *const libc::c_void,
                                q.offset(3 as libc::c_int as isize) as *const libc::c_void,
                                l as libc::c_ulong,
                            ) == 0
                        {
                            (*display)
                                .d_seqh = ((*display).d_seqp)
                                .offset(-(3 as libc::c_int as isize))
                                .offset(-(l as isize));
                            (*display)
                                .d_seqp = q
                                .offset(3 as libc::c_int as isize)
                                .offset(l as isize);
                            break;
                        }
                    }
                    i = (*((*display).d_seqp).offset((-l - 3 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *((*display).d_seqp).offset((-l - 2 as libc::c_int) as isize)
                            as libc::c_int;
                    i &= !(0x4000 as libc::c_int);
                    p = ((*display).d_seqp as *mut libc::c_char).offset(-(l as isize));
                    (*display).d_seql = 0 as libc::c_int;
                    (*display)
                        .d_seqp = ((*display).d_kmaps).offset(3 as libc::c_int as isize);
                    (*display).d_seqh = 0 as *mut libc::c_uchar;
                    if StuffKey(i) != 0 {
                        ProcessInput2(p, l);
                    }
                    if display.is_null() {
                        return;
                    }
                    break;
                }
            }
        }
    }
    if (*display).d_seql != 0 {
        l = (*display).d_seql;
        s = (*display).d_seqp;
        while !(*s.offset((-l - 3 as libc::c_int) as isize) as libc::c_int
            & 0x4000 as libc::c_int >> 8 as libc::c_int != 0)
        {
            i = *s
                .offset(
                    (*s.offset((-l - 1 as libc::c_int) as isize) as libc::c_int
                        + 1 as libc::c_int) as isize,
                ) as libc::c_int;
            if i == 0 as libc::c_int {
                SetTimeout(&mut (*display).d_mapev, maptimeout);
                evenq(&mut (*display).d_mapev);
                break;
            } else {
                s = s.offset((i * 2 as libc::c_int + 4 as libc::c_int) as isize);
            }
        }
    }
    ProcessInput2(ibuf, slen);
}
pub unsafe extern "C" fn ProcessInput2(
    mut ibuf: *mut libc::c_char,
    mut ilen: libc::c_int,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut ktabp: *mut action = 0 as *mut action;
    while ilen != 0 && !display.is_null() {
        flayer = (*(*display).d_forecv).c_layer;
        fore = (*display).d_fore;
        slen = ilen;
        s = ibuf;
        if ((*display).d_ESCseen).is_null() {
            while ilen > 0 as libc::c_int {
                let fresh3 = s;
                s = s.offset(1);
                if *fresh3 as libc::c_uchar as libc::c_int == (*(*display).d_user).u_Esc
                {
                    break;
                }
                ilen -= 1;
                ilen;
            }
            slen -= ilen;
            if slen != 0 {
                DoProcess(fore, &mut ibuf, &mut slen, 0 as *mut paster);
            }
            ilen -= 1;
            if ilen == 0 as libc::c_int {
                (*display).d_ESCseen = ktab.as_mut_ptr();
                WindowChanged(fore, 'E' as i32);
            }
        }
        if ilen <= 0 as libc::c_int {
            return;
        }
        ktabp = if !((*display).d_ESCseen).is_null() {
            (*display).d_ESCseen
        } else {
            ktab.as_mut_ptr()
        };
        if !((*display).d_ESCseen).is_null() {
            (*display).d_ESCseen = 0 as *mut action;
            WindowChanged(fore, 'E' as i32);
        }
        ch = *s as libc::c_uchar as libc::c_int;
        if ch == (*(*display).d_user).u_Esc {
            ch = DefaultEsc;
        } else if ch == (*(*display).d_user).u_MetaEsc {
            ch = DefaultMetaEsc;
        }
        if ch >= 0 as libc::c_int {
            DoAction(&mut *ktabp.offset(ch as isize), ch);
        }
        ibuf = s.offset(1 as libc::c_int as isize);
        ilen -= 1;
        ilen;
    }
}
pub unsafe extern "C" fn DoProcess(
    mut p: *mut win,
    mut bufp: *mut *mut libc::c_char,
    mut lenp: *mut libc::c_int,
    mut pa: *mut paster,
) {
    let mut oldlen: libc::c_int = 0;
    let mut d: *mut display = display;
    if !pa.is_null() && *lenp > 1 as libc::c_int && !p.is_null() && (*p).w_slowpaste != 0
    {
        SetTimeout(&mut (*p).w_paster.pa_slowev, (*p).w_slowpaste);
        evenq(&mut (*p).w_paster.pa_slowev);
        return;
    }
    while !flayer.is_null() && *lenp != 0 {
        if pa.is_null() && !p.is_null() && (*p).w_paster.pa_pastelen != 0
            && flayer == (*p).w_paster.pa_pastelayer
        {
            WBell(p, visual_bell);
            *bufp = (*bufp).offset(*lenp as isize);
            *lenp = 0 as libc::c_int;
            display = d;
            return;
        }
        oldlen = *lenp;
        (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap())).unwrap()(bufp, lenp);
        if !pa.is_null() && ((*pa).pa_pastelayer).is_null() {
            break;
        }
        if !(*lenp == oldlen) {
            continue;
        }
        if !pa.is_null() {
            display = d;
            return;
        }
        WBell(p, visual_bell);
        break;
    }
    *bufp = (*bufp).offset(*lenp as isize);
    *lenp = 0 as libc::c_int;
    display = d;
    if !pa.is_null() && (*pa).pa_pastelen == 0 as libc::c_int {
        FreePaster(pa);
    }
}
pub unsafe extern "C" fn FindCommnr(mut str: *const libc::c_char) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 189 as libc::c_int;
    while l <= r {
        m = (l + r) / 2 as libc::c_int;
        x = strcmp(str, (*comms.as_mut_ptr().offset(m as isize)).name);
        if x > 0 as libc::c_int {
            l = m + 1 as libc::c_int;
        } else if x < 0 as libc::c_int {
            r = m - 1 as libc::c_int;
        } else {
            return m
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn CheckArgNum(
    mut nr: libc::c_int,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    static mut argss: [*mut libc::c_char; 6] = [
        b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"one\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"two\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"three\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"four\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"OOPS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    static mut orformat: [*mut libc::c_char; 4] = [
        b"%s: %s: %s argument%s required\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"%s: %s: %s or %s argument%s required\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"%s: %s: %s, %s or %s argument%s required\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"%s: %s: %s, %s, %s or %s argument%s required\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    ];
    n = (*comms.as_mut_ptr().offset(nr as isize)).flags & 3 as libc::c_int;
    i = 0 as libc::c_int;
    while !(*args.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 5 as libc::c_int != 0
    {
        if i < n {
            Msg(
                0 as libc::c_int,
                b"%s: %s: at least %s argument%s required\0" as *const u8
                    as *const libc::c_char,
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                if n != 1 as libc::c_int {
                    b"s\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*comms.as_mut_ptr().offset(nr as isize)).flags
            & (1 as libc::c_int) << 3 as libc::c_int != 0
        && (*comms.as_mut_ptr().offset(nr as isize)).flags
            & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        if i != n && i != n + 1 as libc::c_int && i != n + 2 as libc::c_int
            && i != n + 3 as libc::c_int
        {
            Msg(
                0 as libc::c_int,
                orformat[3 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 1 as libc::c_int) as usize],
                argss[(n + 2 as libc::c_int) as usize],
                argss[(n + 3 as libc::c_int) as usize],
                b"\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*comms.as_mut_ptr().offset(nr as isize)).flags
            & (1 as libc::c_int) << 3 as libc::c_int != 0
    {
        if i != n && i != n + 1 as libc::c_int && i != n + 2 as libc::c_int {
            Msg(
                0 as libc::c_int,
                orformat[2 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 1 as libc::c_int) as usize],
                argss[(n + 2 as libc::c_int) as usize],
                b"\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 2 as libc::c_int != 0
        && (*comms.as_mut_ptr().offset(nr as isize)).flags
            & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        if i != n && i != n + 1 as libc::c_int && i != n + 3 as libc::c_int {
            Msg(
                0 as libc::c_int,
                orformat[2 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 1 as libc::c_int) as usize],
                argss[(n + 3 as libc::c_int) as usize],
                b"\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 3 as libc::c_int != 0
        && (*comms.as_mut_ptr().offset(nr as isize)).flags
            & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        if i != n && i != n + 2 as libc::c_int && i != n + 3 as libc::c_int {
            Msg(
                0 as libc::c_int,
                orformat[2 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 2 as libc::c_int) as usize],
                argss[(n + 3 as libc::c_int) as usize],
                b"\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 2 as libc::c_int != 0
    {
        if i != n && i != n + 1 as libc::c_int {
            Msg(
                0 as libc::c_int,
                orformat[1 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 1 as libc::c_int) as usize],
                if n != 0 as libc::c_int {
                    b"s\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 3 as libc::c_int != 0
    {
        if i != n && i != n + 2 as libc::c_int {
            Msg(
                0 as libc::c_int,
                orformat[1 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 2 as libc::c_int) as usize],
                b"s\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if (*comms.as_mut_ptr().offset(nr as isize)).flags
        & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        if i != n && i != n + 3 as libc::c_int {
            Msg(
                0 as libc::c_int,
                orformat[1 as libc::c_int as usize],
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                argss[n as usize],
                argss[(n + 3 as libc::c_int) as usize],
                b"\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else if i != n {
        Msg(
            0 as libc::c_int,
            orformat[0 as libc::c_int as usize],
            rc_name,
            (*comms.as_mut_ptr().offset(nr as isize)).name,
            argss[n as usize],
            if n != 1 as libc::c_int {
                b"s\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        return -(1 as libc::c_int);
    }
    return i;
}
unsafe extern "C" fn StuffFin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    if flayer.is_null() {
        return;
    }
    while len != 0 {
        (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
            .unwrap()(&mut buf, &mut len);
    }
}
pub unsafe extern "C" fn DoAction(mut act: *mut action, mut key: libc::c_int) {
    let mut nr: libc::c_int = (*act).nr;
    let mut args: *mut *mut libc::c_char = (*act).args;
    let mut argl: *mut libc::c_int = (*act).argl;
    let mut p: *mut win = 0 as *mut win;
    let mut argc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut msgok: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut odisplay: *mut display = display;
    let mut user: *mut acluser = 0 as *mut acluser;
    user = if !display.is_null() { (*display).d_user } else { users };
    if nr == -(1 as libc::c_int) {
        return;
    }
    n = (*comms.as_mut_ptr().offset(nr as isize)).flags;
    if n & (1 as libc::c_int) << 9 as libc::c_int == 0 && queryflag >= 0 as libc::c_int {
        if (*act).quiet == 0 {
            Some(
                Msg as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else if queryflag >= 0 as libc::c_int {
            Some(
                QueryMsg
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else {
            Some(
                Dummy
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        }
            .unwrap()(
            0 as libc::c_int,
            b"%s command cannot be queried.\0" as *const u8 as *const libc::c_char,
            (*comms.as_mut_ptr().offset(nr as isize)).name,
        );
        queryflag = -(1 as libc::c_int);
        return;
    }
    if n & (1 as libc::c_int) << 7 as libc::c_int != 0 && display.is_null() {
        if (*act).quiet == 0 {
            Some(
                Msg as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else if queryflag >= 0 as libc::c_int {
            Some(
                QueryMsg
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else {
            Some(
                Dummy
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        }
            .unwrap()(
            0 as libc::c_int,
            b"%s: %s: display required\0" as *const u8 as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset(nr as isize)).name,
        );
        queryflag = -(1 as libc::c_int);
        return;
    }
    if n & (1 as libc::c_int) << 6 as libc::c_int != 0 && fore.is_null() {
        if (*act).quiet == 0 {
            Some(
                Msg as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else if queryflag >= 0 as libc::c_int {
            Some(
                QueryMsg
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else {
            Some(
                Dummy
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        }
            .unwrap()(
            0 as libc::c_int,
            b"%s: %s: window required\0" as *const u8 as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset(nr as isize)).name,
        );
        queryflag = -(1 as libc::c_int);
        return;
    }
    if n & (1 as libc::c_int) << 8 as libc::c_int != 0 && flayer.is_null() {
        if (*act).quiet == 0 {
            Some(
                Msg as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else if queryflag >= 0 as libc::c_int {
            Some(
                QueryMsg
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        } else {
            Some(
                Dummy
                    as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> (),
            )
        }
            .unwrap()(
            0 as libc::c_int,
            b"%s: %s: display or window required\0" as *const u8 as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset(nr as isize)).name,
        );
        queryflag = -(1 as libc::c_int);
        return;
    }
    argc = CheckArgNum(nr, args);
    if argc < 0 as libc::c_int {
        return;
    }
    if !display.is_null() {
        if AclCheckPermCmd(
            (*display).d_user,
            0 as libc::c_int,
            &mut *comms.as_mut_ptr().offset(nr as isize),
        ) != 0
        {
            if (*act).quiet == 0 {
                Some(
                    Msg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else if queryflag >= 0 as libc::c_int {
                Some(
                    QueryMsg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else {
                Some(
                    Dummy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            }
                .unwrap()(
                0 as libc::c_int,
                b"%s: %s: permission denied (user %s)\0" as *const u8
                    as *const libc::c_char,
                rc_name,
                (*comms.as_mut_ptr().offset(nr as isize)).name,
                ((*if !EffectiveAclUser.is_null() {
                    EffectiveAclUser
                } else {
                    (*display).d_user
                })
                    .u_name)
                    .as_mut_ptr(),
            );
            queryflag = -(1 as libc::c_int);
            return;
        }
    }
    msgok = (!display.is_null() && *rc_name == 0) as libc::c_int;
    let mut current_block_1498: u64;
    match nr {
        145 => {
            if (*args).is_null() {
                InputSelect();
            } else if *(*args.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *(*args.offset(0 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) == 0
            {
                SetForeWindow(0 as *mut win);
                Activate(0 as libc::c_int);
            } else if *(*args.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *(*args.offset(0 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) == 0
            {
                if fore.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"select . needs a window\0" as *const u8 as *const libc::c_char,
                    );
                    queryflag = -(1 as libc::c_int);
                } else {
                    SetForeWindow(fore);
                    Activate(0 as libc::c_int);
                }
            } else if ParseWinNum(act, &mut n) == 0 as libc::c_int {
                SwitchWindow(n);
            } else if queryflag >= 0 as libc::c_int {
                queryflag = -(1 as libc::c_int);
            }
            current_block_1498 = 188265764000799656;
        }
        41 => {
            if ParseOnOff(act, &mut defautonuke) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Default autonuke turned %s\0" as *const u8 as *const libc::c_char,
                    if defautonuke != 0 {
                        b"on\0" as *const u8 as *const libc::c_char
                    } else {
                        b"off\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if !display.is_null() && *rc_name as libc::c_int != 0 {
                (*display).d_auto_nuke = defautonuke;
            }
            current_block_1498 = 188265764000799656;
        }
        12 => {
            if ParseOnOff(act, &mut (*display).d_auto_nuke) == 0 as libc::c_int
                && msgok != 0
            {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Autonuke turned %s\0" as *const u8 as *const libc::c_char,
                    if (*display).d_auto_nuke != 0 {
                        b"on\0" as *const u8 as *const libc::c_char
                    } else {
                        b"off\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        59 => {
            if ParseNum(act, &mut defobuflimit) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Default limit set to %d\0" as *const u8 as *const libc::c_char,
                    defobuflimit,
                );
            }
            if !display.is_null() && *rc_name as libc::c_int != 0 {
                (*display).d_obufmax = defobuflimit;
                (*display).d_obuflenmax = (*display).d_obuflen - (*display).d_obufmax;
            }
            current_block_1498 = 188265764000799656;
        }
        120 => {
            if (*args).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Limit is %d, current buffer size is %d\0" as *const u8
                        as *const libc::c_char,
                    (*display).d_obufmax,
                    (*display).d_obuflen,
                );
            } else if ParseNum(act, &mut (*display).d_obufmax) == 0 as libc::c_int
                && msgok != 0
            {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Limit set to %d\0" as *const u8 as *const libc::c_char,
                    (*display).d_obufmax,
                );
            }
            (*display).d_obuflenmax = (*display).d_obuflen - (*display).d_obufmax;
            current_block_1498 = 188265764000799656;
        }
        71 => {
            WriteFile(user, 0 as *mut libc::c_char, 0 as libc::c_int);
            current_block_1498 = 188265764000799656;
        }
        84 => {
            let mut mode: libc::c_int = 1 as libc::c_int;
            let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
            if !(*args.offset(0 as libc::c_int as isize)).is_null() {
                if strcmp(*args, b"-h\0" as *const u8 as *const libc::c_char) == 0 {
                    mode = 3 as libc::c_int;
                    file = *args.offset(1 as libc::c_int as isize);
                } else if strcmp(*args, b"--\0" as *const u8 as *const libc::c_char) == 0
                    && !(*args.offset(1 as libc::c_int as isize)).is_null()
                {
                    file = *args.offset(1 as libc::c_int as isize);
                } else {
                    file = *args.offset(0 as libc::c_int as isize);
                }
            }
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && file == *args.offset(0 as libc::c_int as isize)
                && !(*args.offset(1 as libc::c_int as isize)).is_null()
            {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: hardcopy: too many arguments\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
            } else {
                WriteFile(user, file, mode);
            }
            current_block_1498 = 188265764000799656;
        }
        53 => {
            ParseOnOff(act, &mut nwin_default.Lflag);
            current_block_1498 = 188265764000799656;
        }
        101 => {
            n = if !((*fore).w_log).is_null() {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            ParseSwitch(act, &mut n);
            LogToggle(n);
            current_block_1498 = 188265764000799656;
        }
        162 => {
            Detach(1 as libc::c_int);
            current_block_1498 = 188265764000799656;
        }
        117 => {
            if MoreWindows() != 0 {
                SwitchWindow(NextWindow());
            }
            current_block_1498 = 188265764000799656;
        }
        130 => {
            if MoreWindows() != 0 {
                SwitchWindow(PreviousWindow());
            }
            current_block_1498 = 188265764000799656;
        }
        96 => {
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            if key >= 0 as libc::c_int {
                Input(
                    (if !((*fore).w_pwin).is_null() {
                        b"Really kill this filter [y/n]\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"Really kill this window [y/n]\0" as *const u8
                            as *const libc::c_char
                    }) as *mut libc::c_char,
                    1 as libc::c_int,
                    2 as libc::c_int,
                    Some(
                        confirm_fn
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    0 as *mut libc::c_char,
                    96 as libc::c_int,
                );
            } else {
                n = (*fore).w_number;
                if !((*fore).w_pwin).is_null() {
                    FreePseudowin(fore);
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Filter removed.\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    name = SaveStr((*fore).w_title);
                    KillWindow(fore);
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Window %d (%s) killed.\0" as *const u8 as *const libc::c_char,
                        n,
                        name,
                    );
                    if !name.is_null() {
                        free(name as *mut libc::c_void);
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        133 => {
            if key >= 0 as libc::c_int {
                Input(
                    b"Really quit and kill all your windows [y/n]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    1 as libc::c_int,
                    2 as libc::c_int,
                    Some(
                        confirm_fn
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    0 as *mut libc::c_char,
                    133 as libc::c_int,
                );
                current_block_1498 = 188265764000799656;
            } else {
                Finit(0 as libc::c_int);
                current_block_1498 = 4564056263373890281;
            }
        }
        67 => {
            current_block_1498 = 4564056263373890281;
        }
        128 => {
            if key >= 0 as libc::c_int {
                static mut buf: [libc::c_char; 2] = [0; 2];
                buf[0 as libc::c_int as usize] = key as libc::c_char;
                Input(
                    buf.as_mut_ptr(),
                    1 as libc::c_int,
                    2 as libc::c_int,
                    Some(
                        pow_detach_fn
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            } else {
                Detach(3 as libc::c_int);
            }
            current_block_1498 = 188265764000799656;
        }
        40 => {
            if (*args).is_null()
                || strcmp(b"off\0" as *const u8 as *const libc::c_char, *args) != 0
            {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Sorry, screen was compiled without -DDEBUG option.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        187 => {
            if !(*args).is_null()
                && strcmp(*args, b"sendcmd\0" as *const u8 as *const libc::c_char) == 0
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    free(zmodem_sendcmd as *mut libc::c_void);
                    zmodem_sendcmd = SaveStr(*args.offset(1 as libc::c_int as isize));
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"zmodem sendcmd: %s\0" as *const u8 as *const libc::c_char,
                        zmodem_sendcmd,
                    );
                }
            } else if !(*args).is_null()
                && strcmp(*args, b"recvcmd\0" as *const u8 as *const libc::c_char) == 0
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    free(zmodem_recvcmd as *mut libc::c_void);
                    zmodem_recvcmd = SaveStr(*args.offset(1 as libc::c_int as isize));
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"zmodem recvcmd: %s\0" as *const u8 as *const libc::c_char,
                        zmodem_recvcmd,
                    );
                }
            } else {
                if !(*args).is_null() {
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        if strcmp(zmodes[i as usize], *args) == 0 {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if i == 4 as libc::c_int
                        && strcmp(*args, b"on\0" as *const u8 as *const libc::c_char)
                            == 0
                    {
                        i = 1 as libc::c_int;
                    }
                    if i == 4 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"usage: zmodem off|auto|catch|pass\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        zmodem_mode = i;
                        current_block_1498 = 2872334340672008580;
                    }
                } else {
                    current_block_1498 = 2872334340672008580;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if msgok != 0 {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"zmodem mode is %s\0" as *const u8 as *const libc::c_char,
                                zmodes[zmodem_mode as usize],
                            );
                        }
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        170 => {
            let mut i_0: libc::c_uint = 0;
            i_0 = 0 as libc::c_int as libc::c_uint;
            while (i_0 as libc::c_ulong)
                < (::std::mem::size_of::<[action; 338]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<action>() as libc::c_ulong)
            {
                ClearAction(&mut *ktab.as_mut_ptr().offset(i_0 as isize));
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            if (*act).quiet == 0 {
                Some(
                    Msg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else if queryflag >= 0 as libc::c_int {
                Some(
                    QueryMsg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else {
                Some(
                    Dummy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            }
                .unwrap()(
                0 as libc::c_int,
                b"Unbound all keys.\0" as *const u8 as *const libc::c_char,
            );
            current_block_1498 = 188265764000799656;
        }
        188 => {
            s = *args;
            if s.is_null() {
                ZombieKey_destroy = 0 as libc::c_int;
            } else if *argl == 0 as libc::c_int || *argl > 2 as libc::c_int {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s:zombie: one or two characters expected.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
            } else {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    if strcmp(
                        *args.offset(1 as libc::c_int as isize),
                        b"onerror\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        ZombieKey_onerror = 1 as libc::c_int;
                        current_block_1498 = 5250576585193495047;
                    } else {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"usage: zombie [keys [onerror]]\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block_1498 = 188265764000799656;
                    }
                } else {
                    ZombieKey_onerror = 0 as libc::c_int;
                    current_block_1498 = 5250576585193495047;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        ZombieKey_destroy = *(*args.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int;
                        ZombieKey_resurrect = if *argl == 2 as libc::c_int {
                            *(*args.offset(0 as libc::c_int as isize))
                                .offset(1 as libc::c_int as isize) as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        178 => {
            s = ((*(*display).d_user).u_name).as_mut_ptr();
            let mut olddisplay: *mut display = display;
            display = 0 as *mut display;
            if (*act).quiet == 0 {
                Some(
                    Msg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else if queryflag >= 0 as libc::c_int {
                Some(
                    QueryMsg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else {
                Some(
                    Dummy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            }
                .unwrap()(
                0 as libc::c_int,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                s,
                *args,
            );
            display = olddisplay;
            current_block_1498 = 188265764000799656;
        }
        9 => {
            if user.is_null() {
                current_block_1498 = 188265764000799656;
            } else {
                s = SaveStr(((*user).u_name).as_mut_ptr());
                EffectiveAclUser = user;
                n = strlen(*args.offset(0 as libc::c_int as isize)) as libc::c_int;
                if n != 0 {
                    n -= 1;
                    n;
                }
                let mut current_block_232: u64;
                match *(*args.offset(0 as libc::c_int as isize)).offset(n as isize)
                    as libc::c_int
                {
                    42 => {
                        let mut nd: *mut display = 0 as *mut display;
                        let mut u: *mut acluser = 0 as *mut acluser;
                        if n == 0 {
                            u = user;
                            current_block_232 = 5506186398343357842;
                        } else {
                            u = users;
                            while !u.is_null() {
                                if strncmp(
                                    *args,
                                    ((*u).u_name).as_mut_ptr(),
                                    n as libc::c_ulong,
                                ) == 0
                                {
                                    break;
                                }
                                u = (*u).u_next;
                            }
                            if u.is_null() {
                                *(*args.offset(0 as libc::c_int as isize))
                                    .offset(n as isize) = '\0' as i32 as libc::c_char;
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"Did not find any user matching '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    *args.offset(0 as libc::c_int as isize),
                                );
                                current_block_232 = 17697686627427283947;
                            } else {
                                current_block_232 = 5506186398343357842;
                            }
                        }
                        match current_block_232 {
                            17697686627427283947 => {}
                            _ => {
                                display = displays;
                                while !display.is_null() {
                                    nd = (*display).d_next;
                                    if !((*display).d_forecv).is_null() {
                                        flayer = (*(*display).d_forecv).c_layer;
                                        fore = (*display).d_fore;
                                        if !((*display).d_user != u) {
                                            DoCommand(
                                                args.offset(1 as libc::c_int as isize),
                                                argl.offset(1 as libc::c_int as isize),
                                            );
                                            if !display.is_null() {
                                                if (*act).quiet == 0 {
                                                    Some(
                                                        Msg
                                                            as unsafe extern "C" fn(
                                                                libc::c_int,
                                                                *const libc::c_char,
                                                                ...
                                                            ) -> (),
                                                    )
                                                } else if queryflag >= 0 as libc::c_int {
                                                    Some(
                                                        QueryMsg
                                                            as unsafe extern "C" fn(
                                                                libc::c_int,
                                                                *const libc::c_char,
                                                                ...
                                                            ) -> (),
                                                    )
                                                } else {
                                                    Some(
                                                        Dummy
                                                            as unsafe extern "C" fn(
                                                                libc::c_int,
                                                                *const libc::c_char,
                                                                ...
                                                            ) -> (),
                                                    )
                                                }
                                                    .unwrap()(
                                                    0 as libc::c_int,
                                                    b"command from %s: %s %s\0" as *const u8
                                                        as *const libc::c_char,
                                                    s,
                                                    *args.offset(1 as libc::c_int as isize),
                                                    if !(*args.offset(2 as libc::c_int as isize)).is_null() {
                                                        *args.offset(2 as libc::c_int as isize)
                                                            as *const libc::c_char
                                                    } else {
                                                        b"\0" as *const u8 as *const libc::c_char
                                                    },
                                                );
                                            }
                                            display = 0 as *mut display;
                                            flayer = 0 as *mut layer;
                                            fore = 0 as *mut win;
                                        }
                                    }
                                    display = nd;
                                }
                                current_block_232 = 17697686627427283947;
                            }
                        }
                    }
                    37 => {
                        let mut nd_0: *mut display = 0 as *mut display;
                        display = displays;
                        while !display.is_null() {
                            nd_0 = (*display).d_next;
                            if !((*display).d_forecv).is_null() {
                                fore = (*display).d_fore;
                                flayer = (*(*display).d_forecv).c_layer;
                                if !(strncmp(
                                    *args.offset(0 as libc::c_int as isize),
                                    ((*display).d_usertty).as_mut_ptr(),
                                    n as libc::c_ulong,
                                ) != 0
                                    && (strncmp(
                                        b"/dev/\0" as *const u8 as *const libc::c_char,
                                        ((*display).d_usertty).as_mut_ptr(),
                                        5 as libc::c_int as libc::c_ulong,
                                    ) != 0
                                        || strncmp(
                                            *args.offset(0 as libc::c_int as isize),
                                            ((*display).d_usertty)
                                                .as_mut_ptr()
                                                .offset(5 as libc::c_int as isize),
                                            n as libc::c_ulong,
                                        ) != 0)
                                    && (strncmp(
                                        b"/dev/tty\0" as *const u8 as *const libc::c_char,
                                        ((*display).d_usertty).as_mut_ptr(),
                                        8 as libc::c_int as libc::c_ulong,
                                    ) != 0
                                        || strncmp(
                                            *args.offset(0 as libc::c_int as isize),
                                            ((*display).d_usertty)
                                                .as_mut_ptr()
                                                .offset(8 as libc::c_int as isize),
                                            n as libc::c_ulong,
                                        ) != 0))
                                {
                                    DoCommand(
                                        args.offset(1 as libc::c_int as isize),
                                        argl.offset(1 as libc::c_int as isize),
                                    );
                                    if !display.is_null() {
                                        if (*act).quiet == 0 {
                                            Some(
                                                Msg
                                                    as unsafe extern "C" fn(
                                                        libc::c_int,
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> (),
                                            )
                                        } else if queryflag >= 0 as libc::c_int {
                                            Some(
                                                QueryMsg
                                                    as unsafe extern "C" fn(
                                                        libc::c_int,
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> (),
                                            )
                                        } else {
                                            Some(
                                                Dummy
                                                    as unsafe extern "C" fn(
                                                        libc::c_int,
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> (),
                                            )
                                        }
                                            .unwrap()(
                                            0 as libc::c_int,
                                            b"command from %s: %s %s\0" as *const u8
                                                as *const libc::c_char,
                                            s,
                                            *args.offset(1 as libc::c_int as isize),
                                            if !(*args.offset(2 as libc::c_int as isize)).is_null() {
                                                *args.offset(2 as libc::c_int as isize)
                                                    as *const libc::c_char
                                            } else {
                                                b"\0" as *const u8 as *const libc::c_char
                                            },
                                        );
                                    }
                                    display = 0 as *mut display;
                                    fore = 0 as *mut win;
                                    flayer = 0 as *mut layer;
                                }
                            }
                            display = nd_0;
                        }
                        current_block_232 = 17697686627427283947;
                    }
                    35 => {
                        n -= 1;
                        n;
                        current_block_232 = 16979802930995685524;
                    }
                    _ => {
                        current_block_232 = 16979802930995685524;
                    }
                }
                match current_block_232 {
                    16979802930995685524 => {
                        let mut nw: *mut win = 0 as *mut win;
                        let mut ch_0: libc::c_int = 0;
                        n += 1;
                        n;
                        ch_0 = *(*args.offset(0 as libc::c_int as isize))
                            .offset(n as isize) as libc::c_int;
                        *(*args.offset(0 as libc::c_int as isize))
                            .offset(n as isize) = '\0' as i32 as libc::c_char;
                        if **args.offset(0 as libc::c_int as isize) == 0
                            || {
                                i = WindowByNumber(*args.offset(0 as libc::c_int as isize));
                                i < 0 as libc::c_int
                            }
                        {
                            *(*args.offset(0 as libc::c_int as isize))
                                .offset(n as isize) = ch_0 as libc::c_char;
                            fore = windows;
                            while !fore.is_null() {
                                nw = (*fore).w_next;
                                if !(strncmp(
                                    *args.offset(0 as libc::c_int as isize),
                                    (*fore).w_title,
                                    n as libc::c_ulong,
                                ) != 0)
                                {
                                    i = 0 as libc::c_int;
                                    if !((*fore).w_layer.l_cvlist).is_null() {
                                        display = (*(*fore).w_layer.l_cvlist).c_display;
                                    }
                                    flayer = if !((*fore).w_savelayer).is_null() {
                                        (*fore).w_savelayer
                                    } else {
                                        &mut (*fore).w_layer
                                    };
                                    DoCommand(
                                        args.offset(1 as libc::c_int as isize),
                                        argl.offset(1 as libc::c_int as isize),
                                    );
                                    if !fore.is_null() && !((*fore).w_layer.l_cvlist).is_null()
                                    {
                                        display = (*(*fore).w_layer.l_cvlist).c_display;
                                        if (*act).quiet == 0 {
                                            Some(
                                                Msg
                                                    as unsafe extern "C" fn(
                                                        libc::c_int,
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> (),
                                            )
                                        } else if queryflag >= 0 as libc::c_int {
                                            Some(
                                                QueryMsg
                                                    as unsafe extern "C" fn(
                                                        libc::c_int,
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> (),
                                            )
                                        } else {
                                            Some(
                                                Dummy
                                                    as unsafe extern "C" fn(
                                                        libc::c_int,
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> (),
                                            )
                                        }
                                            .unwrap()(
                                            0 as libc::c_int,
                                            b"command from %s: %s %s\0" as *const u8
                                                as *const libc::c_char,
                                            s,
                                            *args.offset(1 as libc::c_int as isize),
                                            if !(*args.offset(2 as libc::c_int as isize)).is_null() {
                                                *args.offset(2 as libc::c_int as isize)
                                                    as *const libc::c_char
                                            } else {
                                                b"\0" as *const u8 as *const libc::c_char
                                            },
                                        );
                                    }
                                }
                                fore = nw;
                            }
                            display = 0 as *mut display;
                            fore = 0 as *mut win;
                            if i < 0 as libc::c_int {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"%s: at '%s': no such window.\n\0" as *const u8
                                        as *const libc::c_char,
                                    rc_name,
                                    *args.offset(0 as libc::c_int as isize),
                                );
                            }
                        } else if i < maxwin
                            && {
                                fore = *wtab.offset(i as isize);
                                !fore.is_null()
                            }
                        {
                            *(*args.offset(0 as libc::c_int as isize))
                                .offset(n as isize) = ch_0 as libc::c_char;
                            if !((*fore).w_layer.l_cvlist).is_null() {
                                display = (*(*fore).w_layer.l_cvlist).c_display;
                            }
                            flayer = if !((*fore).w_savelayer).is_null() {
                                (*fore).w_savelayer
                            } else {
                                &mut (*fore).w_layer
                            };
                            DoCommand(
                                args.offset(1 as libc::c_int as isize),
                                argl.offset(1 as libc::c_int as isize),
                            );
                            if !fore.is_null() && !((*fore).w_layer.l_cvlist).is_null() {
                                display = (*(*fore).w_layer.l_cvlist).c_display;
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"command from %s: %s %s\0" as *const u8
                                        as *const libc::c_char,
                                    s,
                                    *args.offset(1 as libc::c_int as isize),
                                    if !(*args.offset(2 as libc::c_int as isize)).is_null() {
                                        *args.offset(2 as libc::c_int as isize)
                                            as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            display = 0 as *mut display;
                            fore = 0 as *mut win;
                        } else {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"%s: at [identifier][%%|*|#] command [args]\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                            );
                        }
                    }
                    _ => {}
                }
                free(s as *mut libc::c_void);
                EffectiveAclUser = 0 as *mut acluser;
                current_block_1498 = 188265764000799656;
            }
        }
        135 => {
            i = if !fore.is_null() {
                (*fore).w_layer.l_encoding
            } else if !display.is_null() {
                (*display).d_encoding
            } else {
                0 as libc::c_int
            };
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && !(*args.offset(1 as libc::c_int as isize)).is_null()
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"-e\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i = FindEncoding(*args.offset(1 as libc::c_int as isize));
                if i == -(1 as libc::c_int) {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: readreg: unknown encoding\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    args = args.offset(2 as libc::c_int as isize);
                    current_block_1498 = 6055351187523413397;
                }
            } else {
                current_block_1498 = 6055351187523413397;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    s = *args;
                    if s.is_null() {
                        Input(
                            b"Copy to register:\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            1 as libc::c_int,
                            2 as libc::c_int,
                            Some(
                                copy_reg_fn
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        libc::c_int,
                                        *mut libc::c_char,
                                    ) -> (),
                            ),
                            0 as *mut libc::c_char,
                            0 as libc::c_int,
                        );
                    } else if *argl != 1 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: copyreg: character, ^x, or (octal) \\032 expected.\0"
                                as *const u8 as *const libc::c_char,
                            rc_name,
                        );
                    } else {
                        ch = *(*args.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize);
                        if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                            if !(*args.offset(2 as libc::c_int as isize)).is_null() {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"%s: readreg: too many arguments\0" as *const u8
                                        as *const libc::c_char,
                                    rc_name,
                                );
                            } else {
                                s = ReadFile(
                                    *args.offset(1 as libc::c_int as isize),
                                    &mut n,
                                );
                                if !s.is_null() {
                                    let mut pp: *mut plop = plop_tab
                                        .as_mut_ptr()
                                        .offset(ch as libc::c_uchar as libc::c_int as isize);
                                    if !((*pp).buf).is_null() {
                                        free((*pp).buf as *mut libc::c_void);
                                    }
                                    (*pp).buf = s;
                                    (*pp).len = n;
                                    (*pp).enc = i;
                                }
                            }
                        } else {
                            copy_reg_fn(
                                &mut ch,
                                0 as libc::c_int,
                                0 as *mut libc::c_char,
                            );
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        137 => {
            i = if !fore.is_null() {
                (*fore).w_layer.l_encoding
            } else if !display.is_null() {
                (*display).d_encoding
            } else {
                0 as libc::c_int
            };
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && !(*args.offset(1 as libc::c_int as isize)).is_null()
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"-e\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i = FindEncoding(*args.offset(1 as libc::c_int as isize));
                if i == -(1 as libc::c_int) {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: register: unknown encoding\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    args = args.offset(2 as libc::c_int as isize);
                    argc -= 2 as libc::c_int;
                    current_block_1498 = 4308757698705929541;
                }
            } else {
                current_block_1498 = 4308757698705929541;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if argc != 2 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: register: illegal number of arguments.\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                        );
                    } else if *argl != 1 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: register: character, ^x, or (octal) \\032 expected.\0"
                                as *const u8 as *const libc::c_char,
                            rc_name,
                        );
                    } else {
                        ch = *(*args.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize);
                        if ch as libc::c_int == '.' as i32 {
                            if !((*user).u_plop.buf).is_null() {
                                UserFreeCopyBuffer(user);
                            }
                            if !(*args.offset(1 as libc::c_int as isize)).is_null()
                                && *(*args.offset(1 as libc::c_int as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int != 0
                            {
                                (*user)
                                    .u_plop
                                    .buf = SaveStrn(
                                    *args.offset(1 as libc::c_int as isize),
                                    *argl.offset(1 as libc::c_int as isize),
                                );
                                (*user)
                                    .u_plop
                                    .len = *argl.offset(1 as libc::c_int as isize);
                                (*user).u_plop.enc = i;
                            }
                        } else {
                            let mut plp: *mut plop = plop_tab
                                .as_mut_ptr()
                                .offset(ch as libc::c_uchar as libc::c_int as isize);
                            if !((*plp).buf).is_null() {
                                free((*plp).buf as *mut libc::c_void);
                            }
                            (*plp)
                                .buf = SaveStrn(
                                *args.offset(1 as libc::c_int as isize),
                                *argl.offset(1 as libc::c_int as isize),
                            );
                            (*plp).len = *argl.offset(1 as libc::c_int as isize);
                            (*plp).enc = i;
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        132 => {
            s = *args;
            if s.is_null() {
                Input(
                    b"Process register:\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    1 as libc::c_int,
                    2 as libc::c_int,
                    Some(
                        process_fn
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            } else if *argl != 1 as libc::c_int {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: process: character, ^x, or (octal) \\032 expected.\0"
                        as *const u8 as *const libc::c_char,
                    rc_name,
                );
            } else {
                ch = *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize);
                process_fn(&mut ch, 0 as libc::c_int, 0 as *mut libc::c_char);
            }
            current_block_1498 = 188265764000799656;
        }
        160 => {
            s = *args;
            if (*args.offset(0 as libc::c_int as isize)).is_null() {
                Input(
                    b"Stuff:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    100 as libc::c_int,
                    0 as libc::c_int,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> ()>,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                        >,
                    >(
                        Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn(
                                    *mut libc::c_char,
                                    libc::c_int,
                                    *mut libc::c_char,
                                ) -> (),
                                unsafe extern "C" fn() -> (),
                            >(StuffFin),
                        ),
                    ),
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            } else {
                n = *argl;
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    if strcmp(s, b"-k\0" as *const u8 as *const libc::c_char) != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: stuff: invalid option %s\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                            s,
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        s = *args.offset(1 as libc::c_int as isize);
                        i = 106 as libc::c_int;
                        while i < 188 as libc::c_int {
                            if strcmp((*term.as_mut_ptr().offset(i as isize)).tcname, s)
                                == 0 as libc::c_int
                            {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        if i == 188 as libc::c_int {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"%s: stuff: unknown key '%s'\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                                s,
                            );
                            current_block_1498 = 188265764000799656;
                        } else if StuffKey(i - 106 as libc::c_int) == 0 as libc::c_int {
                            current_block_1498 = 188265764000799656;
                        } else {
                            s = if !display.is_null() {
                                (*display).d_tcs[i as usize].str_0
                            } else {
                                0 as *mut libc::c_char
                            };
                            if s.is_null() {
                                current_block_1498 = 188265764000799656;
                            } else {
                                n = strlen(s) as libc::c_int;
                                current_block_1498 = 2197125908392311113;
                            }
                        }
                    }
                } else {
                    current_block_1498 = 2197125908392311113;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        while n != 0 {
                            (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                                .unwrap()(&mut s, &mut n);
                        }
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        136 => {
            Activate(-(1 as libc::c_int));
            current_block_1498 = 188265764000799656;
        }
        181 => {
            if !(*args.offset(0 as libc::c_int as isize)).is_null() {
                ShowWindowsX(*args.offset(0 as libc::c_int as isize));
            } else {
                ShowWindows(-(1 as libc::c_int));
            }
            current_block_1498 = 188265764000799656;
        }
        177 => {
            if (*act).quiet == 0 {
                Some(
                    Msg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else if queryflag >= 0 as libc::c_int {
                Some(
                    QueryMsg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            } else {
                Some(
                    Dummy
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> (),
                )
            }
                .unwrap()(
                0 as libc::c_int,
                b"screen %s\0" as *const u8 as *const libc::c_char,
                version.as_mut_ptr(),
            );
            current_block_1498 = 188265764000799656;
        }
        167 => {
            if !(*args).is_null() {
                timestring = SaveStr(*args);
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    MakeWinMsg(timestring, fore, '%' as i32),
                );
            }
            current_block_1498 = 188265764000799656;
        }
        94 => {
            ShowInfo();
            current_block_1498 = 188265764000799656;
        }
        69 => {
            ShowDInfo();
            current_block_1498 = 188265764000799656;
        }
        35 => {
            let mut ktabp: *mut action = ktab.as_mut_ptr();
            if argc == 2 as libc::c_int
                && strcmp(*args, b"-c\0" as *const u8 as *const libc::c_char) == 0
            {
                ktabp = FindKtab(
                    *args.offset(1 as libc::c_int as isize),
                    0 as libc::c_int,
                );
                if ktabp.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Unknown command class '%s'\0" as *const u8
                            as *const libc::c_char,
                        *args.offset(1 as libc::c_int as isize),
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    current_block_1498 = 3842749721932025293;
                }
            } else {
                current_block_1498 = 3842749721932025293;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if (*display).d_ESCseen != ktab.as_mut_ptr()
                        || ktabp != ktab.as_mut_ptr()
                    {
                        if (*display).d_ESCseen != ktabp {
                            (*display).d_ESCseen = ktabp;
                            WindowChanged(fore, 'E' as i32);
                        }
                        current_block_1498 = 188265764000799656;
                    } else {
                        if !((*display).d_ESCseen).is_null() {
                            (*display).d_ESCseen = 0 as *mut action;
                            WindowChanged(fore, 'E' as i32);
                        }
                        current_block_1498 = 9234767613549682605;
                    }
                }
            }
        }
        122 => {
            current_block_1498 = 9234767613549682605;
        }
        110 => {
            if (*user).u_Esc == -(1 as libc::c_int) {
                current_block_1498 = 188265764000799656;
            } else {
                ch = (*user).u_Esc as libc::c_char;
                s = &mut ch;
                n = 1 as libc::c_int;
                (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                    .unwrap()(&mut s, &mut n);
                current_block_1498 = 188265764000799656;
            }
        }
        186 => {
            ch = ('q' as i32 & 0o37 as libc::c_int) as libc::c_char;
            s = &mut ch;
            n = 1 as libc::c_int;
            (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                .unwrap()(&mut s, &mut n);
            current_block_1498 = 188265764000799656;
        }
        185 => {
            ch = ('s' as i32 & 0o37 as libc::c_int) as libc::c_char;
            s = &mut ch;
            n = 1 as libc::c_int;
            (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                .unwrap()(&mut s, &mut n);
            current_block_1498 = 188265764000799656;
        }
        43 | 22 => {
            static mut types: [*mut libc::c_char; 4] = [
                b"TIOCSBRK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"TCSBRK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"tcsendbreak\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ];
            extern "C" {
                static mut breaktype: libc::c_int;
            }
            if !(*args).is_null() {
                if ParseNum(act, &mut n) != 0 {
                    n = 0 as libc::c_int;
                    while n
                        < (::std::mem::size_of::<[*mut libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int
                    {
                        i = 0 as libc::c_int;
                        while i < 4 as libc::c_int {
                            ch = *(*args.offset(0 as libc::c_int as isize))
                                .offset(i as isize);
                            if ch as libc::c_int >= 'a' as i32
                                && ch as libc::c_int <= 'z' as i32
                            {
                                ch = (ch as libc::c_int - ('a' as i32 - 'A' as i32))
                                    as libc::c_char;
                            }
                            if ch as libc::c_int
                                != *(types[n as usize]).offset(i as isize) as libc::c_int
                                && ch as libc::c_int + ('a' as i32 - 'A' as i32)
                                    != *(types[n as usize]).offset(i as isize) as libc::c_int
                            {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        if i == 4 as libc::c_int {
                            break;
                        }
                        n += 1;
                        n;
                    }
                }
                if n < 0 as libc::c_int
                    || n
                        >= (::std::mem::size_of::<[*mut libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s invalid, chose one of %s, %s or %s\0" as *const u8
                            as *const libc::c_char,
                        *args,
                        types[0 as libc::c_int as usize],
                        types[1 as libc::c_int as usize],
                        types[2 as libc::c_int as usize],
                    );
                } else {
                    breaktype = n;
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"breaktype set to (%d) %s\0" as *const u8
                            as *const libc::c_char,
                        n,
                        types[n as usize],
                    );
                }
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"breaktype is (%d) %s\0" as *const u8 as *const libc::c_char,
                    breaktype,
                    types[breaktype as usize],
                );
            }
            current_block_1498 = 188265764000799656;
        }
        127 | 21 => {
            n = 0 as libc::c_int;
            if !(*args).is_null() && ParseNum(act, &mut n) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                SendBreak(fore, n, (nr == 127 as libc::c_int) as libc::c_int);
                current_block_1498 = 188265764000799656;
            }
        }
        100 => {
            Detach(5 as libc::c_int);
            current_block_1498 = 188265764000799656;
        }
        179 | 88 => {
            let mut w: libc::c_int = 0;
            let mut h: libc::c_int = 0;
            let mut what: libc::c_int = 0 as libc::c_int;
            i = 1 as libc::c_int;
            if !(*args).is_null()
                && strcmp(*args, b"-w\0" as *const u8 as *const libc::c_char) == 0
            {
                what = 1 as libc::c_int;
            } else if !(*args).is_null()
                && strcmp(*args, b"-d\0" as *const u8 as *const libc::c_char) == 0
            {
                what = 2 as libc::c_int;
            }
            if what != 0 {
                args = args.offset(1);
                args;
            }
            if what == 0 as libc::c_int && !flayer.is_null() && display.is_null() {
                what = 1 as libc::c_int;
            }
            if what == 1 as libc::c_int {
                if flayer.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: %s: window required\0" as *const u8 as *const libc::c_char,
                        rc_name,
                        (*comms.as_mut_ptr().offset(nr as isize)).name,
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    w = (*flayer).l_width;
                    h = (*flayer).l_height;
                    current_block_1498 = 1169396748315396699;
                }
            } else if display.is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: %s: display required\0" as *const u8 as *const libc::c_char,
                    rc_name,
                    (*comms.as_mut_ptr().offset(nr as isize)).name,
                );
                current_block_1498 = 188265764000799656;
            } else {
                w = (*display).d_width;
                h = (*display).d_height;
                current_block_1498 = 1169396748315396699;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if !(*args).is_null()
                        && *(*args.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                    {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: %s: unknown option %s\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                            (*comms.as_mut_ptr().offset(nr as isize)).name,
                            *args,
                        );
                    } else {
                        if nr == 88 as libc::c_int {
                            if (*args).is_null() {
                                if h == 42 as libc::c_int {
                                    h = 24 as libc::c_int;
                                } else if h == 24 as libc::c_int {
                                    h = 42 as libc::c_int;
                                } else if h
                                    > (42 as libc::c_int + 24 as libc::c_int) / 2 as libc::c_int
                                {
                                    h = 42 as libc::c_int;
                                } else {
                                    h = 24 as libc::c_int;
                                }
                            } else {
                                h = atoi(*args);
                                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                                    w = atoi(*args.offset(1 as libc::c_int as isize));
                                }
                            }
                        } else if (*args).is_null() {
                            if w == Z0width {
                                w = Z1width;
                            } else if w == Z1width {
                                w = Z0width;
                            } else if w > (Z0width + Z1width) / 2 as libc::c_int {
                                w = Z0width;
                            } else {
                                w = Z1width;
                            }
                        } else {
                            w = atoi(*args);
                            if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                                h = atoi(*args.offset(1 as libc::c_int as isize));
                            }
                        }
                        if !(*args).is_null()
                            && !(*args.offset(1 as libc::c_int as isize)).is_null()
                            && !(*args.offset(2 as libc::c_int as isize)).is_null()
                        {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"%s: %s: too many arguments\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                                (*comms.as_mut_ptr().offset(nr as isize)).name,
                            );
                        } else if w <= 0 as libc::c_int {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"Illegal width\0" as *const u8 as *const libc::c_char,
                            );
                        } else if h <= 0 as libc::c_int {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"Illegal height\0" as *const u8 as *const libc::c_char,
                            );
                        } else if what == 1 as libc::c_int {
                            if !((*flayer).l_width == w && (*flayer).l_height == h) {
                                ResizeLayer(flayer, w, h, 0 as *mut display);
                            }
                        } else if !((*display).d_width == w && (*display).d_height == h)
                        {
                            if what == 2 as libc::c_int {
                                ChangeScreenSize(w, h, 1 as libc::c_int);
                            } else if ResizeDisplay(w, h) == 0 as libc::c_int {
                                Activate(
                                    if !((*display).d_fore).is_null() {
                                        (*(*display).d_fore).w_norefresh as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    },
                                );
                                ResizeLayer(
                                    (*(*display).d_forecv).c_layer,
                                    (*(*display).d_forecv).c_xe - (*(*display).d_forecv).c_xs
                                        + 1 as libc::c_int,
                                    (*(*display).d_forecv).c_ye - (*(*display).d_forecv).c_ys
                                        + 1 as libc::c_int,
                                    0 as *mut display,
                                );
                            } else if h == (*display).d_height {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"Your termcap does not specify how to change the terminal's width to %d.\0"
                                        as *const u8 as *const libc::c_char,
                                    w,
                                );
                            } else if w == (*display).d_width {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"Your termcap does not specify how to change the terminal's height to %d.\0"
                                        as *const u8 as *const libc::c_char,
                                    h,
                                );
                            } else {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"Your termcap does not specify how to change the terminal's resolution to %dx%d.\0"
                                        as *const u8 as *const libc::c_char,
                                    w,
                                    h,
                                );
                            }
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        46 => {
            ParseOnOff(act, &mut nwin_default.dynamicaka);
            current_block_1498 = 188265764000799656;
        }
        72 => {
            ParseOnOff(act, &mut (*fore).w_dynamicaka);
            current_block_1498 = 188265764000799656;
        }
        168 => {
            if queryflag >= 0 as libc::c_int {
                if !fore.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*fore).w_title,
                    );
                } else {
                    queryflag = -(1 as libc::c_int);
                }
            } else if (*args).is_null() {
                InputAKA();
            } else {
                ChangeAKA(fore, *args, strlen(*args) as libc::c_int);
            }
            current_block_1498 = 188265764000799656;
        }
        34 => {
            Input(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                768 as libc::c_int,
                4 as libc::c_int,
                Some(
                    Colonfin
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                ),
                0 as *mut libc::c_char,
                0 as libc::c_int,
            );
            if !(*args).is_null() && **args as libc::c_int != 0 {
                s = *args;
                n = strlen(s) as libc::c_int;
                (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                    .unwrap()(&mut s, &mut n);
            }
            current_block_1498 = 188265764000799656;
        }
        97 => {
            if !((*display).d_status_lastmsg).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*display).d_status_lastmsg,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        143 => {
            DoScreen(
                b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                args,
            );
            current_block_1498 = 188265764000799656;
        }
        182 => {
            if ParseSwitch(act, &mut (*fore).w_wrap) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%cwrap\0" as *const u8 as *const libc::c_char,
                    if (*fore).w_wrap != 0 { '+' as i32 } else { '-' as i32 },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        79 => {
            if !(*args).is_null() {
                if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32
                {
                    (*fore)
                        .w_flow = if (*fore).w_flow
                        & (1 as libc::c_int) << 1 as libc::c_int != 0
                    {
                        (1 as libc::c_int) << 2 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int
                    } else {
                        (1 as libc::c_int) << 2 as libc::c_int
                    };
                    current_block_1498 = 10896741756951662056;
                } else if ParseOnOff(act, &mut n) != 0 {
                    current_block_1498 = 188265764000799656;
                } else {
                    (*fore)
                        .w_flow = (*fore).w_flow & (1 as libc::c_int) << 1 as libc::c_int
                        | n;
                    current_block_1498 = 10896741756951662056;
                }
            } else {
                if (*fore).w_flow & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    (*fore)
                        .w_flow = (*fore).w_flow & (1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int;
                } else if (*fore).w_flow & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                    (*fore).w_flow &= !((1 as libc::c_int) << 0 as libc::c_int);
                } else {
                    (*fore)
                        .w_flow = if (*fore).w_flow != 0 {
                        (1 as libc::c_int) << 2 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int
                    } else {
                        (1 as libc::c_int) << 2 as libc::c_int
                    };
                }
                current_block_1498 = 10896741756951662056;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    SetFlow((*fore).w_flow & (1 as libc::c_int) << 0 as libc::c_int);
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%cflow%s\0" as *const u8 as *const libc::c_char,
                            if (*fore).w_flow & (1 as libc::c_int) << 0 as libc::c_int
                                != 0
                            {
                                '+' as i32
                            } else {
                                '-' as i32
                            },
                            if (*fore).w_flow & (1 as libc::c_int) << 2 as libc::c_int
                                != 0
                            {
                                b"(auto)\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        66 => {
            if *(*args.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32
            {
                nwin_default.wlock = 1 as libc::c_int;
            } else if !(ParseOnOff(act, &mut n) != 0) {
                nwin_default
                    .wlock = if n != 0 { 2 as libc::c_int } else { 0 as libc::c_int };
            }
            current_block_1498 = 188265764000799656;
        }
        184 => {
            if !(*args).is_null() {
                if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32
                {
                    (*fore).w_wlock = 1 as libc::c_int;
                    current_block_1498 = 6143943233158298354;
                } else if ParseOnOff(act, &mut n) != 0 {
                    current_block_1498 = 188265764000799656;
                } else {
                    (*fore)
                        .w_wlock = if n != 0 {
                        2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    current_block_1498 = 6143943233158298354;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if AclCheckPermWin((*display).d_user, 1 as libc::c_int, fore)
                            == 0
                        {
                            (*fore).w_wlockuser = (*display).d_user;
                        }
                        current_block_1498 = 11850832584604245957;
                    }
                }
            } else {
                current_block_1498 = 11850832584604245957;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"writelock %s\0" as *const u8 as *const libc::c_char,
                        if (*fore).w_wlock == 1 as libc::c_int {
                            b"auto\0" as *const u8 as *const libc::c_char
                        } else if (*fore).w_wlock == 0 as libc::c_int {
                            b"off\0" as *const u8 as *const libc::c_char
                        } else {
                            b"on\0" as *const u8 as *const libc::c_char
                        },
                    );
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        32 => {
            ResetAnsiState(fore);
            WriteString(
                fore,
                b"\x1B[H\x1B[J\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                6 as libc::c_int,
            );
            current_block_1498 = 188265764000799656;
        }
        141 => {
            ResetAnsiState(fore);
            if !((*fore).w_zdisplay).is_null() {
                zmodem_abort(fore, (*fore).w_zdisplay);
            }
            WriteString(
                fore,
                b"\x1Bc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                2 as libc::c_int,
            );
            current_block_1498 = 188265764000799656;
        }
        111 => {
            n = ((*fore).w_monitor != 0 as libc::c_int) as libc::c_int;
            if !display.is_null() {
                n = (n != 0
                    && *((*fore).w_mon_notify)
                        .offset(((*(*display).d_user).u_id >> 3 as libc::c_int) as isize)
                        as libc::c_int
                        & 0x80 as libc::c_int
                            >> ((*(*display).d_user).u_id & 7 as libc::c_int) != 0)
                    as libc::c_int;
            }
            if ParseSwitch(act, &mut n) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                if n != 0 {
                    if !display.is_null() {
                        let ref mut fresh4 = *((*fore).w_mon_notify)
                            .offset(
                                ((*(*display).d_user).u_id >> 3 as libc::c_int) as isize,
                            );
                        *fresh4 = (*fresh4 as libc::c_int
                            | 0x80 as libc::c_int
                                >> ((*(*display).d_user).u_id & 7 as libc::c_int))
                            as libc::c_uchar;
                    } else {
                        i = 0 as libc::c_int;
                        while i < maxusercount {
                            let ref mut fresh5 = *((*fore).w_mon_notify)
                                .offset((i >> 3 as libc::c_int) as isize);
                            *fresh5 = (*fresh5 as libc::c_int
                                | 0x80 as libc::c_int >> (i & 7 as libc::c_int))
                                as libc::c_uchar;
                            i += 1;
                            i;
                        }
                    }
                    if (*fore).w_monitor == 0 as libc::c_int {
                        (*fore).w_monitor = 1 as libc::c_int;
                    }
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Window %d (%s) is now being monitored for all activity.\0"
                            as *const u8 as *const libc::c_char,
                        (*fore).w_number,
                        (*fore).w_title,
                    );
                } else {
                    if !display.is_null() {
                        let ref mut fresh6 = *((*fore).w_mon_notify)
                            .offset(
                                ((*(*display).d_user).u_id >> 3 as libc::c_int) as isize,
                            );
                        *fresh6 = (*fresh6 as libc::c_int
                            & !(0x80 as libc::c_int
                                >> ((*(*display).d_user).u_id & 7 as libc::c_int)))
                            as libc::c_uchar;
                    } else {
                        i = 0 as libc::c_int;
                        while i < maxusercount {
                            let ref mut fresh7 = *((*fore).w_mon_notify)
                                .offset((i >> 3 as libc::c_int) as isize);
                            *fresh7 = (*fresh7 as libc::c_int
                                & !(0x80 as libc::c_int >> (i & 7 as libc::c_int)))
                                as libc::c_uchar;
                            i += 1;
                            i;
                        }
                    }
                    i = maxusercount - 1 as libc::c_int;
                    while i >= 0 as libc::c_int {
                        if *((*fore).w_mon_notify)
                            .offset((i >> 3 as libc::c_int) as isize) != 0
                        {
                            break;
                        }
                        i -= 1;
                        i;
                    }
                    if i < 0 as libc::c_int {
                        (*fore).w_monitor = 0 as libc::c_int;
                    }
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Window %d (%s) is no longer being monitored for activity.\0"
                            as *const u8 as *const libc::c_char,
                        (*fore).w_number,
                        (*fore).w_title,
                    );
                }
                current_block_1498 = 188265764000799656;
            }
        }
        70 => {
            display_displays();
            current_block_1498 = 188265764000799656;
        }
        180 => {
            if (*args).is_null() {
                display_windows(0 as libc::c_int, 0 as libc::c_int, 0 as *mut win);
            } else if strcmp(*args, b"string\0" as *const u8 as *const libc::c_char) == 0
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    if !wliststr.is_null() {
                        free(wliststr as *mut libc::c_void);
                    }
                    wliststr = SaveStr(*args.offset(1 as libc::c_int as isize));
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"windowlist string is '%s'\0" as *const u8
                            as *const libc::c_char,
                        wliststr,
                    );
                }
            } else if strcmp(*args, b"title\0" as *const u8 as *const libc::c_char) == 0
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    if !wlisttit.is_null() {
                        free(wlisttit as *mut libc::c_void);
                    }
                    wlisttit = SaveStr(*args.offset(1 as libc::c_int as isize));
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"windowlist title is '%s'\0" as *const u8
                            as *const libc::c_char,
                        wlisttit,
                    );
                }
            } else {
                let mut flag: libc::c_int = 0 as libc::c_int;
                let mut blank: libc::c_int = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < argc {
                    if !(*args.offset(i as isize)).is_null() {
                        if strcmp(
                            *args.offset(i as isize),
                            b"-m\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            flag |= 1 as libc::c_int;
                        } else if strcmp(
                            *args.offset(i as isize),
                            b"-b\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            blank = 1 as libc::c_int;
                        } else if strcmp(
                            *args.offset(i as isize),
                            b"-g\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            flag |= 2 as libc::c_int;
                        } else {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"usage: windowlist [-b] [-g] [-m] [string [string] | title [title]]\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            break;
                        }
                    }
                    i += 1;
                    i;
                }
                if i == argc {
                    display_windows(blank, flag, 0 as *mut win);
                }
            }
            current_block_1498 = 188265764000799656;
        }
        89 => {
            if argc == 2 as libc::c_int
                && strcmp(*args, b"-c\0" as *const u8 as *const libc::c_char) == 0
            {
                let mut ktabp_0: *mut action = 0 as *mut action;
                ktabp_0 = FindKtab(
                    *args.offset(1 as libc::c_int as isize),
                    0 as libc::c_int,
                );
                if ktabp_0.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Unknown command class '%s'\0" as *const u8
                            as *const libc::c_char,
                        *args.offset(1 as libc::c_int as isize),
                    );
                } else {
                    display_help(*args.offset(1 as libc::c_int as isize), ktabp_0);
                }
            } else {
                display_help(0 as *mut libc::c_char, ktab.as_mut_ptr());
            }
            current_block_1498 = 188265764000799656;
        }
        99 => {
            display_copyright();
            current_block_1498 = 188265764000799656;
        }
        38 => {
            if (*flayer).l_layfn != &mut WinLf as *mut LayFuncs {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Must be on a window layer\0" as *const u8 as *const libc::c_char,
                );
            } else {
                MarkRoutine();
                WindowChanged(fore, 'P' as i32);
            }
            current_block_1498 = 188265764000799656;
        }
        90 => {
            static mut pasteargs: [*mut libc::c_char; 2] = [
                b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ];
            static mut pasteargl: [libc::c_int; 1] = [1 as libc::c_int];
            if (*flayer).l_layfn != &mut WinLf as *mut LayFuncs {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Must be on a window layer\0" as *const u8 as *const libc::c_char,
                );
                current_block_1498 = 188265764000799656;
            } else if GetHistory() == 0 as libc::c_int {
                current_block_1498 = 188265764000799656;
            } else if ((*user).u_plop.buf).is_null() {
                current_block_1498 = 188265764000799656;
            } else {
                args = pasteargs.as_mut_ptr();
                argl = pasteargl.as_mut_ptr();
                current_block_1498 = 8653303946012650245;
            }
        }
        125 => {
            current_block_1498 = 8653303946012650245;
        }
        183 => {
            if ((*user).u_plop.buf).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"empty buffer\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let mut oldplop: plop = plop {
                    buf: 0 as *const libc::c_char as *mut libc::c_char,
                    len: 0,
                    enc: 0,
                };
                oldplop = (*user).u_plop;
                if !(*args.offset(0 as libc::c_int as isize)).is_null()
                    && !(*args.offset(1 as libc::c_int as isize)).is_null()
                    && strcmp(
                        *args.offset(0 as libc::c_int as isize),
                        b"-e\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    let mut enc_0: libc::c_int = 0;
                    let mut l_0: libc::c_int = 0;
                    let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
                    enc_0 = FindEncoding(*args.offset(1 as libc::c_int as isize));
                    if enc_0 == -(1 as libc::c_int) {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: writebuf: unknown encoding\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        if enc_0 != oldplop.enc {
                            l_0 = RecodeBuf(
                                oldplop.buf as *mut libc::c_uchar,
                                oldplop.len,
                                oldplop.enc,
                                enc_0,
                                0 as *mut libc::c_uchar,
                            );
                            newbuf = malloc((l_0 + 1 as libc::c_int) as libc::c_ulong)
                                as *mut libc::c_char;
                            if newbuf.is_null() {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    strnomem.as_mut_ptr(),
                                );
                                current_block_1498 = 188265764000799656;
                            } else {
                                (*user)
                                    .u_plop
                                    .len = RecodeBuf(
                                    oldplop.buf as *mut libc::c_uchar,
                                    oldplop.len,
                                    oldplop.enc,
                                    enc_0,
                                    newbuf as *mut libc::c_uchar,
                                );
                                (*user).u_plop.buf = newbuf;
                                (*user).u_plop.enc = enc_0;
                                current_block_1498 = 12366097880291256458;
                            }
                        } else {
                            current_block_1498 = 12366097880291256458;
                        }
                        match current_block_1498 {
                            188265764000799656 => {}
                            _ => {
                                args = args.offset(2 as libc::c_int as isize);
                                current_block_1498 = 11919729123286668413;
                            }
                        }
                    }
                } else {
                    current_block_1498 = 11919729123286668413;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if !(*args.offset(0 as libc::c_int as isize)).is_null()
                            && !(*args.offset(1 as libc::c_int as isize)).is_null()
                        {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"%s: writebuf: too many arguments\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                            );
                        } else {
                            WriteFile(
                                user,
                                *args.offset(0 as libc::c_int as isize),
                                2 as libc::c_int,
                            );
                        }
                        if (*user).u_plop.buf != oldplop.buf {
                            free((*user).u_plop.buf as *mut libc::c_void);
                        }
                        (*user).u_plop = oldplop;
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        134 => {
            i = if !fore.is_null() {
                (*fore).w_layer.l_encoding
            } else if !display.is_null() {
                (*display).d_encoding
            } else {
                0 as libc::c_int
            };
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && !(*args.offset(1 as libc::c_int as isize)).is_null()
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"-e\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i = FindEncoding(*args.offset(1 as libc::c_int as isize));
                if i == -(1 as libc::c_int) {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: readbuf: unknown encoding\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    args = args.offset(2 as libc::c_int as isize);
                    current_block_1498 = 6407486463487586332;
                }
            } else {
                current_block_1498 = 6407486463487586332;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if !(*args.offset(0 as libc::c_int as isize)).is_null()
                        && !(*args.offset(1 as libc::c_int as isize)).is_null()
                    {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: readbuf: too many arguments\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                        );
                    } else {
                        s = ReadFile(
                            if !(*args.offset(0 as libc::c_int as isize)).is_null() {
                                *args.offset(0 as libc::c_int as isize)
                            } else {
                                BufferFile
                            },
                            &mut n,
                        );
                        if !s.is_null() {
                            if !((*user).u_plop.buf).is_null() {
                                UserFreeCopyBuffer(user);
                            }
                            (*user).u_plop.len = n;
                            (*user).u_plop.buf = s;
                            (*user).u_plop.enc = i;
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        139 => {
            KillBuffers();
            current_block_1498 = 188265764000799656;
        }
        93 => {
            ParseSwitch(act, &mut search_ic);
            if msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Will %signore case in searches\0" as *const u8
                        as *const libc::c_char,
                    if search_ic != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        75 => {
            if *argl == 0 as libc::c_int {
                SetEscape(user, -(1 as libc::c_int), -(1 as libc::c_int));
                current_block_1498 = 3525159178271462901;
            } else if *argl == 2 as libc::c_int {
                SetEscape(
                    user,
                    *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                    *(*args.offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                );
                current_block_1498 = 3525159178271462901;
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: two characters required after escape.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
                current_block_1498 = 188265764000799656;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if !display.is_null() && user != users {
                        current_block_1498 = 188265764000799656;
                    } else {
                        current_block_1498 = 16816390665721891485;
                    }
                }
            }
        }
        48 => {
            current_block_1498 = 16816390665721891485;
        }
        30 => {
            s = if !(*args).is_null() { *args } else { home };
            if chdir(s) == -(1 as libc::c_int) {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    s,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        149 | 61 => {
            if ParseSaveStr(act, &mut ShellProg) == 0 as libc::c_int {
                let ref mut fresh8 = *ShellArgs
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize);
                *fresh8 = ShellProg;
            }
            current_block_1498 = 188265764000799656;
        }
        86 => {
            if !(*args).is_null() {
                ParseSaveStr(act, &mut hardcopydir);
            }
            if msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"hardcopydir is %s\n\0" as *const u8 as *const libc::c_char,
                    if !hardcopydir.is_null() && *hardcopydir as libc::c_int != 0 {
                        hardcopydir as *const libc::c_char
                    } else {
                        b"<cwd>\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        102 => {
            if !(*args).is_null() {
                let mut buf_0: [libc::c_char; 1024] = [0; 1024];
                if !(*args.offset(1 as libc::c_int as isize)).is_null()
                    && strcmp(*args, b"flush\0" as *const u8 as *const libc::c_char) == 0
                {
                    log_flush = atoi(*args.offset(1 as libc::c_int as isize));
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"log flush timeout set to %ds\n\0" as *const u8
                                as *const libc::c_char,
                            log_flush,
                        );
                    }
                    current_block_1498 = 188265764000799656;
                } else if ParseSaveStr(act, &mut screenlogfile) != 0 {
                    current_block_1498 = 188265764000799656;
                } else {
                    if !fore.is_null() && !((*fore).w_log).is_null() {
                        if DoStartLog(
                            fore,
                            buf_0.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong as libc::c_int,
                        ) != 0
                        {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"Error opening logfile \"%s\"\0" as *const u8
                                    as *const libc::c_char,
                                buf_0.as_mut_ptr(),
                            );
                        }
                    }
                    if msgok == 0 {
                        current_block_1498 = 188265764000799656;
                    } else {
                        current_block_1498 = 13908727566713266484;
                    }
                }
            } else {
                current_block_1498 = 13908727566713266484;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"logfile is '%s'\0" as *const u8 as *const libc::c_char,
                        screenlogfile,
                    );
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        104 => {
            if (*args).is_null()
                || strcmp(*args, b"on\0" as *const u8 as *const libc::c_char) == 0
                || strcmp(*args, b"off\0" as *const u8 as *const libc::c_char) == 0
            {
                if ParseSwitch(act, &mut logtstamp_on) == 0 as libc::c_int && msgok != 0
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"timestamps turned %s\0" as *const u8 as *const libc::c_char,
                        if logtstamp_on != 0 {
                            b"on\0" as *const u8 as *const libc::c_char
                        } else {
                            b"off\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            } else if strcmp(*args, b"string\0" as *const u8 as *const libc::c_char) == 0
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    if !logtstamp_string.is_null() {
                        free(logtstamp_string as *mut libc::c_void);
                    }
                    logtstamp_string = SaveStr(*args.offset(1 as libc::c_int as isize));
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"logfile timestamp is '%s'\0" as *const u8
                            as *const libc::c_char,
                        logtstamp_string,
                    );
                }
            } else if strcmp(*args, b"after\0" as *const u8 as *const libc::c_char) == 0
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    logtstamp_after = atoi(*args.offset(1 as libc::c_int as isize));
                    if msgok == 0 {
                        current_block_1498 = 188265764000799656;
                    } else {
                        current_block_1498 = 14776667709610020417;
                    }
                } else {
                    current_block_1498 = 14776667709610020417;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"timestamp printed after %ds\n\0" as *const u8
                                as *const libc::c_char,
                            logtstamp_after,
                        );
                    }
                }
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"usage: logtstamp [after [n]|string [str]|on|off]\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        150 => {
            ParseSaveStr(act, &mut nwin_default.aka);
            current_block_1498 = 188265764000799656;
        }
        164 | 165 | 166 => {
            if rc_name.is_null() || *rc_name == 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Sorry, too late now. Place that in your .screenrc file.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        163 => {
            s = 0 as *mut libc::c_char;
            if ParseSaveStr(act, &mut s) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                if strlen(s) > 32 as libc::c_int as libc::c_ulong {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: term: argument too long ( < %d)\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        32 as libc::c_int,
                    );
                    free(s as *mut libc::c_void);
                } else {
                    strncpy(
                        screenterm.as_mut_ptr(),
                        s,
                        32 as libc::c_int as libc::c_ulong,
                    );
                    *screenterm
                        .as_mut_ptr()
                        .offset(
                            32 as libc::c_int as isize,
                        ) = '\0' as i32 as libc::c_char;
                    free(s as *mut libc::c_void);
                    MakeTermcap((display == 0 as *mut display) as libc::c_int);
                }
                current_block_1498 = 188265764000799656;
            }
        }
        73 => {
            if msgok == 0
                && (rc_name.is_null()
                    || strcmp(rc_name, b"-X\0" as *const u8 as *const libc::c_char) != 0)
            {
                current_block_1498 = 188265764000799656;
            } else {
                if argc > 1 as libc::c_int
                    && strcmp(*args, b"-n\0" as *const u8 as *const libc::c_char) == 0
                {
                    args = args.offset(1);
                    args;
                    argc -= 1;
                    argc;
                }
                s = *args;
                if argc > 1 as libc::c_int
                    && strcmp(*args, b"-p\0" as *const u8 as *const libc::c_char) == 0
                {
                    args = args.offset(1);
                    args;
                    argc -= 1;
                    argc;
                    s = *args;
                    if !s.is_null() {
                        s = MakeWinMsg(s, fore, '%' as i32);
                    }
                }
                if !s.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        s,
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: 'echo [-n] [-p] \"string\"' expected.\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                    );
                    queryflag = -(1 as libc::c_int);
                }
                current_block_1498 = 188265764000799656;
            }
        }
        15 | 16 => {
            if (*args).is_null() {
                let mut buf_1: [libc::c_char; 256] = [0; 256];
                AddXChars(
                    buf_1.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as libc::c_int,
                    BellString,
                );
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"bell_msg is '%s'\0" as *const u8 as *const libc::c_char,
                    buf_1.as_mut_ptr(),
                );
            } else {
                ParseSaveStr(act, &mut BellString);
            }
            current_block_1498 = 188265764000799656;
        }
        23 => {
            if (*args).is_null() {
                BufferFile = SaveStr(
                    b"/tmp/screen-exchange\0" as *const u8 as *const libc::c_char,
                );
                current_block_1498 = 9007907953910763611;
            } else if ParseSaveStr(act, &mut BufferFile) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                current_block_1498 = 9007907953910763611;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"Bufferfile is now '%s'\0" as *const u8
                                as *const libc::c_char,
                            BufferFile,
                        );
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        5 => {
            ParseSaveStr(act, &mut ActivityString);
            current_block_1498 = 188265764000799656;
        }
        129 => {
            if (*args).is_null() {
                let mut buf_2: [libc::c_char; 256] = [0; 256];
                AddXChars(
                    buf_2.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as libc::c_int,
                    PowDetachString,
                );
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"pow_detach_msg is '%s'\0" as *const u8 as *const libc::c_char,
                    buf_2.as_mut_ptr(),
                );
            } else {
                ParseSaveStr(act, &mut PowDetachString);
            }
            current_block_1498 = 188265764000799656;
        }
        103 => {
            n = ((*fore).w_slot != -(1 as libc::c_int) as slot_t) as libc::c_int;
            if !(*args).is_null()
                && strcmp(*args, b"always\0" as *const u8 as *const libc::c_char) == 0
            {
                (*fore).w_lflag = 3 as libc::c_int;
                if displays.is_null() && n != 0 {
                    SlotToggle(n);
                }
            } else if !(*args).is_null()
                && strcmp(*args, b"attached\0" as *const u8 as *const libc::c_char) == 0
            {
                (*fore).w_lflag = 1 as libc::c_int;
                if displays.is_null() && n != 0 {
                    SlotToggle(0 as libc::c_int);
                }
            } else if ParseSwitch(act, &mut n) == 0 as libc::c_int {
                SlotToggle(n);
            }
            current_block_1498 = 188265764000799656;
        }
        54 => {
            if strcmp(*args, b"always\0" as *const u8 as *const libc::c_char) == 0 {
                nwin_default.lflag |= 2 as libc::c_int;
            } else if strcmp(*args, b"attached\0" as *const u8 as *const libc::c_char)
                == 0
            {
                nwin_default.lflag &= !(2 as libc::c_int);
            } else {
                ParseOnOff(act, &mut nwin_default.lflag);
            }
            current_block_1498 = 188265764000799656;
        }
        49 => {
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && !(*args.offset(1 as libc::c_int as isize)).is_null()
                && *(*args.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32
            {
                iflag = 1 as libc::c_int;
                display = displays;
                while !display.is_null() {
                    if !((*display).d_flow == 0) {
                        (*display)
                            .d_NewMode
                            .tio
                            .c_cc[0 as libc::c_int
                            as usize] = (*display)
                            .d_OldMode
                            .tio
                            .c_cc[0 as libc::c_int as usize];
                        (*display).d_NewMode.tio.c_lflag
                            |= 0o1 as libc::c_int as libc::c_uint;
                        SetTTY((*display).d_userfd, &mut (*display).d_NewMode);
                    }
                    display = (*display).d_next;
                }
            }
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32
            {
                nwin_default.flowflag = (1 as libc::c_int) << 2 as libc::c_int;
            } else {
                ParseOnOff(act, &mut nwin_default.flowflag);
            }
            current_block_1498 = 188265764000799656;
        }
        65 => {
            ParseOnOff(act, &mut nwin_default.wrap);
            current_block_1498 = 188265764000799656;
        }
        44 => {
            ParseOnOff(act, &mut nwin_default.c1);
            current_block_1498 = 188265764000799656;
        }
        42 => {
            ParseOnOff(act, &mut nwin_default.bce);
            current_block_1498 = 188265764000799656;
        }
        50 => {
            ParseOnOff(act, &mut nwin_default.gr);
            current_block_1498 = 188265764000799656;
        }
        56 => {
            if ParseOnOff(act, &mut n) == 0 as libc::c_int {
                nwin_default
                    .monitor = if n == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            }
            current_block_1498 = 188265764000799656;
        }
        57 => {
            if ParseOnOff(act, &mut n) == 0 as libc::c_int {
                defmousetrack = if n == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1000 as libc::c_int
                };
            }
            current_block_1498 = 188265764000799656;
        }
        112 => {
            if (*args.offset(0 as libc::c_int as isize)).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Mouse tracking for this display is turned %s\0" as *const u8
                        as *const libc::c_char,
                    if (*display).d_mousetrack != 0 {
                        b"on\0" as *const u8 as *const libc::c_char
                    } else {
                        b"off\0" as *const u8 as *const libc::c_char
                    },
                );
            } else if ParseOnOff(act, &mut n) == 0 as libc::c_int {
                (*display)
                    .d_mousetrack = if n == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1000 as libc::c_int
                };
                if !((*display).d_fore).is_null() {
                    MouseMode((*(*display).d_fore).w_mouse);
                }
            }
            current_block_1498 = 188265764000799656;
        }
        62 => {
            if ParseOnOff(act, &mut n) == 0 as libc::c_int {
                nwin_default
                    .silence = if n == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            }
            current_block_1498 = 188265764000799656;
        }
        176 => {
            if (*args).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"W%s echo command when creating windows.\0" as *const u8
                        as *const libc::c_char,
                    if VerboseCreate != 0 {
                        b"ill\0" as *const u8 as *const libc::c_char
                    } else {
                        b"on't\0" as *const u8 as *const libc::c_char
                    },
                );
            } else if ParseOnOff(act, &mut n) == 0 as libc::c_int {
                VerboseCreate = n;
            }
            current_block_1498 = 188265764000799656;
        }
        87 => {
            if !display.is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                RemoveStatus();
            }
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"on\0" as *const u8 as *const libc::c_char,
                ) != 0
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"off\0" as *const u8 as *const libc::c_char,
                ) != 0
            {
                let mut olddisplay_0: *mut display = display;
                let mut old_use: libc::c_int = 0;
                let mut new_use: libc::c_int = -(1 as libc::c_int);
                s = *args.offset(0 as libc::c_int as isize);
                if strncmp(
                    s,
                    b"always\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    s = s.offset(6 as libc::c_int as isize);
                }
                if strcmp(s, b"firstline\0" as *const u8 as *const libc::c_char) == 0 {
                    new_use = 4 as libc::c_int;
                    current_block_1498 = 2888458114965489930;
                } else if strcmp(s, b"lastline\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    new_use = 1 as libc::c_int;
                    current_block_1498 = 2888458114965489930;
                } else if strcmp(s, b"ignore\0" as *const u8 as *const libc::c_char) == 0
                {
                    new_use = 0 as libc::c_int;
                    current_block_1498 = 2888458114965489930;
                } else if strcmp(s, b"message\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    new_use = 2 as libc::c_int;
                    current_block_1498 = 2888458114965489930;
                } else if strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"string\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (*args.offset(1 as libc::c_int as isize)).is_null() {
                        let mut buf_3: [libc::c_char; 256] = [0; 256];
                        AddXChars(
                            buf_3.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                as libc::c_int,
                            hstatusstring,
                        );
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"hardstatus string is '%s'\0" as *const u8
                                as *const libc::c_char,
                            buf_3.as_mut_ptr(),
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        current_block_1498 = 2888458114965489930;
                    }
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: usage: hardstatus [always]lastline|ignore|message|string [string]\0"
                            as *const u8 as *const libc::c_char,
                        rc_name,
                    );
                    current_block_1498 = 188265764000799656;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if new_use != -(1 as libc::c_int) {
                            hardstatusemu = new_use
                                | (if s == *args.offset(0 as libc::c_int as isize) {
                                    0 as libc::c_int
                                } else {
                                    (1 as libc::c_int) << 3 as libc::c_int
                                });
                            display = displays;
                            while !display.is_null() {
                                RemoveStatus();
                                new_use = hardstatusemu
                                    & !((1 as libc::c_int) << 3 as libc::c_int);
                                if (*display).d_tcs[75 as libc::c_int as usize].flg != 0
                                    && s == *args.offset(0 as libc::c_int as isize)
                                {
                                    new_use = 3 as libc::c_int;
                                }
                                ShowHStatus(0 as *mut libc::c_char);
                                old_use = (*display).d_has_hstatus;
                                (*display).d_has_hstatus = new_use;
                                if new_use == 1 as libc::c_int
                                    && old_use != 1 as libc::c_int
                                    || new_use != 1 as libc::c_int
                                        && old_use == 1 as libc::c_int
                                {
                                    ChangeScreenSize(
                                        (*display).d_width,
                                        (*display).d_height,
                                        1 as libc::c_int,
                                    );
                                }
                                if new_use == 4 as libc::c_int
                                    && old_use != 4 as libc::c_int
                                    || new_use != 4 as libc::c_int
                                        && old_use == 4 as libc::c_int
                                {
                                    ChangeScreenSize(
                                        (*display).d_width,
                                        (*display).d_height,
                                        1 as libc::c_int,
                                    );
                                }
                                RefreshHStatus();
                                display = (*display).d_next;
                            }
                        }
                        if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                            if !hstatusstring.is_null() {
                                free(hstatusstring as *mut libc::c_void);
                            }
                            hstatusstring = SaveStr(
                                *args.offset(1 as libc::c_int as isize),
                            );
                            display = displays;
                            while !display.is_null() {
                                RefreshHStatus();
                                display = (*display).d_next;
                            }
                        }
                        display = olddisplay_0;
                    }
                }
            } else {
                ParseSwitch(act, &mut use_hardstatus);
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"messages displayed on %s\0" as *const u8
                            as *const libc::c_char,
                        if use_hardstatus != 0 {
                            b"hardstatus line\0" as *const u8 as *const libc::c_char
                        } else {
                            b"window\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        27 => {
            if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"always\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"splitonly\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                let mut olddisplay_1: *mut display = display;
                captionalways = (*(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32)
                    as libc::c_int;
                display = displays;
                while !display.is_null() {
                    ChangeScreenSize(
                        (*display).d_width,
                        (*display).d_height,
                        1 as libc::c_int,
                    );
                    display = (*display).d_next;
                }
                display = olddisplay_1;
                current_block_1498 = 17655962085468275945;
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"string\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    let mut buf_4: [libc::c_char; 256] = [0; 256];
                    AddXChars(
                        buf_4.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                            as libc::c_int,
                        captionstring,
                    );
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"caption string is '%s'\0" as *const u8 as *const libc::c_char,
                        buf_4.as_mut_ptr(),
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    current_block_1498 = 17655962085468275945;
                }
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: usage: caption always|splitonly|string <string>\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
                current_block_1498 = 188265764000799656;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if (*args.offset(1 as libc::c_int as isize)).is_null() {
                        current_block_1498 = 188265764000799656;
                    } else {
                        if !captionstring.is_null() {
                            free(captionstring as *mut libc::c_void);
                        }
                        captionstring = SaveStr(*args.offset(1 as libc::c_int as isize));
                        RedisplayDisplays(0 as libc::c_int);
                        current_block_1498 = 188265764000799656;
                    }
                }
            }
        }
        37 => {
            n = (console_window != 0 as *mut win) as libc::c_int;
            if ParseSwitch(act, &mut n) != 0 {
                current_block_1498 = 188265764000799656;
            } else if TtyGrabConsole((*fore).w_ptyfd, n, rc_name) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                if n == 0 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: releasing console %s\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        HostName.as_mut_ptr(),
                    );
                } else if !console_window.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: stealing console %s from window %d (%s)\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        HostName.as_mut_ptr(),
                        (*console_window).w_number,
                        (*console_window).w_title,
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: grabbing console %s\0" as *const u8 as *const libc::c_char,
                        rc_name,
                        HostName.as_mut_ptr(),
                    );
                }
                console_window = if n != 0 { fore } else { 0 as *mut win };
                current_block_1498 = 188265764000799656;
            }
        }
        7 => {
            if ParseOnOff(act, &mut all_norefresh) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                if all_norefresh == 0 && !fore.is_null() {
                    Activate(-(1 as libc::c_int));
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        if all_norefresh != 0 {
                            b"No refresh on window change!\n\0" as *const u8
                                as *const libc::c_char
                        } else {
                            b"Window specific refresh\n\0" as *const u8
                                as *const libc::c_char
                        },
                    );
                }
                current_block_1498 = 188265764000799656;
            }
        }
        123 => {
            ParseSwitch(act, &mut n);
            (*fore).w_norefresh = n as libc::c_char;
            current_block_1498 = 188265764000799656;
        }
        173 => {
            if ParseSwitch(act, &mut visual_bell) != 0 || msgok == 0 {
                current_block_1498 = 188265764000799656;
            } else {
                if visual_bell == 0 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"switched to audible bell.\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"switched to visual bell.\0" as *const u8 as *const libc::c_char,
                    );
                }
                current_block_1498 = 188265764000799656;
            }
        }
        175 => {
            if ParseNum1000(act, &mut VBellWait) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"vbellwait set to %.10g seconds\0" as *const u8
                        as *const libc::c_char,
                    VBellWait as libc::c_double / 1000.0f64,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        114 => {
            if ParseNum1000(act, &mut MsgWait) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"msgwait set to %.10g seconds\0" as *const u8
                        as *const libc::c_char,
                    MsgWait as libc::c_double / 1000.0f64,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        113 => {
            if ParseNum1000(act, &mut MsgMinWait) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"msgminwait set to %.10g seconds\0" as *const u8
                        as *const libc::c_char,
                    MsgMinWait as libc::c_double / 1000.0f64,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        152 => {
            if ParseNum(act, &mut SilenceWait) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                if SilenceWait < 1 as libc::c_int {
                    SilenceWait = 1 as libc::c_int;
                }
                p = windows;
                while !p.is_null() {
                    (*p).w_silencewait = SilenceWait;
                    p = (*p).w_next;
                }
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"silencewait set to %d seconds\0" as *const u8
                            as *const libc::c_char,
                        SilenceWait,
                    );
                }
                current_block_1498 = 188265764000799656;
            }
        }
        25 => {
            if (*fore).w_number < NextWindow() {
                WindowChangeNumber((*fore).w_number, NextWindow());
            }
            current_block_1498 = 188265764000799656;
        }
        24 => {
            if (*fore).w_number > PreviousWindow() {
                WindowChangeNumber((*fore).w_number, PreviousWindow());
            }
            current_block_1498 = 188265764000799656;
        }
        33 => {
            CollapseWindowlist();
            current_block_1498 = 188265764000799656;
        }
        119 => {
            if (*args).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    if queryflag >= 0 as libc::c_int {
                        b"%d (%s)\0" as *const u8 as *const libc::c_char
                    } else {
                        b"This is window %d (%s).\0" as *const u8 as *const libc::c_char
                    },
                    (*fore).w_number,
                    (*fore).w_title,
                );
            } else {
                let mut old: libc::c_int = (*fore).w_number;
                let mut rel: libc::c_int = 0 as libc::c_int;
                let mut parse: libc::c_int = 0;
                if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
                {
                    rel = 1 as libc::c_int;
                } else if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                {
                    rel = -(1 as libc::c_int);
                }
                if rel != 0 {
                    let ref mut fresh9 = *((*act).args)
                        .offset(0 as libc::c_int as isize);
                    *fresh9 = (*fresh9).offset(1);
                    *fresh9;
                }
                parse = ParseNum(act, &mut n);
                if rel != 0 {
                    let ref mut fresh10 = *((*act).args)
                        .offset(0 as libc::c_int as isize);
                    *fresh10 = (*fresh10).offset(-1);
                    *fresh10;
                }
                if !(parse != 0) {
                    if rel > 0 as libc::c_int {
                        n += old;
                    } else if rel < 0 as libc::c_int {
                        n = old - n;
                    }
                    if WindowChangeNumber(old, n) == 0 {
                        queryflag = -(1 as libc::c_int);
                        return;
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        189 => {
            if argc != 1 as libc::c_int {
                Msg(
                    0 as libc::c_int,
                    b"Setting zombie polling needs a timeout arg\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                nwin_default
                    .poll_zombie_timeout = atoi(*args.offset(0 as libc::c_int as isize));
                if !fore.is_null() {
                    (*fore).w_poll_zombie_timeout = nwin_default.poll_zombie_timeout;
                }
            }
            current_block_1498 = 188265764000799656;
        }
        156 => {
            if !fore.is_null() {
                Msg(
                    0 as libc::c_int,
                    b"Sorting inside a window is not allowed. Push CTRL-a \" and try again\n\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                i = 0 as libc::c_int;
                while i < maxwin {
                    if !(*wtab.offset(i as isize)).is_null() {
                        n = i;
                        nr = i + 1 as libc::c_int;
                        while nr < maxwin {
                            if !(*wtab.offset(nr as isize)).is_null() {
                                if strcmp(
                                    (**wtab.offset(nr as isize)).w_title,
                                    (**wtab.offset(n as isize)).w_title,
                                ) < 0 as libc::c_int
                                {
                                    n = nr;
                                }
                            }
                            nr += 1;
                            nr;
                        }
                        if n != i {
                            p = *wtab.offset(n as isize);
                            let ref mut fresh11 = *wtab.offset(n as isize);
                            *fresh11 = *wtab.offset(i as isize);
                            let ref mut fresh12 = *wtab.offset(i as isize);
                            *fresh12 = p;
                            (**wtab.offset(n as isize)).w_number = n;
                            (**wtab.offset(i as isize)).w_number = i;
                            AclWinSwap(i, n);
                        }
                    }
                    i += 1;
                    i;
                }
                WindowChanged(0 as *mut win, 0 as libc::c_int);
            }
            current_block_1498 = 188265764000799656;
        }
        151 => {
            n = ((*fore).w_silence != 0 as libc::c_int) as libc::c_int;
            i = (*fore).w_silencewait;
            if !(*args.offset(0 as libc::c_int as isize)).is_null()
                && (*(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                    || *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                        && *(*args.offset(0 as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            <= '9' as i32)
            {
                if ParseNum(act, &mut i) != 0 {
                    current_block_1498 = 188265764000799656;
                } else {
                    n = (i > 0 as libc::c_int) as libc::c_int;
                    current_block_1498 = 11551883506742131556;
                }
            } else if ParseSwitch(act, &mut n) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                current_block_1498 = 11551883506742131556;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if n != 0 {
                        if !display.is_null() {
                            let ref mut fresh13 = *((*fore).w_lio_notify)
                                .offset(
                                    ((*(*display).d_user).u_id >> 3 as libc::c_int) as isize,
                                );
                            *fresh13 = (*fresh13 as libc::c_int
                                | 0x80 as libc::c_int
                                    >> ((*(*display).d_user).u_id & 7 as libc::c_int))
                                as libc::c_uchar;
                        } else {
                            n = 0 as libc::c_int;
                            while n < maxusercount {
                                let ref mut fresh14 = *((*fore).w_lio_notify)
                                    .offset((n >> 3 as libc::c_int) as isize);
                                *fresh14 = (*fresh14 as libc::c_int
                                    | 0x80 as libc::c_int >> (n & 7 as libc::c_int))
                                    as libc::c_uchar;
                                n += 1;
                                n;
                            }
                        }
                        (*fore).w_silencewait = i;
                        (*fore).w_silence = 1 as libc::c_int;
                        SetTimeout(
                            &mut (*fore).w_silenceev,
                            (*fore).w_silencewait * 1000 as libc::c_int,
                        );
                        evenq(&mut (*fore).w_silenceev);
                        if !(msgok == 0) {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"The window is now being monitored for %d sec. silence.\0"
                                    as *const u8 as *const libc::c_char,
                                (*fore).w_silencewait,
                            );
                        }
                    } else {
                        if !display.is_null() {
                            let ref mut fresh15 = *((*fore).w_lio_notify)
                                .offset(
                                    ((*(*display).d_user).u_id >> 3 as libc::c_int) as isize,
                                );
                            *fresh15 = (*fresh15 as libc::c_int
                                & !(0x80 as libc::c_int
                                    >> ((*(*display).d_user).u_id & 7 as libc::c_int)))
                                as libc::c_uchar;
                        } else {
                            n = 0 as libc::c_int;
                            while n < maxusercount {
                                let ref mut fresh16 = *((*fore).w_lio_notify)
                                    .offset((n >> 3 as libc::c_int) as isize);
                                *fresh16 = (*fresh16 as libc::c_int
                                    & !(0x80 as libc::c_int >> (n & 7 as libc::c_int)))
                                    as libc::c_uchar;
                                n += 1;
                                n;
                            }
                        }
                        i = maxusercount - 1 as libc::c_int;
                        while i >= 0 as libc::c_int {
                            if *((*fore).w_lio_notify)
                                .offset((i >> 3 as libc::c_int) as isize) != 0
                            {
                                break;
                            }
                            i -= 1;
                            i;
                        }
                        if i < 0 as libc::c_int {
                            (*fore).w_silence = 0 as libc::c_int;
                            evdeq(&mut (*fore).w_silenceev);
                        }
                        if !(msgok == 0) {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"The window is no longer being monitored for silence.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        60 => {
            ParseNum(act, &mut nwin_default.histheight);
            current_block_1498 = 188265764000799656;
        }
        144 => {
            if (*flayer).l_layfn == &mut MarkLf as *mut LayFuncs {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Cannot resize scrollback buffer in copy/scrollback mode.\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                ParseNum(act, &mut n);
                ChangeWindowSize(
                    fore,
                    (*fore).w_layer.l_width,
                    (*fore).w_layer.l_height,
                    n,
                );
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"scrollback set to %d\0" as *const u8 as *const libc::c_char,
                        (*fore).w_histheight,
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        146 => {
            if (*args).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"This session is named '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    SockName,
                );
            } else {
                let mut buf_5: [libc::c_char; 4096] = [0; 4096];
                s = 0 as *mut libc::c_char;
                if !(ParseSaveStr(act, &mut s) != 0) {
                    if *s == 0
                        || (strlen(s))
                            .wrapping_add(
                                SockName.offset_from(SockPath.as_mut_ptr()) as libc::c_long
                                    as libc::c_ulong,
                            )
                            > (4096 as libc::c_int - 13 as libc::c_int) as libc::c_ulong
                        || !(index(s, '/' as i32)).is_null()
                    {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: bad session name '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                            s,
                        );
                        free(s as *mut libc::c_void);
                    } else {
                        strncpy(
                            buf_5.as_mut_ptr(),
                            SockPath.as_mut_ptr(),
                            SockName.offset_from(SockPath.as_mut_ptr()) as libc::c_long
                                as libc::c_ulong,
                        );
                        sprintf(
                            buf_5
                                .as_mut_ptr()
                                .offset(
                                    SockName.offset_from(SockPath.as_mut_ptr()) as libc::c_long
                                        as isize,
                                ),
                            b"%d.%s\0" as *const u8 as *const libc::c_char,
                            getpid(),
                            s,
                        );
                        free(s as *mut libc::c_void);
                        if access(buf_5.as_mut_ptr(), 0 as libc::c_int)
                            == 0 as libc::c_int
                            || *__errno_location() != 2 as libc::c_int
                        {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"%s: inappropriate path: '%s'.\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                                buf_5.as_mut_ptr(),
                            );
                        } else if rename(SockPath.as_mut_ptr(), buf_5.as_mut_ptr()) != 0
                        {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                *__errno_location(),
                                b"%s: failed to rename(%s, %s)\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                                SockPath.as_mut_ptr(),
                                buf_5.as_mut_ptr(),
                            );
                        } else {
                            strcpy(SockPath.as_mut_ptr(), buf_5.as_mut_ptr());
                            MakeNewEnv();
                            WindowChanged(0 as *mut win, 'S' as i32);
                        }
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        147 => {
            if (*args.offset(0 as libc::c_int as isize)).is_null()
                || (*args.offset(1 as libc::c_int as isize)).is_null()
            {
                InputSetenv(*args.offset(0 as libc::c_int as isize));
            } else {
                xsetenv(
                    *args.offset(0 as libc::c_int as isize),
                    *args.offset(1 as libc::c_int as isize),
                );
                MakeNewEnv();
            }
            current_block_1498 = 188265764000799656;
        }
        171 => {
            unsetenv(*args);
            MakeNewEnv();
            current_block_1498 = 188265764000799656;
        }
        63 => {
            ParseNum(act, &mut nwin_default.slow);
            current_block_1498 = 188265764000799656;
        }
        154 => {
            if (*args).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    if (*fore).w_slowpaste != 0 {
                        b"Slowpaste in window %d is %d milliseconds.\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"Slowpaste in window %d is unset.\0" as *const u8
                            as *const libc::c_char
                    },
                    (*fore).w_number,
                    (*fore).w_slowpaste,
                );
            } else if ParseNum(act, &mut (*fore).w_slowpaste) == 0 as libc::c_int
                && msgok != 0
            {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    if (*fore).w_slowpaste != 0 {
                        b"Slowpaste in window %d set to %d milliseconds.\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"Slowpaste in window %d now unset.\0" as *const u8
                            as *const libc::c_char
                    },
                    (*fore).w_number,
                    (*fore).w_slowpaste,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        108 => {
            if CompileKeys(*args, *argl, mark_key_tab.as_mut_ptr()) != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: markkeys: syntax error.\0" as *const u8 as *const libc::c_char,
                    rc_name,
                );
                current_block_1498 = 188265764000799656;
            } else {
                current_block_1498 = 188265764000799656;
            }
        }
        126 => {
            if ParseSwitch(act, &mut pastefont) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Will %spaste font settings\0" as *const u8 as *const libc::c_char,
                    if pastefont != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        39 => {
            ParseSwitch(act, &mut join_with_cr);
            current_block_1498 = 188265764000799656;
        }
        36 => {
            if ParseSwitch(act, &mut compacthist) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%scompacting history lines\0" as *const u8 as *const libc::c_char,
                    if compacthist != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        116 => {
            ParseOnOff(act, &mut nethackflag);
            current_block_1498 = 188265764000799656;
        }
        85 => {
            ParseOnOff(act, &mut hardcopy_append);
            current_block_1498 = 188265764000799656;
        }
        174 => {
            if (*args).is_null() {
                let mut buf_6: [libc::c_char; 256] = [0; 256];
                AddXChars(
                    buf_6.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as libc::c_int,
                    VisualBellString,
                );
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"vbell_msg is '%s'\0" as *const u8 as *const libc::c_char,
                    buf_6.as_mut_ptr(),
                );
            } else {
                ParseSaveStr(act, &mut VisualBellString);
            }
            current_block_1498 = 188265764000799656;
        }
        55 => {
            if ParseBase(
                act,
                *args,
                &mut n,
                8 as libc::c_int,
                b"octal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                current_block_1498 = 188265764000799656;
            } else {
                if n < 0 as libc::c_int || n > 0o777 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: mode: Invalid tty mode %o\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        n,
                    );
                } else {
                    TtyMode = n;
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"Ttymode set to %03o\0" as *const u8 as *const libc::c_char,
                            TtyMode,
                        );
                    }
                }
                current_block_1498 = 188265764000799656;
            }
        }
        11 => {
            ParseOnOff(act, &mut auto_detach);
            current_block_1498 = 188265764000799656;
        }
        159 => {
            ParseOnOff(act, &mut default_startup);
            current_block_1498 = 188265764000799656;
        }
        124 => {
            if !(*args).is_null() {
                n = if *(*user).u_password as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                if (*user).u_password != NullStr.as_mut_ptr() {
                    free((*user).u_password as *mut libc::c_void);
                }
                (*user).u_password = SaveStr(*args);
                if strcmp(
                    (*user).u_password,
                    b"none\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if n != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"Password checking disabled\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    free((*user).u_password as *mut libc::c_void);
                    (*user).u_password = NullStr.as_mut_ptr();
                }
            } else if fore.is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: password: window required\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
            } else {
                Input(
                    b"New screen password:\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    100 as libc::c_int,
                    1 as libc::c_int,
                    Some(
                        pass1
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    if !display.is_null() {
                        (*display).d_user as *mut libc::c_char
                    } else {
                        users as *mut libc::c_char
                    },
                    0 as libc::c_int,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        17 => {
            let mut ktabp_1: *mut action = ktab.as_mut_ptr();
            let mut kflag: libc::c_int = 0 as libc::c_int;
            loop {
                if argc > 2 as libc::c_int
                    && strcmp(*args, b"-c\0" as *const u8 as *const libc::c_char) == 0
                {
                    ktabp_1 = FindKtab(
                        *args.offset(1 as libc::c_int as isize),
                        1 as libc::c_int,
                    );
                    if ktabp_1.is_null() {
                        break;
                    }
                    args = args.offset(2 as libc::c_int as isize);
                    argl = argl.offset(2 as libc::c_int as isize);
                    argc -= 2 as libc::c_int;
                } else {
                    if !(argc > 1 as libc::c_int
                        && strcmp(*args, b"-k\0" as *const u8 as *const libc::c_char)
                            == 0)
                    {
                        break;
                    }
                    kflag = 1 as libc::c_int;
                    args = args.offset(1);
                    args;
                    argl = argl.offset(1);
                    argl;
                    argc -= 1;
                    argc;
                }
            }
            if kflag != 0 {
                n = 0 as libc::c_int;
                while n < 188 as libc::c_int - 106 as libc::c_int {
                    if strcmp(
                        (*term.as_mut_ptr().offset((n + 106 as libc::c_int) as isize))
                            .tcname,
                        *args,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                    n += 1;
                    n;
                }
                if n == 188 as libc::c_int - 106 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: bind: unknown key '%s'\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        *args,
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    n += 256 as libc::c_int;
                    current_block_1498 = 8808883811079835581;
                }
            } else if *argl != 1 as libc::c_int {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: bind: character, ^x, or (octal) \\032 expected.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
                current_block_1498 = 188265764000799656;
            } else {
                n = *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int;
                current_block_1498 = 8808883811079835581;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                        i = FindCommnr(*args.offset(1 as libc::c_int as isize));
                        if i == -(1 as libc::c_int) {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"%s: bind: unknown command '%s'\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                                *args.offset(1 as libc::c_int as isize),
                            );
                        } else if !(CheckArgNum(
                            i,
                            args.offset(2 as libc::c_int as isize),
                        ) < 0 as libc::c_int)
                        {
                            ClearAction(&mut *ktabp_1.offset(n as isize));
                            SaveAction(
                                ktabp_1.offset(n as isize),
                                i,
                                args.offset(2 as libc::c_int as isize),
                                argl.offset(2 as libc::c_int as isize),
                            );
                        }
                    } else {
                        ClearAction(&mut *ktabp_1.offset(n as isize));
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        18 => {
            let mut newact: *mut action = 0 as *mut action;
            let mut newnr: libc::c_int = 0;
            let mut fl: libc::c_int = 0 as libc::c_int;
            let mut kf: libc::c_int = 0 as libc::c_int;
            let mut af: libc::c_int = 0 as libc::c_int;
            let mut df: libc::c_int = 0 as libc::c_int;
            let mut mf: libc::c_int = 0 as libc::c_int;
            let mut odisp: *mut display = display;
            let mut used: libc::c_int = 0 as libc::c_int;
            let mut kme: *mut kmap_ext = 0 as *mut kmap_ext;
            while !(*args).is_null() && **args as libc::c_int == '-' as i32 {
                if strcmp(*args, b"-t\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    fl = 0x4000 as libc::c_int;
                } else if strcmp(*args, b"-k\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    kf = 1 as libc::c_int;
                } else if strcmp(*args, b"-a\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    af = 1 as libc::c_int;
                } else if strcmp(*args, b"-d\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    df = 1 as libc::c_int;
                } else if strcmp(*args, b"-m\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    mf = 1 as libc::c_int;
                } else if strcmp(*args, b"--\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    args = args.offset(1);
                    args;
                    argl = argl.offset(1);
                    argl;
                    break;
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: bindkey: invalid option %s\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        *args,
                    );
                    return;
                }
                args = args.offset(1);
                args;
                argl = argl.offset(1);
                argl;
            }
            if df != 0 && mf != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: bindkey: -d does not work with -m\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
            } else if (*args).is_null() {
                if mf != 0 {
                    display_bindkey(
                        b"Edit mode\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        mmtab.as_mut_ptr(),
                    );
                } else if df != 0 {
                    display_bindkey(
                        b"Default\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        dmtab.as_mut_ptr(),
                    );
                } else {
                    display_bindkey(
                        b"User\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        umtab.as_mut_ptr(),
                    );
                }
            } else {
                if kf == 0 as libc::c_int {
                    if af != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: bindkey: -a only works with -k\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                        );
                        current_block_1498 = 188265764000799656;
                    } else if *argl == 0 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: bindkey: empty string makes no sense\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        i = 0 as libc::c_int;
                        kme = kmap_exts;
                        while i < kmap_extn {
                            if ((*kme).str_0).is_null() {
                                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                                    break;
                                }
                            } else if *argl == (*kme).fl & !(0x4000 as libc::c_int)
                                && bcmp(
                                    (*kme).str_0 as *const libc::c_void,
                                    *args as *const libc::c_void,
                                    *argl as libc::c_ulong,
                                ) == 0 as libc::c_int
                            {
                                break;
                            }
                            i += 1;
                            i;
                            kme = kme.offset(1);
                            kme;
                        }
                        if i == kmap_extn {
                            if (*args.offset(1 as libc::c_int as isize)).is_null() {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"%s: bindkey: keybinding not found\0" as *const u8
                                        as *const libc::c_char,
                                    rc_name,
                                );
                                current_block_1498 = 188265764000799656;
                            } else {
                                kmap_extn += 8 as libc::c_int;
                                kmap_exts = xrealloc(
                                    kmap_exts as *mut libc::c_char,
                                    (kmap_extn as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<kmap_ext>() as libc::c_ulong,
                                        ) as libc::c_int,
                                ) as *mut kmap_ext;
                                kme = kmap_exts.offset(i as isize);
                                bzero(
                                    kme as *mut libc::c_char as *mut libc::c_void,
                                    (8 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<kmap_ext>() as libc::c_ulong,
                                        ),
                                );
                                while i < kmap_extn {
                                    (*kme).str_0 = 0 as *mut libc::c_char;
                                    (*kme).um.nr = -(1 as libc::c_int);
                                    (*kme).mm.nr = (*kme).um.nr;
                                    (*kme).dm.nr = (*kme).mm.nr;
                                    (*kme).um.args = noargs.as_mut_ptr();
                                    (*kme).mm.args = (*kme).um.args;
                                    (*kme).dm.args = (*kme).mm.args;
                                    (*kme).um.argl = 0 as *mut libc::c_int;
                                    (*kme).mm.argl = (*kme).um.argl;
                                    (*kme).dm.argl = (*kme).mm.argl;
                                    i += 1;
                                    i;
                                    kme = kme.offset(1);
                                    kme;
                                }
                                i -= 8 as libc::c_int;
                                kme = kme.offset(-(8 as libc::c_int as isize));
                                current_block_1498 = 13465939446890021715;
                            }
                        } else {
                            current_block_1498 = 13465939446890021715;
                        }
                        match current_block_1498 {
                            188265764000799656 => {}
                            _ => {
                                if df == 0 as libc::c_int
                                    && (*kme).dm.nr != -(1 as libc::c_int)
                                {
                                    used = 1 as libc::c_int;
                                }
                                if mf == 0 as libc::c_int
                                    && (*kme).mm.nr != -(1 as libc::c_int)
                                {
                                    used = 1 as libc::c_int;
                                }
                                if (df != 0 || mf != 0)
                                    && (*kme).um.nr != -(1 as libc::c_int)
                                {
                                    used = 1 as libc::c_int;
                                }
                                i
                                    += 188 as libc::c_int - 106 as libc::c_int
                                        + (188 as libc::c_int - 166 as libc::c_int);
                                newact = if df != 0 {
                                    &mut (*kme).dm
                                } else if mf != 0 {
                                    &mut (*kme).mm
                                } else {
                                    &mut (*kme).um
                                };
                                current_block_1498 = 11751514538013075448;
                            }
                        }
                    }
                } else {
                    i = 106 as libc::c_int;
                    while i < 188 as libc::c_int {
                        if strcmp((*term.as_mut_ptr().offset(i as isize)).tcname, *args)
                            == 0 as libc::c_int
                        {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if i == 188 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: bindkey: unknown key '%s'\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                            *args,
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        if af != 0 && i >= 166 as libc::c_int && i < 188 as libc::c_int {
                            i
                                -= 166 as libc::c_int
                                    - (188 as libc::c_int - 106 as libc::c_int);
                        } else {
                            i -= 106 as libc::c_int;
                        }
                        newact = if df != 0 {
                            &mut *dmtab.as_mut_ptr().offset(i as isize) as *mut action
                        } else if mf != 0 {
                            &mut *mmtab.as_mut_ptr().offset(i as isize) as *mut action
                        } else {
                            &mut *umtab.as_mut_ptr().offset(i as isize) as *mut action
                        };
                        current_block_1498 = 11751514538013075448;
                    }
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                            newnr = FindCommnr(*args.offset(1 as libc::c_int as isize));
                            if newnr == -(1 as libc::c_int) {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"%s: bindkey: unknown command '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    rc_name,
                                    *args.offset(1 as libc::c_int as isize),
                                );
                                current_block_1498 = 188265764000799656;
                            } else if CheckArgNum(
                                newnr,
                                args.offset(2 as libc::c_int as isize),
                            ) < 0 as libc::c_int
                            {
                                current_block_1498 = 188265764000799656;
                            } else {
                                ClearAction(newact);
                                SaveAction(
                                    newact,
                                    newnr,
                                    args.offset(2 as libc::c_int as isize),
                                    argl.offset(2 as libc::c_int as isize),
                                );
                                if kf == 0 as libc::c_int
                                    && !(*args.offset(1 as libc::c_int as isize)).is_null()
                                {
                                    if !((*kme).str_0).is_null() {
                                        free((*kme).str_0 as *mut libc::c_void);
                                    }
                                    (*kme).str_0 = SaveStrn(*args, *argl);
                                    (*kme).fl = fl | *argl;
                                }
                                current_block_1498 = 2428404172121858456;
                            }
                        } else {
                            ClearAction(newact);
                            current_block_1498 = 2428404172121858456;
                        }
                        match current_block_1498 {
                            188265764000799656 => {}
                            _ => {
                                display = displays;
                                while !display.is_null() {
                                    remap(
                                        i,
                                        if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                                            1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        },
                                    );
                                    display = (*display).d_next;
                                }
                                if kf == 0 as libc::c_int
                                    && (*args.offset(1 as libc::c_int as isize)).is_null()
                                {
                                    if used == 0 && !((*kme).str_0).is_null() {
                                        free((*kme).str_0 as *mut libc::c_void);
                                        (*kme).str_0 = 0 as *mut libc::c_char;
                                        (*kme).fl = 0 as libc::c_int;
                                    }
                                }
                                display = odisp;
                            }
                        }
                    }
                }
            }
            current_block_1498 = 188265764000799656;
        }
        107 => {
            if !(*args).is_null() {
                if ParseNum(act, &mut n) != 0 {
                    current_block_1498 = 188265764000799656;
                } else if n < 0 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: maptimeout: illegal time %d\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        n,
                    );
                    current_block_1498 = 188265764000799656;
                } else {
                    maptimeout = n;
                    current_block_1498 = 7518192951036421866;
                }
            } else {
                current_block_1498 = 7518192951036421866;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if (*args).is_null() || msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"maptimeout is %dms\0" as *const u8 as *const libc::c_char,
                            maptimeout,
                        );
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        106 => {
            (*display).d_dontmap = 1 as libc::c_int;
            current_block_1498 = 188265764000799656;
        }
        105 => {
            (*display).d_mapdefault = 1 as libc::c_int;
            current_block_1498 = 188265764000799656;
        }
        1 | 0 | 6 | 28 => {
            UsersAcl(0 as *mut acluser, argc, args);
            current_block_1498 = 188265764000799656;
        }
        2 => {
            if UserDel(*args.offset(0 as libc::c_int as isize), 0 as *mut *mut acluser)
                != 0
            {
                current_block_1498 = 188265764000799656;
            } else {
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s removed from acl database\0" as *const u8
                            as *const libc::c_char,
                        *args.offset(0 as libc::c_int as isize),
                    );
                }
                current_block_1498 = 188265764000799656;
            }
        }
        3 => {
            if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                if strcmp(
                    *args.offset(1 as libc::c_int as isize),
                    b"none\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    if !(AclLinkUser(
                        *args.offset(0 as libc::c_int as isize),
                        *args.offset(1 as libc::c_int as isize),
                    ) != 0)
                    {
                        if msgok != 0 {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"User %s joined acl-group %s\0" as *const u8
                                    as *const libc::c_char,
                                *args.offset(0 as libc::c_int as isize),
                                *args.offset(1 as libc::c_int as isize),
                            );
                        }
                    }
                } else {
                    let mut u_0: *mut acluser = 0 as *mut acluser;
                    let mut g: *mut aclusergroup = 0 as *mut aclusergroup;
                    u_0 = *FindUserPtr(*args.offset(0 as libc::c_int as isize));
                    if !u_0.is_null() {
                        loop {
                            g = (*u_0).u_group;
                            if g.is_null() {
                                break;
                            }
                            (*u_0).u_group = (*g).next;
                            free(g as *mut libc::c_char as *mut libc::c_void);
                        }
                    }
                }
            } else {
                let mut buf_7: [libc::c_char; 256] = [0; 256];
                let mut p_0: *mut libc::c_char = buf_7.as_mut_ptr();
                let mut ngroups: libc::c_int = 0 as libc::c_int;
                let mut u_1: *mut acluser = 0 as *mut acluser;
                let mut g_0: *mut aclusergroup = 0 as *mut aclusergroup;
                u_1 = *FindUserPtr(*args.offset(0 as libc::c_int as isize));
                if u_1.is_null() {
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"User %s does not exist.\0" as *const u8
                                as *const libc::c_char,
                            *args.offset(0 as libc::c_int as isize),
                        );
                    }
                } else {
                    g_0 = (*u_1).u_group;
                    while !g_0.is_null() {
                        ngroups += 1;
                        ngroups;
                        sprintf(
                            p_0,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            ((*(*g_0).u).u_name).as_mut_ptr(),
                        );
                        p_0 = p_0.offset(strlen(p_0) as isize);
                        if p_0 > buf_7.as_mut_ptr().offset(200 as libc::c_int as isize) {
                            break;
                        }
                        g_0 = (*g_0).next;
                    }
                    if ngroups != 0 {
                        p_0 = p_0.offset(-1);
                        *p_0 = '\0' as i32 as libc::c_char;
                    }
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s's group%s: %s.\0" as *const u8 as *const libc::c_char,
                        *args.offset(0 as libc::c_int as isize),
                        if ngroups == 1 as libc::c_int {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b"s\0" as *const u8 as *const libc::c_char
                        },
                        if ngroups == 0 as libc::c_int {
                            b"none\0" as *const u8 as *const libc::c_char
                        } else {
                            buf_7.as_mut_ptr() as *const libc::c_char
                        },
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        4 | 169 => {
            loop {
                let fresh17 = args;
                args = args.offset(1);
                s = *fresh17;
                if s.is_null() {
                    break;
                }
                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                if AclUmask(
                    if !display.is_null() { (*display).d_user } else { users },
                    s,
                    &mut err,
                ) != 0
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"umask: %s\n\0" as *const u8 as *const libc::c_char,
                        err,
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        115 => {
            if ParseOnOff(act, &mut n) != 0 {
                current_block_1498 = 188265764000799656;
            } else {
                multi = (if n != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    0 as *const libc::c_char
                }) as *mut libc::c_char;
                chsock();
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Multiuser mode %s\0" as *const u8 as *const libc::c_char,
                        if !multi.is_null() {
                            b"enabled\0" as *const u8 as *const libc::c_char
                        } else {
                            b"disabled\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                current_block_1498 = 188265764000799656;
            }
        }
        77 => {
            winexec(args);
            current_block_1498 = 188265764000799656;
        }
        118 => {
            i = ((*display).d_nonblock >= 0 as libc::c_int) as libc::c_int;
            if !(*args).is_null()
                && (*(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
                    || *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
            {
                if ParseNum1000(act, &mut i) != 0 {
                    current_block_1498 = 188265764000799656;
                } else {
                    current_block_1498 = 9046981832224787820;
                }
            } else if ParseSwitch(act, &mut i) == 0 {
                i = if i == 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    1000 as libc::c_int
                };
                current_block_1498 = 9046981832224787820;
            } else {
                current_block_1498 = 188265764000799656;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if msgok != 0 && i == -(1 as libc::c_int) {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"display set to blocking mode\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if msgok != 0 && i == 0 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"display set to nonblocking mode, no timeout\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"display set to nonblocking mode, %.10gs timeout\0"
                                as *const u8 as *const libc::c_char,
                            i as libc::c_double / 1000.0f64,
                        );
                    }
                    (*display).d_nonblock = i;
                    if (*display).d_nonblock <= 0 as libc::c_int {
                        evdeq(&mut (*display).d_blockedev);
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        58 => {
            if !(*args).is_null()
                && (*(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
                    || *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
            {
                if ParseNum1000(act, &mut defnonblock) != 0 {
                    current_block_1498 = 188265764000799656;
                } else {
                    current_block_1498 = 943148765588424804;
                }
            } else if ParseOnOff(act, &mut defnonblock) == 0 {
                defnonblock = if defnonblock == 0 as libc::c_int {
                    -(1 as libc::c_int)
                } else {
                    1000 as libc::c_int
                };
                current_block_1498 = 943148765588424804;
            } else {
                current_block_1498 = 188265764000799656;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if !display.is_null() && *rc_name as libc::c_int != 0 {
                        (*display).d_nonblock = defnonblock;
                        if (*display).d_nonblock <= 0 as libc::c_int {
                            evdeq(&mut (*display).d_blockedev);
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        82 => {
            if (*fore).w_gr == 2 as libc::c_int {
                (*fore).w_gr = 0 as libc::c_int;
            }
            if ParseSwitch(act, &mut (*fore).w_gr) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Will %suse GR\0" as *const u8 as *const libc::c_char,
                    if (*fore).w_gr != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if (*fore).w_gr == 0 as libc::c_int && (*fore).w_FontE as libc::c_int != 0 {
                (*fore).w_gr = 2 as libc::c_int;
            }
            current_block_1498 = 188265764000799656;
        }
        26 => {
            if ParseSwitch(act, &mut (*fore).w_c1) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Will %suse C1\0" as *const u8 as *const libc::c_char,
                    if (*fore).w_c1 != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        14 => {
            if ParseSwitch(act, &mut (*fore).w_bce) == 0 as libc::c_int && msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Will %serase with background color\0" as *const u8
                        as *const libc::c_char,
                    if (*fore).w_bce != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        95 | 74 => {
            if !(*args).is_null()
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"-d\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"encodings directory is %s\0" as *const u8
                            as *const libc::c_char,
                        if !screenencodings.is_null() {
                            screenencodings as *const libc::c_char
                        } else {
                            b"<unset>\0" as *const u8 as *const libc::c_char
                        },
                    );
                } else {
                    free(screenencodings as *mut libc::c_void);
                    screenencodings = SaveStr(*args.offset(1 as libc::c_int as isize));
                }
            } else if !(*args).is_null()
                && strcmp(
                    *args.offset(0 as libc::c_int as isize),
                    b"-l\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"encoding: -l: argument required\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if LoadFontTranslation(
                    -(1 as libc::c_int),
                    *args.offset(1 as libc::c_int as isize),
                ) != 0
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"encoding: could not load utf8 encoding file\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"encoding: utf8 encoding file loaded\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                i = 0 as libc::c_int;
                while i < 2 as libc::c_int {
                    if (*args.offset(i as isize)).is_null() {
                        break;
                    }
                    if !(strcmp(
                        *args.offset(i as isize),
                        b".\0" as *const u8 as *const libc::c_char,
                    ) == 0)
                    {
                        n = FindEncoding(*args.offset(i as isize));
                        if n == -(1 as libc::c_int) {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"encoding: unknown encoding '%s'\0" as *const u8
                                    as *const libc::c_char,
                                *args.offset(i as isize),
                            );
                            break;
                        } else if i == 0 as libc::c_int && !fore.is_null() {
                            WinSwitchEncoding(fore, n);
                            ResetCharsets(fore);
                        } else if i != 0 && !display.is_null() {
                            (*display).d_encoding = n;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            current_block_1498 = 188265764000799656;
        }
        52 | 47 => {
            n = FindEncoding(*args);
            if n == -(1 as libc::c_int) {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"defencoding: unknown encoding '%s'\0" as *const u8
                        as *const libc::c_char,
                    *args,
                );
            } else {
                nwin_default.encoding = n;
            }
            current_block_1498 = 188265764000799656;
        }
        64 => {
            n = (nwin_default.encoding == 8 as libc::c_int) as libc::c_int;
            if ParseSwitch(act, &mut n) == 0 as libc::c_int {
                nwin_default
                    .encoding = if n != 0 { 8 as libc::c_int } else { 0 as libc::c_int };
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Will %suse UTF-8 encoding for new windows\0" as *const u8
                            as *const libc::c_char,
                        if n != 0 {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b"not \0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        172 => {
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                if i != 0 && (*args.offset(i as isize)).is_null() {
                    break;
                }
                if (*args.offset(i as isize)).is_null() {
                    n = ((*fore).w_layer.l_encoding != 8 as libc::c_int) as libc::c_int;
                } else if strcmp(
                    *args.offset(i as isize),
                    b"off\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    n = 0 as libc::c_int;
                } else if strcmp(
                    *args.offset(i as isize),
                    b"on\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    n = 1 as libc::c_int;
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"utf8: illegal argument (%s)\0" as *const u8
                            as *const libc::c_char,
                        *args.offset(i as isize),
                    );
                    break;
                }
                if i == 0 as libc::c_int {
                    WinSwitchEncoding(
                        fore,
                        if n != 0 { 8 as libc::c_int } else { 0 as libc::c_int },
                    );
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"Will %suse UTF-8 encoding\0" as *const u8
                                as *const libc::c_char,
                            if n != 0 {
                                b"\0" as *const u8 as *const libc::c_char
                            } else {
                                b"not \0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                } else if !display.is_null() {
                    (*display)
                        .d_encoding = if n != 0 {
                        8 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                }
                if (*args.offset(i as isize)).is_null() {
                    break;
                }
                i += 1;
                i;
            }
            current_block_1498 = 188265764000799656;
        }
        131 => {
            if !(*args).is_null() {
                if !printcmd.is_null() {
                    free(printcmd as *mut libc::c_void);
                }
                printcmd = 0 as *mut libc::c_char;
                if **args != 0 {
                    printcmd = SaveStr(*args);
                }
            }
            if (*args).is_null() || msgok != 0 {
                if !printcmd.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"using '%s' as print command\0" as *const u8
                            as *const libc::c_char,
                        printcmd,
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"using termcap entries for printing\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_1498 = 188265764000799656;
            } else {
                current_block_1498 = 188265764000799656;
            }
        }
        68 => {
            if !argl.is_null()
                && *argl.offset(0 as libc::c_int as isize) > 0 as libc::c_int
                && !(*args.offset(1 as libc::c_int as isize)).is_null()
                && *argl.offset(1 as libc::c_int as isize) > 0 as libc::c_int
            {
                if *argl.offset(0 as libc::c_int as isize) != 2 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Two characters expected to define a digraph\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    i = digraph_find(*args.offset(0 as libc::c_int as isize));
                    digraphs[i as usize]
                        .d[0 as libc::c_int
                        as usize] = *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_uchar;
                    digraphs[i as usize]
                        .d[1 as libc::c_int
                        as usize] = *(*args.offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_uchar;
                    if parse_input_int(
                        *args.offset(1 as libc::c_int as isize),
                        *argl.offset(1 as libc::c_int as isize),
                        &mut (*digraphs.as_mut_ptr().offset(i as isize)).value,
                    ) == 0
                    {
                        digraphs[i as usize]
                            .value = atoi(*args.offset(1 as libc::c_int as isize));
                        if digraphs[i as usize].value == 0 {
                            if *(*args.offset(1 as libc::c_int as isize))
                                .offset(1 as libc::c_int as isize) == 0
                            {
                                digraphs[i as usize]
                                    .value = *(*args.offset(1 as libc::c_int as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int;
                            } else {
                                let mut t: libc::c_int = 0;
                                let mut s_0: *mut libc::c_uchar = *args
                                    .offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
                                digraphs[i as usize].value = 0 as libc::c_int;
                                while *s_0 != 0 {
                                    let fresh18 = s_0;
                                    s_0 = s_0.offset(1);
                                    t = FromUtf8(
                                        *fresh18 as libc::c_int,
                                        &mut (*digraphs.as_mut_ptr().offset(i as isize)).value,
                                    );
                                    if t == -(1 as libc::c_int) {
                                        continue;
                                    }
                                    if t == -(2 as libc::c_int) {
                                        digraphs[i as usize].value = 0 as libc::c_int;
                                    } else {
                                        digraphs[i as usize].value = t;
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                Input(
                    b"Enter digraph: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    10 as libc::c_int,
                    4 as libc::c_int,
                    Some(
                        digraph_fn
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
                if !(*args).is_null() && **args as libc::c_int != 0 {
                    s = *args;
                    n = strlen(s) as libc::c_int;
                    (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                        .unwrap()(&mut s, &mut n);
                }
            }
            current_block_1498 = 188265764000799656;
        }
        51 => {
            if (*args).is_null() {
                let mut buf_8: [libc::c_char; 256] = [0; 256];
                *buf_8.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
                if !(nwin_default.hstatus).is_null() {
                    AddXChars(
                        buf_8.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                            as libc::c_int,
                        nwin_default.hstatus,
                    );
                }
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"default hstatus is '%s'\0" as *const u8 as *const libc::c_char,
                    buf_8.as_mut_ptr(),
                );
            } else {
                ParseSaveStr(act, &mut nwin_default.hstatus);
                if *nwin_default.hstatus as libc::c_int == 0 as libc::c_int {
                    free(nwin_default.hstatus as *mut libc::c_void);
                    nwin_default.hstatus = 0 as *mut libc::c_char;
                }
            }
            current_block_1498 = 188265764000799656;
        }
        91 => {
            ParseSaveStr(act, &mut (*fore).w_hstatus);
            if *(*fore).w_hstatus as libc::c_int == 0 as libc::c_int {
                free((*fore).w_hstatus as *mut libc::c_void);
                (*fore).w_hstatus = 0 as *mut libc::c_char;
            }
            WindowChanged(fore, 'h' as i32);
            current_block_1498 = 188265764000799656;
        }
        45 | 29 => {
            if (*args).is_null() {
                let mut buf_9: [libc::c_char; 256] = [0; 256];
                *buf_9.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
                if !(nwin_default.charset).is_null() {
                    AddXChars(
                        buf_9.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                            as libc::c_int,
                        nwin_default.charset,
                    );
                }
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"default charset is '%s'\0" as *const u8 as *const libc::c_char,
                    buf_9.as_mut_ptr(),
                );
            } else {
                n = strlen(*args) as libc::c_int;
                if n == 0 as libc::c_int || n > 6 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: %s: string has illegal size.\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        (*comms.as_mut_ptr().offset(nr as isize)).name,
                    );
                } else if n > 4 as libc::c_int
                    && (((*(*args.offset(0 as libc::c_int as isize))
                        .offset(4 as libc::c_int as isize) as libc::c_int) < '0' as i32
                        || *(*args.offset(0 as libc::c_int as isize))
                            .offset(4 as libc::c_int as isize) as libc::c_int
                            > '3' as i32)
                        && *(*args.offset(0 as libc::c_int as isize))
                            .offset(4 as libc::c_int as isize) as libc::c_int
                            != '.' as i32
                        || ((*(*args.offset(0 as libc::c_int as isize))
                            .offset(5 as libc::c_int as isize) as libc::c_int)
                            < '0' as i32
                            || *(*args.offset(0 as libc::c_int as isize))
                                .offset(5 as libc::c_int as isize) as libc::c_int
                                > '3' as i32)
                            && *(*args.offset(0 as libc::c_int as isize))
                                .offset(5 as libc::c_int as isize) as libc::c_int != 0
                            && *(*args.offset(0 as libc::c_int as isize))
                                .offset(5 as libc::c_int as isize) as libc::c_int
                                != '.' as i32)
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: %s: illegal mapping number.\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                        (*comms.as_mut_ptr().offset(nr as isize)).name,
                    );
                } else if nr == 29 as libc::c_int {
                    SetCharsets(fore, *args);
                } else {
                    if !(nwin_default.charset).is_null() {
                        free(nwin_default.charset as *mut libc::c_void);
                    }
                    nwin_default.charset = SaveStr(*args);
                }
            }
            current_block_1498 = 188265764000799656;
        }
        10 => {
            s = *args.offset(0 as libc::c_int as isize);
            if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
                i = *s as libc::c_int - '0' as i32;
            } else {
                i = 0 as libc::c_int;
                while i < 8 as libc::c_int {
                    if *s as libc::c_int
                        == (*::std::mem::transmute::<
                            &[u8; 9],
                            &[libc::c_char; 9],
                        >(b"dubrsBiI\0"))[i as usize] as libc::c_int
                    {
                        break;
                    }
                    i += 1;
                    i;
                }
            }
            s = s.offset(1);
            s;
            nr = 0 as libc::c_int;
            if *s as libc::c_int != 0
                && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
                && *s.offset(2 as libc::c_int as isize) == 0
            {
                if *s as libc::c_int == 'd' as i32
                    && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'd' as i32
                {
                    nr = 3 as libc::c_int;
                } else if *s as libc::c_int == '.' as i32
                    && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'd' as i32
                {
                    nr = 2 as libc::c_int;
                } else if *s as libc::c_int == 'd' as i32
                    && *s.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                {
                    nr = 1 as libc::c_int;
                } else if *s as libc::c_int != '.' as i32
                    || *s.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32
                {
                    s = s.offset(-1);
                    s;
                }
                s = s.offset(2 as libc::c_int as isize);
            }
            if *s as libc::c_int != 0 || i < 0 as libc::c_int || i >= 8 as libc::c_int {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: attrcolor: unknown attribute '%s'.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                    *args.offset(0 as libc::c_int as isize),
                );
            } else {
                n = 0 as libc::c_int;
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    n = ParseAttrColor(
                        *args.offset(1 as libc::c_int as isize),
                        *args.offset(2 as libc::c_int as isize),
                        1 as libc::c_int,
                    );
                }
                if !(n == -(1 as libc::c_int)) {
                    (*attr2color.as_mut_ptr().offset(i as isize))[nr as usize] = n;
                    n = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < 8 as libc::c_int {
                        if (*attr2color
                            .as_mut_ptr()
                            .offset(i as isize))[0 as libc::c_int as usize] != 0
                            || (*attr2color
                                .as_mut_ptr()
                                .offset(i as isize))[1 as libc::c_int as usize] != 0
                            || (*attr2color
                                .as_mut_ptr()
                                .offset(i as isize))[2 as libc::c_int as usize] != 0
                            || (*attr2color
                                .as_mut_ptr()
                                .offset(i as isize))[3 as libc::c_int as usize] != 0
                        {
                            n |= (1 as libc::c_int) << i;
                        }
                        i += 1;
                        i;
                    }
                    nattr2color = n;
                }
            }
            current_block_1498 = 188265764000799656;
        }
        140 => {
            i = -(1 as libc::c_int);
            if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"bell\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                i = REND_BELL as libc::c_int;
                current_block_1498 = 9185264750798732957;
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"monitor\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                i = REND_MONITOR as libc::c_int;
                current_block_1498 = 9185264750798732957;
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"silence\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                i = REND_SILENCE as libc::c_int;
                current_block_1498 = 9185264750798732957;
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"so\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Invalid option '%s' for rendition\0" as *const u8
                        as *const libc::c_char,
                    *args.offset(0 as libc::c_int as isize),
                );
                current_block_1498 = 188265764000799656;
            } else {
                current_block_1498 = 9185264750798732957;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    args = args.offset(1);
                    args;
                    argl = argl.offset(1);
                    argl;
                    if i != -(1 as libc::c_int) {
                        *renditions
                            .as_mut_ptr()
                            .offset(
                                i as isize,
                            ) = ParseAttrColor(
                            *args.offset(0 as libc::c_int as isize),
                            *args.offset(1 as libc::c_int as isize),
                            1 as libc::c_int,
                        );
                        WindowChanged(0 as *mut win, 'w' as i32);
                        WindowChanged(0 as *mut win, 'W' as i32);
                        WindowChanged(0 as *mut win, 0 as libc::c_int);
                        current_block_1498 = 188265764000799656;
                    } else {
                        current_block_1498 = 10867446318941641510;
                    }
                }
            }
        }
        155 => {
            current_block_1498 = 10867446318941641510;
        }
        157 => {
            do_source(*args);
            current_block_1498 = 188265764000799656;
        }
        161 => {
            s = 0 as *mut libc::c_char;
            if (*args).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s:%s screen login\0" as *const u8 as *const libc::c_char,
                    HostName.as_mut_ptr(),
                    SockPath.as_mut_ptr(),
                );
                InputSu(
                    (*display).d_fore,
                    &mut (*display).d_user,
                    0 as *mut libc::c_char,
                );
            } else if (*args.offset(1 as libc::c_int as isize)).is_null() {
                InputSu(
                    (*display).d_fore,
                    &mut (*display).d_user,
                    *args.offset(0 as libc::c_int as isize),
                );
            } else if (*args.offset(2 as libc::c_int as isize)).is_null() {
                s = DoSu(
                    &mut (*display).d_user,
                    *args.offset(0 as libc::c_int as isize),
                    *args.offset(1 as libc::c_int as isize),
                    b"\xFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                s = DoSu(
                    &mut (*display).d_user,
                    *args.offset(0 as libc::c_int as isize),
                    *args.offset(1 as libc::c_int as isize),
                    *args.offset(2 as libc::c_int as isize),
                );
            }
            if !s.is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    s,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        158 => {
            s = *args.offset(0 as libc::c_int as isize);
            if !s.is_null()
                && strcmp(s, b"-v\0" as *const u8 as *const libc::c_char) == 0
            {
                AddCanvas((1 as libc::c_int) << 1 as libc::c_int);
            } else {
                AddCanvas((1 as libc::c_int) << 0 as libc::c_int);
            }
            Activate(-(1 as libc::c_int));
            current_block_1498 = 188265764000799656;
        }
        138 => {
            RemCanvas();
            Activate(-(1 as libc::c_int));
            current_block_1498 = 188265764000799656;
        }
        121 => {
            OneCanvas();
            Activate(-(1 as libc::c_int));
            current_block_1498 = 188265764000799656;
        }
        78 => {
            (*(*display).d_forecv).c_xoff = (*(*display).d_forecv).c_xs;
            (*(*display).d_forecv).c_yoff = (*(*display).d_forecv).c_ys;
            RethinkViewportOffsets((*display).d_forecv);
            ResizeLayer(
                (*(*display).d_forecv).c_layer,
                (*(*display).d_forecv).c_xe - (*(*display).d_forecv).c_xs
                    + 1 as libc::c_int,
                (*(*display).d_forecv).c_ye - (*(*display).d_forecv).c_ys
                    + 1 as libc::c_int,
                0 as *mut display,
            );
            flayer = (*(*display).d_forecv).c_layer;
            LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
            current_block_1498 = 188265764000799656;
        }
        80 => {
            let mut cv: *mut canvas = 0 as *mut canvas;
            if (*args).is_null()
                || strcmp(*args, b"next\0" as *const u8 as *const libc::c_char) == 0
            {
                cv = if !((*(*display).d_forecv).c_next).is_null() {
                    (*(*display).d_forecv).c_next
                } else {
                    (*display).d_cvlist
                };
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"prev\0" as *const u8 as *const libc::c_char) == 0 {
                cv = (*display).d_cvlist;
                while !((*cv).c_next).is_null() && (*cv).c_next != (*display).d_forecv {
                    cv = (*cv).c_next;
                }
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"top\0" as *const u8 as *const libc::c_char) == 0 {
                cv = (*display).d_cvlist;
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"bottom\0" as *const u8 as *const libc::c_char) == 0
            {
                cv = (*display).d_cvlist;
                while !((*cv).c_next).is_null() {
                    cv = (*cv).c_next;
                }
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"up\0" as *const u8 as *const libc::c_char) == 0 {
                cv = FindCanvas(
                    (*(*display).d_forecv).c_xs,
                    (*(*display).d_forecv).c_ys - 1 as libc::c_int,
                );
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"down\0" as *const u8 as *const libc::c_char) == 0 {
                cv = FindCanvas(
                    (*(*display).d_forecv).c_xs,
                    (*(*display).d_forecv).c_ye + 2 as libc::c_int,
                );
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"left\0" as *const u8 as *const libc::c_char) == 0 {
                cv = FindCanvas(
                    (*(*display).d_forecv).c_xs - 1 as libc::c_int,
                    (*(*display).d_forecv).c_ys,
                );
                current_block_1498 = 13644492758130880546;
            } else if strcmp(*args, b"right\0" as *const u8 as *const libc::c_char) == 0
            {
                cv = FindCanvas(
                    (*(*display).d_forecv).c_xe + 1 as libc::c_int,
                    (*(*display).d_forecv).c_ys,
                );
                current_block_1498 = 13644492758130880546;
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: usage: focus [next|prev|up|down|left|right|top|bottom]\0"
                        as *const u8 as *const libc::c_char,
                    rc_name,
                );
                current_block_1498 = 188265764000799656;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    SetForeCanvas(display, cv);
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        142 => {
            i = 0 as libc::c_int;
            if (*(*display).d_forecv).c_slorient == 0 as libc::c_int {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"resize: need more than one region\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                while !(*args).is_null() {
                    if strcmp(*args, b"-h\0" as *const u8 as *const libc::c_char) == 0 {
                        i |= 1 as libc::c_int;
                    } else if strcmp(*args, b"-v\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        i |= 2 as libc::c_int;
                    } else if strcmp(*args, b"-b\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        i |= 1 as libc::c_int | 2 as libc::c_int;
                    } else if strcmp(*args, b"-p\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        i
                            |= if (*(*display).d_forecv).c_slorient
                                == (1 as libc::c_int) << 0 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                2 as libc::c_int
                            };
                    } else {
                        if !(strcmp(*args, b"-l\0" as *const u8 as *const libc::c_char)
                            == 0)
                        {
                            break;
                        }
                        i |= 4 as libc::c_int;
                    }
                    args = args.offset(1);
                    args;
                }
                if !(*args).is_null()
                    && !(*args.offset(1 as libc::c_int as isize)).is_null()
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: usage: resize [-h] [-v] [-l] [num]\n\0" as *const u8
                            as *const libc::c_char,
                        rc_name,
                    );
                } else if !(*args).is_null() {
                    ResizeRegions(*args, i);
                } else {
                    Input(
                        resizeprompts[i as usize],
                        20 as libc::c_int,
                        4 as libc::c_int,
                        Some(
                            ResizeFin
                                as unsafe extern "C" fn(
                                    *mut libc::c_char,
                                    libc::c_int,
                                    *mut libc::c_char,
                                ) -> (),
                        ),
                        0 as *mut libc::c_char,
                        i,
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        148 => {
            ParseSwitch(act, &mut separate_sids);
            current_block_1498 = 188265764000799656;
        }
        76 => {
            args = SaveArgs(args);
            i = 0 as libc::c_int;
            while !(*args.offset(i as isize)).is_null() {
                if *(*args.offset(i as isize)).offset(0 as libc::c_int as isize) != 0 {
                    Colonfin(
                        *args.offset(i as isize),
                        strlen(*args.offset(i as isize)) as libc::c_int,
                        0 as *mut libc::c_char,
                    );
                }
                free(*args.offset(i as isize) as *mut libc::c_void);
                i += 1;
                i;
            }
            free(args as *mut libc::c_void);
            current_block_1498 = 188265764000799656;
        }
        8 => {
            ParseSwitch(act, &mut use_altscreen);
            if msgok != 0 {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"Will %sdo alternate screen switching\0" as *const u8
                        as *const libc::c_char,
                    if use_altscreen != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"not \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            current_block_1498 = 188265764000799656;
        }
        109 => {
            if (*args.offset(0 as libc::c_int as isize)).is_null() {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"maximum windows allowed: %d\0" as *const u8 as *const libc::c_char,
                    maxwin,
                );
            } else if !(ParseNum(act, &mut n) != 0) {
                if n < 1 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"illegal maxwin number specified\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if n > 2048 as libc::c_int {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"maximum 2048 windows allowed\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if n > maxwin && !windows.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"may increase maxwin only when there's no window\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    if windows.is_null() {
                        wtab = realloc(
                            wtab as *mut libc::c_void,
                            (n as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut win>() as libc::c_ulong,
                                ),
                        ) as *mut *mut win;
                        bzero(
                            wtab as *mut libc::c_void,
                            (n as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut win>() as libc::c_ulong,
                                ),
                        );
                    }
                    maxwin = n;
                }
            }
            current_block_1498 = 188265764000799656;
        }
        13 => {
            if ParseBase(
                act,
                *args,
                &mut n,
                10 as libc::c_int,
                b"decimal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                current_block_1498 = 188265764000799656;
            } else {
                if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    setbacktick(
                        n,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as *mut *mut libc::c_char,
                    );
                    current_block_1498 = 11534312634323218616;
                } else {
                    let mut lifespan: libc::c_int = 0;
                    let mut tick: libc::c_int = 0;
                    if argc < 4 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: usage: backtick num [lifespan tick cmd args...]\0"
                                as *const u8 as *const libc::c_char,
                            rc_name,
                        );
                        current_block_1498 = 188265764000799656;
                    } else if ParseBase(
                        act,
                        *args.offset(1 as libc::c_int as isize),
                        &mut lifespan,
                        10 as libc::c_int,
                        b"decimal\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) != 0
                    {
                        current_block_1498 = 188265764000799656;
                    } else if ParseBase(
                        act,
                        *args.offset(2 as libc::c_int as isize),
                        &mut tick,
                        10 as libc::c_int,
                        b"decimal\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) != 0
                    {
                        current_block_1498 = 188265764000799656;
                    } else {
                        setbacktick(
                            n,
                            lifespan,
                            tick,
                            SaveArgs(args.offset(3 as libc::c_int as isize)),
                        );
                        current_block_1498 = 11534312634323218616;
                    }
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        WindowChanged(0 as *mut win, '`' as i32);
                        current_block_1498 = 188265764000799656;
                    }
                }
            }
        }
        19 => {
            if !blankerprg.is_null() {
                RunBlanker(blankerprg);
            } else {
                ClearAll();
                CursorVisibility(-(1 as libc::c_int));
                (*display).d_blocked = 4 as libc::c_int;
            }
            current_block_1498 = 188265764000799656;
        }
        20 => {
            if (*args.offset(0 as libc::c_int as isize)).is_null() {
                if !blankerprg.is_null() {
                    let mut path: [libc::c_char; 4096] = [0; 4096];
                    let mut p_1: *mut libc::c_char = path.as_mut_ptr();
                    let mut pp_2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                    pp_2 = blankerprg;
                    while !(*pp_2).is_null() {
                        p_1 = p_1
                            .offset(
                                snprintf(
                                    p_1,
                                    (::std::mem::size_of::<[libc::c_char; 4096]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(
                                            p_1.offset_from(path.as_mut_ptr()) as libc::c_long
                                                as libc::c_ulong,
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    b"%s \0" as *const u8 as *const libc::c_char,
                                    *pp_2,
                                ) as isize,
                            );
                        pp_2 = pp_2.offset(1);
                        pp_2;
                    }
                    *p_1
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = '\0' as i32 as libc::c_char;
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"blankerprg: %s\0" as *const u8 as *const libc::c_char,
                        path.as_mut_ptr(),
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"No blankerprg set.\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                if !blankerprg.is_null() {
                    let mut pp_3: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                    pp_3 = blankerprg;
                    while !(*pp_3).is_null() {
                        free(*pp_3 as *mut libc::c_void);
                        pp_3 = pp_3.offset(1);
                        pp_3;
                    }
                    free(blankerprg as *mut libc::c_void);
                    blankerprg = 0 as *mut *mut libc::c_char;
                }
                if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) != 0
                {
                    blankerprg = SaveArgs(args);
                }
            }
            current_block_1498 = 188265764000799656;
        }
        92 => {
            if !(*args).is_null() {
                let mut olddisplay_2: *mut display = display;
                if strcmp(*args, b"off\0" as *const u8 as *const libc::c_char) == 0 {
                    idletimo = 0 as libc::c_int;
                } else if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) != 0
                {
                    idletimo = atoi(*args) * 1000 as libc::c_int;
                }
                if argc > 1 as libc::c_int {
                    i = FindCommnr(*args.offset(1 as libc::c_int as isize));
                    if i == -(1 as libc::c_int) {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"%s: idle: unknown command '%s'\0" as *const u8
                                as *const libc::c_char,
                            rc_name,
                            *args.offset(1 as libc::c_int as isize),
                        );
                        current_block_1498 = 188265764000799656;
                    } else if CheckArgNum(i, args.offset(2 as libc::c_int as isize))
                        < 0 as libc::c_int
                    {
                        current_block_1498 = 188265764000799656;
                    } else {
                        ClearAction(&mut idleaction);
                        SaveAction(
                            &mut idleaction,
                            i,
                            args.offset(2 as libc::c_int as isize),
                            argl.offset(2 as libc::c_int as isize),
                        );
                        current_block_1498 = 10962111400457132340;
                    }
                } else {
                    current_block_1498 = 10962111400457132340;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        display = displays;
                        while !display.is_null() {
                            ResetIdle();
                            display = (*display).d_next;
                        }
                        display = olddisplay_2;
                        current_block_1498 = 1281038683408929997;
                    }
                }
            } else {
                current_block_1498 = 1281038683408929997;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if msgok != 0 {
                        if idletimo != 0 {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"idle timeout %ds, %s\0" as *const u8
                                    as *const libc::c_char,
                                idletimo / 1000 as libc::c_int,
                                (*comms.as_mut_ptr().offset(idleaction.nr as isize)).name,
                            );
                        } else {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"idle off\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    current_block_1498 = 188265764000799656;
                }
            }
        }
        81 => {
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int && !(*args.offset(i as isize)).is_null() {
                if strcmp(
                    *args.offset(i as isize),
                    b"max\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcmp(
                        *args.offset(i as isize),
                        b"_\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    n = -(1 as libc::c_int);
                } else {
                    n = atoi(*args.offset(i as isize));
                }
                if i == 0 as libc::c_int {
                    focusminwidth = n;
                } else {
                    focusminheight = n;
                }
                i += 1;
                i;
            }
            if msgok != 0 {
                let mut b: [[libc::c_char; 20]; 2] = [[0; 20]; 2];
                i = 0 as libc::c_int;
                while i < 2 as libc::c_int {
                    n = if i == 0 as libc::c_int {
                        focusminwidth
                    } else {
                        focusminheight
                    };
                    if n == -(1 as libc::c_int) {
                        strcpy(
                            (b[i as usize]).as_mut_ptr(),
                            b"max\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        sprintf(
                            (b[i as usize]).as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            n,
                        );
                    }
                    i += 1;
                    i;
                }
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"focus min size is %s %s\n\0" as *const u8 as *const libc::c_char,
                    (b[0 as libc::c_int as usize]).as_mut_ptr(),
                    (b[1 as libc::c_int as usize]).as_mut_ptr(),
                );
            }
            current_block_1498 = 188265764000799656;
        }
        83 => {
            if !(*args).is_null() {
                (*fore).w_group = 0 as *mut win;
                if *(*args.offset(0 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) != 0
                {
                    (*fore).w_group = WindowByName(*args);
                    if (*fore).w_group == fore
                        || !((*fore).w_group).is_null()
                            && (*(*fore).w_group).w_type != 3 as libc::c_int
                    {
                        (*fore).w_group = 0 as *mut win;
                    }
                }
                WindowChanged(0 as *mut win, 'w' as i32);
                WindowChanged(0 as *mut win, 'W' as i32);
                WindowChanged(0 as *mut win, 0 as libc::c_int);
            }
            if msgok != 0 {
                if !((*fore).w_group).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"window group is %d (%s)\n\0" as *const u8
                            as *const libc::c_char,
                        (*(*fore).w_group).w_number,
                        (*(*fore).w_group).w_title,
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"window belongs to no group\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        98 => {
            if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"title\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if display.is_null() {
                    if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                        if layout_attach.is_null()
                            || layout_attach == &mut layout_last_marker as *mut layout
                        {
                            layout_attach = CreateLayout(
                                *args.offset(1 as libc::c_int as isize),
                                0 as libc::c_int,
                            );
                        } else {
                            RenameLayout(
                                layout_attach,
                                *args.offset(1 as libc::c_int as isize),
                            );
                        }
                    }
                } else if ((*display).d_layout).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"not on a layout\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"current layout is %d (%s)\0" as *const u8
                            as *const libc::c_char,
                        (*(*display).d_layout).lay_number,
                        (*(*display).d_layout).lay_title,
                    );
                } else {
                    RenameLayout(
                        (*display).d_layout,
                        *args.offset(1 as libc::c_int as isize),
                    );
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"number\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if display.is_null() {
                    if !(*args.offset(1 as libc::c_int as isize)).is_null()
                        && !layout_attach.is_null()
                        && layout_attach != &mut layout_last_marker as *mut layout
                    {
                        RenumberLayout(
                            layout_attach,
                            atoi(*args.offset(1 as libc::c_int as isize)),
                        );
                    }
                } else if ((*display).d_layout).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"not on a layout\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"This is layout %d (%s).\n\0" as *const u8
                            as *const libc::c_char,
                        (*(*display).d_layout).lay_number,
                        (*(*display).d_layout).lay_title,
                    );
                } else {
                    RenumberLayout(
                        (*display).d_layout,
                        atoi(*args.offset(1 as libc::c_int as isize)),
                    );
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"autosave\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if display.is_null() {
                    if !(*args.offset(1 as libc::c_int as isize)).is_null()
                        && !layout_attach.is_null()
                        && layout_attach != &mut layout_last_marker as *mut layout
                    {
                        if strcmp(
                            *args.offset(1 as libc::c_int as isize),
                            b"on\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*layout_attach).lay_autosave = 1 as libc::c_int;
                        } else if strcmp(
                            *args.offset(1 as libc::c_int as isize),
                            b"off\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*layout_attach).lay_autosave = 0 as libc::c_int;
                        }
                    }
                } else if ((*display).d_layout).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"not on a layout\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                        if strcmp(
                            *args.offset(1 as libc::c_int as isize),
                            b"on\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*(*display).d_layout).lay_autosave = 1 as libc::c_int;
                            current_block_1498 = 3959729371146987326;
                        } else if strcmp(
                            *args.offset(1 as libc::c_int as isize),
                            b"off\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*(*display).d_layout).lay_autosave = 0 as libc::c_int;
                            current_block_1498 = 3959729371146987326;
                        } else {
                            if (*act).quiet == 0 {
                                Some(
                                    Msg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else if queryflag >= 0 as libc::c_int {
                                Some(
                                    QueryMsg
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            } else {
                                Some(
                                    Dummy
                                        as unsafe extern "C" fn(
                                            libc::c_int,
                                            *const libc::c_char,
                                            ...
                                        ) -> (),
                                )
                            }
                                .unwrap()(
                                0 as libc::c_int,
                                b"invalid argument. Give 'on' or 'off\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block_1498 = 188265764000799656;
                        }
                    } else {
                        current_block_1498 = 3959729371146987326;
                    }
                    match current_block_1498 {
                        188265764000799656 => {}
                        _ => {
                            if msgok != 0 {
                                if (*act).quiet == 0 {
                                    Some(
                                        Msg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else if queryflag >= 0 as libc::c_int {
                                    Some(
                                        QueryMsg
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                } else {
                                    Some(
                                        Dummy
                                            as unsafe extern "C" fn(
                                                libc::c_int,
                                                *const libc::c_char,
                                                ...
                                            ) -> (),
                                    )
                                }
                                    .unwrap()(
                                    0 as libc::c_int,
                                    b"autosave is %s\0" as *const u8 as *const libc::c_char,
                                    if (*(*display).d_layout).lay_autosave != 0 {
                                        b"on\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"off\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                        }
                    }
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"new\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                let mut t_0: *mut libc::c_char = *args.offset(1 as libc::c_int as isize);
                n = 0 as libc::c_int;
                if !t_0.is_null() {
                    while *t_0 as libc::c_int >= '0' as i32
                        && *t_0 as libc::c_int <= '9' as i32
                    {
                        t_0 = t_0.offset(1);
                        t_0;
                    }
                    if t_0 != *args.offset(1 as libc::c_int as isize)
                        && (*t_0 == 0 || *t_0 as libc::c_int == ':' as i32)
                    {
                        n = atoi(*args.offset(1 as libc::c_int as isize));
                        if *t_0 != 0 {
                            t_0 = t_0.offset(1);
                            t_0;
                        }
                    } else {
                        t_0 = *args.offset(1 as libc::c_int as isize);
                    }
                }
                if t_0.is_null() || *t_0 == 0 {
                    t_0 = b"layout\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                NewLayout(t_0, n);
                Activate(-(1 as libc::c_int));
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"save\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"usage: layout save <name>\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if !display.is_null() {
                    SaveLayout(
                        *args.offset(1 as libc::c_int as isize),
                        &mut (*display).d_canvas,
                    );
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"select\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if display.is_null() {
                    if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                        layout_attach = FindLayout(
                            *args.offset(1 as libc::c_int as isize),
                        );
                    }
                } else if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    Input(
                        b"Switch to layout: \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        20 as libc::c_int,
                        0 as libc::c_int,
                        Some(
                            SelectLayoutFin
                                as unsafe extern "C" fn(
                                    *mut libc::c_char,
                                    libc::c_int,
                                    *mut libc::c_char,
                                ) -> (),
                        ),
                        0 as *mut libc::c_char,
                        0 as libc::c_int,
                    );
                } else {
                    SelectLayoutFin(
                        *args.offset(1 as libc::c_int as isize),
                        strlen(*args.offset(1 as libc::c_int as isize)) as libc::c_int,
                        0 as *mut libc::c_char,
                    );
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"next\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if display.is_null() {
                    if !layout_attach.is_null()
                        && layout_attach != &mut layout_last_marker as *mut layout
                    {
                        layout_attach = if !((*layout_attach).lay_next).is_null() {
                            (*layout_attach).lay_next
                        } else {
                            layouts
                        };
                    }
                } else {
                    let mut lay: *mut layout = (*display).d_layout;
                    if !lay.is_null() {
                        lay = if !((*lay).lay_next).is_null() {
                            (*lay).lay_next
                        } else {
                            layouts
                        };
                    } else {
                        lay = layouts;
                    }
                    if lay.is_null() {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"no layout defined\0" as *const u8 as *const libc::c_char,
                        );
                    } else if !(lay == (*display).d_layout) {
                        LoadLayout(lay, &mut (*display).d_canvas);
                        Activate(-(1 as libc::c_int));
                    }
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"prev\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                let mut lay_0: *mut layout = if !display.is_null() {
                    (*display).d_layout
                } else {
                    layout_attach
                };
                let mut target: *mut layout = lay_0;
                if !lay_0.is_null() {
                    lay_0 = layouts;
                    while !((*lay_0).lay_next).is_null() && (*lay_0).lay_next != target {
                        lay_0 = (*lay_0).lay_next;
                    }
                } else {
                    lay_0 = layouts;
                }
                if display.is_null() {
                    layout_attach = lay_0;
                } else if lay_0.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"no layout defined\0" as *const u8 as *const libc::c_char,
                    );
                } else if !(lay_0 == (*display).d_layout) {
                    LoadLayout(lay_0, &mut (*display).d_canvas);
                    Activate(-(1 as libc::c_int));
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"attach\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if (*args.offset(1 as libc::c_int as isize)).is_null() {
                    if layout_attach.is_null() {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"no attach layout set\0" as *const u8 as *const libc::c_char,
                        );
                    } else if layout_attach == &mut layout_last_marker as *mut layout {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"will attach to last layout\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"will attach to layout %d (%s)\0" as *const u8
                                as *const libc::c_char,
                            (*layout_attach).lay_number,
                            (*layout_attach).lay_title,
                        );
                    }
                } else if strcmp(
                    *args.offset(1 as libc::c_int as isize),
                    b":last\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    layout_attach = &mut layout_last_marker;
                } else if *(*args.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) == 0
                {
                    layout_attach = 0 as *mut layout;
                } else {
                    let mut lay_1: *mut layout = 0 as *mut layout;
                    lay_1 = FindLayout(*args.offset(1 as libc::c_int as isize));
                    if lay_1.is_null() {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"unknown layout '%s'\0" as *const u8 as *const libc::c_char,
                            *args.offset(1 as libc::c_int as isize),
                        );
                    } else {
                        layout_attach = lay_1;
                    }
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"show\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                ShowLayouts(-(1 as libc::c_int));
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"remove\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                let mut lay_2: *mut layout = if !display.is_null() {
                    (*display).d_layout
                } else {
                    layouts
                };
                if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                    lay_2 = if !layouts.is_null() {
                        FindLayout(*args.offset(1 as libc::c_int as isize))
                    } else {
                        0 as *mut layout
                    };
                    if lay_2.is_null() {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"unknown layout '%s'\0" as *const u8 as *const libc::c_char,
                            *args.offset(1 as libc::c_int as isize),
                        );
                        current_block_1498 = 188265764000799656;
                    } else {
                        current_block_1498 = 17321388432876444877;
                    }
                } else {
                    current_block_1498 = 17321388432876444877;
                }
                match current_block_1498 {
                    188265764000799656 => {}
                    _ => {
                        if !lay_2.is_null() {
                            RemoveLayout(lay_2);
                        }
                    }
                }
            } else if strcmp(
                *args.offset(0 as libc::c_int as isize),
                b"dump\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if display.is_null() {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Must have a display for 'layout dump'.\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if LayoutDumpCanvas(
                    &mut (*display).d_canvas,
                    (if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                        *args.offset(1 as libc::c_int as isize) as *const libc::c_char
                    } else {
                        b"layout-dump\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char,
                ) == 0
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        *__errno_location(),
                        b"Error dumping layout.\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Layout dumped to \"%s\"\0" as *const u8 as *const libc::c_char,
                        if !(*args.offset(1 as libc::c_int as isize)).is_null() {
                            *args.offset(1 as libc::c_int as isize)
                                as *const libc::c_char
                        } else {
                            b"layout-dump\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"unknown layout subcommand\0" as *const u8 as *const libc::c_char,
                );
            }
            current_block_1498 = 188265764000799656;
        }
        31 => {
            if ParseSwitch(act, &mut cjkwidth) == 0 as libc::c_int {
                if msgok != 0 {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"Treat ambiguous width characters as %s width\0" as *const u8
                            as *const libc::c_char,
                        if cjkwidth != 0 {
                            b"full\0" as *const u8 as *const libc::c_char
                        } else {
                            b"half\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
            current_block_1498 = 188265764000799656;
        }
        153 | _ => {
            current_block_1498 = 188265764000799656;
        }
    }
    match current_block_1498 {
        10867446318941641510 => {
            i = 0 as libc::c_int;
            if !(*args).is_null() {
                i = ParseAttrColor(
                    *args,
                    *args.offset(1 as libc::c_int as isize),
                    1 as libc::c_int,
                );
                if i == -(1 as libc::c_int) {
                    current_block_1498 = 188265764000799656;
                } else {
                    ApplyAttrColor(i, &mut mchar_so);
                    WindowChanged(0 as *mut win, 0 as libc::c_int);
                    current_block_1498 = 4264774981795876775;
                }
            } else {
                current_block_1498 = 4264774981795876775;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    if msgok != 0 {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"Standout attributes 0x%02x  color 0x%02x\0" as *const u8
                                as *const libc::c_char,
                            mchar_so.attr as libc::c_int,
                            0x99 as libc::c_int ^ mchar_so.color as libc::c_int,
                        );
                    }
                }
            }
        }
        16816390665721891485 => {
            if *argl == 0 as libc::c_int {
                SetEscape(0 as *mut acluser, -(1 as libc::c_int), -(1 as libc::c_int));
                current_block_1498 = 17569780573092599132;
            } else if *argl == 2 as libc::c_int {
                SetEscape(
                    0 as *mut acluser,
                    *(*args.offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                    *(*args.offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                );
                current_block_1498 = 17569780573092599132;
            } else {
                if (*act).quiet == 0 {
                    Some(
                        Msg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else if queryflag >= 0 as libc::c_int {
                    Some(
                        QueryMsg
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                } else {
                    Some(
                        Dummy
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_char,
                                ...
                            ) -> (),
                    )
                }
                    .unwrap()(
                    0 as libc::c_int,
                    b"%s: two characters required after defescape.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
                current_block_1498 = 188265764000799656;
            }
            match current_block_1498 {
                188265764000799656 => {}
                _ => {
                    CheckEscape();
                }
            }
        }
        8653303946012650245 => {
            let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut dch: libc::c_char = 0;
            let mut l: libc::c_int = 0 as libc::c_int;
            let mut enc: libc::c_int = -(1 as libc::c_int);
            s = *args;
            if s.is_null() {
                Input(
                    b"Paste from register:\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    1 as libc::c_int,
                    2 as libc::c_int,
                    Some(
                        ins_reg_fn
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                *mut libc::c_char,
                            ) -> (),
                    ),
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            } else if !((*args.offset(1 as libc::c_int as isize)).is_null()
                && fore.is_null())
            {
                if !(*args.offset(1 as libc::c_int as isize)).is_null()
                    && *argl.offset(1 as libc::c_int as isize) != 1 as libc::c_int
                {
                    if (*act).quiet == 0 {
                        Some(
                            Msg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else if queryflag >= 0 as libc::c_int {
                        Some(
                            QueryMsg
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    } else {
                        Some(
                            Dummy
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    *const libc::c_char,
                                    ...
                                ) -> (),
                        )
                    }
                        .unwrap()(
                        0 as libc::c_int,
                        b"%s: paste destination: character, ^x, or (octal) \\032 expected.\0"
                            as *const u8 as *const libc::c_char,
                        rc_name,
                    );
                } else {
                    if !fore.is_null() {
                        enc = (*fore).w_layer.l_encoding;
                    }
                    s = *args;
                    ss = s;
                    loop {
                        ch = *ss;
                        if !(ch != 0) {
                            break;
                        }
                        if ch as libc::c_int == '.' as i32 {
                            if enc == -(1 as libc::c_int) {
                                enc = (*user).u_plop.enc;
                            }
                            if enc != (*user).u_plop.enc {
                                l
                                    += RecodeBuf(
                                        (*user).u_plop.buf as *mut libc::c_uchar,
                                        (*user).u_plop.len,
                                        (*user).u_plop.enc,
                                        enc,
                                        0 as *mut libc::c_uchar,
                                    );
                            } else {
                                l += (*user).u_plop.len;
                            }
                        } else {
                            if enc == -(1 as libc::c_int) {
                                enc = plop_tab[ch as libc::c_uchar as libc::c_int as usize]
                                    .enc;
                            }
                            if enc
                                != plop_tab[ch as libc::c_uchar as libc::c_int as usize].enc
                            {
                                l
                                    += RecodeBuf(
                                        plop_tab[ch as libc::c_uchar as libc::c_int as usize].buf
                                            as *mut libc::c_uchar,
                                        plop_tab[ch as libc::c_uchar as libc::c_int as usize].len,
                                        plop_tab[ch as libc::c_uchar as libc::c_int as usize].enc,
                                        enc,
                                        0 as *mut libc::c_uchar,
                                    );
                            } else {
                                l
                                    += plop_tab[ch as libc::c_uchar as libc::c_int as usize]
                                        .len;
                            }
                        }
                        ss = ss.offset(1);
                        ss;
                    }
                    if l == 0 as libc::c_int {
                        if (*act).quiet == 0 {
                            Some(
                                Msg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else if queryflag >= 0 as libc::c_int {
                            Some(
                                QueryMsg
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        } else {
                            Some(
                                Dummy
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *const libc::c_char,
                                        ...
                                    ) -> (),
                            )
                        }
                            .unwrap()(
                            0 as libc::c_int,
                            b"empty buffer\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        if *s.offset(1 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int
                            && (*args.offset(1 as libc::c_int as isize)).is_null()
                        {
                            if enc
                                == (if *s as libc::c_int == '.' as i32 {
                                    (*user).u_plop.enc
                                } else {
                                    plop_tab[*s as libc::c_uchar as libc::c_int as usize].enc
                                })
                            {
                                MakePaster(
                                    &mut (*fore).w_paster,
                                    if *s as libc::c_int == '.' as i32 {
                                        (*user).u_plop.buf
                                    } else {
                                        plop_tab[*s as libc::c_uchar as libc::c_int as usize].buf
                                    },
                                    l,
                                    0 as libc::c_int,
                                );
                                current_block_1498 = 188265764000799656;
                            } else {
                                current_block_1498 = 5375334875883628501;
                            }
                        } else {
                            current_block_1498 = 5375334875883628501;
                        }
                        match current_block_1498 {
                            188265764000799656 => {}
                            _ => {
                                dbuf = malloc(l as libc::c_ulong) as *mut libc::c_char;
                                if dbuf.is_null() {
                                    if (*act).quiet == 0 {
                                        Some(
                                            Msg
                                                as unsafe extern "C" fn(
                                                    libc::c_int,
                                                    *const libc::c_char,
                                                    ...
                                                ) -> (),
                                        )
                                    } else if queryflag >= 0 as libc::c_int {
                                        Some(
                                            QueryMsg
                                                as unsafe extern "C" fn(
                                                    libc::c_int,
                                                    *const libc::c_char,
                                                    ...
                                                ) -> (),
                                        )
                                    } else {
                                        Some(
                                            Dummy
                                                as unsafe extern "C" fn(
                                                    libc::c_int,
                                                    *const libc::c_char,
                                                    ...
                                                ) -> (),
                                        )
                                    }
                                        .unwrap()(
                                        0 as libc::c_int,
                                        b"%s\0" as *const u8 as *const libc::c_char,
                                        strnomem.as_mut_ptr(),
                                    );
                                } else {
                                    l = 0 as libc::c_int;
                                    ss = s;
                                    loop {
                                        ch = *ss;
                                        if !(ch != 0) {
                                            break;
                                        }
                                        let mut pp_0: *mut plop = if ch as libc::c_int == '.' as i32
                                        {
                                            &mut (*user).u_plop
                                        } else {
                                            &mut *plop_tab
                                                .as_mut_ptr()
                                                .offset(ch as libc::c_uchar as libc::c_int as isize)
                                                as *mut plop
                                        };
                                        if (*pp_0).enc != enc {
                                            l
                                                += RecodeBuf(
                                                    (*pp_0).buf as *mut libc::c_uchar,
                                                    (*pp_0).len,
                                                    (*pp_0).enc,
                                                    enc,
                                                    (dbuf as *mut libc::c_uchar).offset(l as isize),
                                                );
                                        } else {
                                            bcopy(
                                                (*pp_0).buf as *const libc::c_void,
                                                dbuf.offset(l as isize) as *mut libc::c_void,
                                                (*pp_0).len as size_t,
                                            );
                                            l += (*pp_0).len;
                                        }
                                        ss = ss.offset(1);
                                        ss;
                                    }
                                    if (*args.offset(1 as libc::c_int as isize)).is_null() {
                                        MakePaster(
                                            &mut (*fore).w_paster,
                                            dbuf,
                                            l,
                                            1 as libc::c_int,
                                        );
                                    } else {
                                        dch = *(*args.offset(1 as libc::c_int as isize))
                                            .offset(0 as libc::c_int as isize);
                                        if dch as libc::c_int == '.' as i32 {
                                            if !((*user).u_plop.buf).is_null() {
                                                UserFreeCopyBuffer(user);
                                            }
                                            (*user).u_plop.buf = dbuf;
                                            (*user).u_plop.len = l;
                                            (*user).u_plop.enc = enc;
                                        } else {
                                            let mut pp_1: *mut plop = plop_tab
                                                .as_mut_ptr()
                                                .offset(dch as libc::c_uchar as libc::c_int as isize);
                                            if !((*pp_1).buf).is_null() {
                                                free((*pp_1).buf as *mut libc::c_void);
                                            }
                                            (*pp_1).buf = dbuf;
                                            (*pp_1).len = l;
                                            (*pp_1).enc = enc;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        4564056263373890281 => {
            if !(*args).is_null()
                && strcmp(*args, b"-h\0" as *const u8 as *const libc::c_char) == 0
            {
                Hangup();
            } else {
                Detach(0 as libc::c_int);
            }
        }
        9234767613549682605 => {
            if MoreWindows() != 0 {
                SwitchWindow(
                    if !display.is_null() && !((*display).d_other).is_null() {
                        (*(*display).d_other).w_number
                    } else {
                        NextWindow()
                    },
                );
            }
        }
        _ => {}
    }
    if display != odisplay {
        display = displays;
        while !display.is_null() {
            if display == odisplay {
                break;
            }
            display = (*display).d_next;
        }
    }
}
unsafe extern "C" fn CollapseWindowlist() {
    let mut pos: libc::c_int = 0;
    let mut moveto: libc::c_int = 0 as libc::c_int;
    pos = 1 as libc::c_int;
    while pos < 100 as libc::c_int {
        if !(*wtab.offset(pos as isize)).is_null() {
            while moveto < pos {
                if (*wtab.offset(moveto as isize)).is_null() {
                    WindowChangeNumber(pos, moveto);
                    break;
                } else {
                    moveto += 1;
                    moveto;
                }
            }
        }
        pos += 1;
        pos;
    }
}
pub unsafe extern "C" fn DoCommand(
    mut argv: *mut *mut libc::c_char,
    mut argl: *mut libc::c_int,
) {
    let mut act: action = action {
        nr: 0,
        args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        argl: 0 as *const libc::c_int as *mut libc::c_int,
        quiet: 0,
    };
    let mut cmd: *const libc::c_char = *argv;
    act.quiet = 0 as libc::c_int;
    if *cmd as libc::c_int == '@' as i32 {
        act.quiet |= 0x1 as libc::c_int;
        cmd = cmd.offset(1);
        cmd;
    }
    if *cmd as libc::c_int == '-' as i32 {
        act.quiet |= 0x2 as libc::c_int;
        cmd = cmd.offset(1);
        cmd;
    }
    act.nr = FindCommnr(cmd);
    if act.nr == -(1 as libc::c_int) {
        Msg(
            0 as libc::c_int,
            b"%s: unknown command '%s'\0" as *const u8 as *const libc::c_char,
            rc_name,
            cmd,
        );
        return;
    }
    act.args = argv.offset(1 as libc::c_int as isize);
    act.argl = argl.offset(1 as libc::c_int as isize);
    DoAction(&mut act, -(1 as libc::c_int));
}
unsafe extern "C" fn SaveAction(
    mut act: *mut action,
    mut nr: libc::c_int,
    mut args: *mut *mut libc::c_char,
    mut argl: *mut libc::c_int,
) {
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lp: *mut libc::c_int = 0 as *mut libc::c_int;
    if !args.is_null() {
        while !(*args.offset(argc as isize)).is_null() {
            argc += 1;
            argc;
        }
    }
    if argc == 0 as libc::c_int {
        (*act).nr = nr;
        (*act).args = noargs.as_mut_ptr();
        (*act).argl = 0 as *mut libc::c_int;
        return;
    }
    pp = malloc(
        ((argc + 1 as libc::c_int) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if pp.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    }
    lp = malloc(
        (argc as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if lp.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    }
    (*act).nr = nr;
    (*act).args = pp;
    (*act).argl = lp;
    loop {
        let fresh19 = argc;
        argc = argc - 1;
        if !(fresh19 != 0) {
            break;
        }
        *lp = if !argl.is_null() {
            let fresh20 = argl;
            argl = argl.offset(1);
            *fresh20
        } else {
            strlen(*args) as libc::c_int
        };
        let fresh21 = args;
        args = args.offset(1);
        let fresh22 = lp;
        lp = lp.offset(1);
        let fresh23 = pp;
        pp = pp.offset(1);
        *fresh23 = SaveStrn(*fresh21, *fresh22);
    }
    *pp = 0 as *mut libc::c_char;
}
unsafe extern "C" fn SaveArgs(
    mut args: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argc: libc::c_int = 0 as libc::c_int;
    while !(*args.offset(argc as isize)).is_null() {
        argc += 1;
        argc;
    }
    ap = malloc(
        ((argc + 1 as libc::c_int) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    pp = ap;
    if pp.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    }
    loop {
        let fresh24 = argc;
        argc = argc - 1;
        if !(fresh24 != 0) {
            break;
        }
        let fresh25 = args;
        args = args.offset(1);
        let fresh26 = pp;
        pp = pp.offset(1);
        *fresh26 = SaveStr(*fresh25);
    }
    *pp = 0 as *mut libc::c_char;
    return ap;
}
pub unsafe extern "C" fn Parse(
    mut buf: *mut libc::c_char,
    mut bufl: libc::c_int,
    mut args: *mut *mut libc::c_char,
    mut argl: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = buf;
    let mut ap: *mut *mut libc::c_char = args;
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delim: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut lp: *mut libc::c_int = argl;
    argc = 0 as libc::c_int;
    pp = buf;
    delim = 0 as libc::c_int;
    loop {
        *lp = 0 as libc::c_int;
        while *p as libc::c_int != 0
            && (*p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32)
        {
            p = p.offset(1);
            p;
        }
        if argc == 0 as libc::c_int && *p as libc::c_int == '!' as i32 {
            let fresh27 = ap;
            ap = ap.offset(1);
            *fresh27 = b"exec\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let fresh28 = lp;
            lp = lp.offset(1);
            *fresh28 = 4 as libc::c_int;
            p = p.offset(1);
            p;
            argc += 1;
            argc;
        } else {
            if *p as libc::c_int == '\0' as i32 || *p as libc::c_int == '#' as i32
                || *p as libc::c_int == '\n' as i32
            {
                *p = '\0' as i32 as libc::c_char;
                delim = 0 as libc::c_int;
                while delim < argc {
                    delim += 1;
                    delim;
                }
                let ref mut fresh29 = *args.offset(argc as isize);
                *fresh29 = 0 as *mut libc::c_char;
                return argc;
            }
            argc += 1;
            if argc >= 64 as libc::c_int {
                Msg(
                    0 as libc::c_int,
                    b"%s: too many tokens.\0" as *const u8 as *const libc::c_char,
                    rc_name,
                );
                return 0 as libc::c_int;
            }
            let fresh30 = ap;
            ap = ap.offset(1);
            *fresh30 = pp;
            while *p != 0 {
                if *p as libc::c_int == delim {
                    delim = 0 as libc::c_int;
                } else if delim != '\'' as i32 && *p as libc::c_int == '\\' as i32
                    && (*p.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'r' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 't' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\'' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '"' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\\' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '$' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '#' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '^' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '7' as i32)
                {
                    p = p.offset(1);
                    p;
                    if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '7' as i32
                    {
                        *pp = (*p as libc::c_int - '0' as i32) as libc::c_char;
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '7' as i32
                        {
                            p = p.offset(1);
                            p;
                            *pp = ((*pp as libc::c_int) << 3 as libc::c_int
                                | *p as libc::c_int - '0' as i32) as libc::c_char;
                            if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                >= '0' as i32
                                && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    <= '7' as i32
                            {
                                p = p.offset(1);
                                p;
                                *pp = ((*pp as libc::c_int) << 3 as libc::c_int
                                    | *p as libc::c_int - '0' as i32) as libc::c_char;
                            }
                        }
                        pp = pp.offset(1);
                        pp;
                    } else {
                        match *p as libc::c_int {
                            110 => {
                                *pp = '\n' as i32 as libc::c_char;
                            }
                            114 => {
                                *pp = '\r' as i32 as libc::c_char;
                            }
                            116 => {
                                *pp = '\t' as i32 as libc::c_char;
                            }
                            _ => {
                                *pp = *p;
                            }
                        }
                        pp = pp.offset(1);
                        pp;
                    }
                } else if delim != '\'' as i32 && *p as libc::c_int == '$' as i32
                    && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == ':' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 'a' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                <= 'z' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 'A' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                <= 'Z' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '9' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '_' as i32)
                {
                    let mut ps: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut pe: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut op: libc::c_char = 0;
                    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut xbuf: [libc::c_char; 11] = [0; 11];
                    let mut path: [libc::c_char; 4096] = [0; 4096];
                    let mut vl: libc::c_int = 0;
                    p = p.offset(1);
                    ps = p;
                    p = p.offset(1);
                    p;
                    while *p != 0 {
                        if *ps as libc::c_int == '{' as i32
                            && *p as libc::c_int == '}' as i32
                        {
                            break;
                        }
                        if *ps as libc::c_int == ':' as i32
                            && *p as libc::c_int == ':' as i32
                        {
                            break;
                        }
                        if *ps as libc::c_int != '{' as i32
                            && *ps as libc::c_int != ':' as i32
                            && ((*p as libc::c_int) < 'a' as i32
                                || *p as libc::c_int > 'z' as i32)
                            && ((*p as libc::c_int) < 'A' as i32
                                || *p as libc::c_int > 'Z' as i32)
                            && ((*p as libc::c_int) < '0' as i32
                                || *p as libc::c_int > '9' as i32)
                            && *p as libc::c_int != '_' as i32
                        {
                            break;
                        }
                        p = p.offset(1);
                        p;
                    }
                    pe = p;
                    if *ps as libc::c_int == '{' as i32
                        || *ps as libc::c_int == ':' as i32
                    {
                        if *p == 0 {
                            Msg(
                                0 as libc::c_int,
                                b"%s: bad variable name.\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                            );
                            return 0 as libc::c_int;
                        }
                        p = p.offset(1);
                        p;
                    }
                    op = *pe;
                    *pe = 0 as libc::c_int as libc::c_char;
                    if *ps as libc::c_int == ':' as i32 {
                        v = gettermcapstring(ps.offset(1 as libc::c_int as isize));
                    } else {
                        if *ps as libc::c_int == '{' as i32 {
                            ps = ps.offset(1);
                            ps;
                        }
                        v = xbuf.as_mut_ptr();
                        if strcmp(ps, b"TERM\0" as *const u8 as *const libc::c_char) == 0
                        {
                            v = (if !display.is_null() {
                                ((*display).d_termname).as_mut_ptr() as *const libc::c_char
                            } else {
                                b"unknown\0" as *const u8 as *const libc::c_char
                            }) as *mut libc::c_char;
                        } else if strcmp(
                            ps,
                            b"COLUMNS\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            sprintf(
                                xbuf.as_mut_ptr(),
                                b"%d\0" as *const u8 as *const libc::c_char,
                                if !display.is_null() {
                                    (*display).d_width
                                } else {
                                    -(1 as libc::c_int)
                                },
                            );
                        } else if strcmp(
                            ps,
                            b"LINES\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            sprintf(
                                xbuf.as_mut_ptr(),
                                b"%d\0" as *const u8 as *const libc::c_char,
                                if !display.is_null() {
                                    (*display).d_height
                                } else {
                                    -(1 as libc::c_int)
                                },
                            );
                        } else if strcmp(
                            ps,
                            b"PID\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            sprintf(
                                xbuf.as_mut_ptr(),
                                b"%d\0" as *const u8 as *const libc::c_char,
                                getpid(),
                            );
                        } else if strcmp(
                            ps,
                            b"PWD\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            if (getcwd(
                                path.as_mut_ptr(),
                                (::std::mem::size_of::<[libc::c_char; 4096]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ))
                                .is_null()
                            {
                                v = b"?\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            } else {
                                v = path.as_mut_ptr();
                            }
                        } else if strcmp(
                            ps,
                            b"STY\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            v = strchr(SockName, '.' as i32);
                            if !v.is_null() {
                                v = v.offset(1);
                                v;
                            } else {
                                v = SockName;
                            }
                        } else {
                            v = getenv(ps);
                        }
                    }
                    *pe = op;
                    vl = (if !v.is_null() {
                        strlen(v)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as libc::c_int;
                    if vl != 0 {
                        if (p.offset_from(pp) as libc::c_long) < vl as libc::c_long {
                            let mut right: libc::c_int = buf
                                .offset(bufl as isize)
                                .offset_from(
                                    p
                                        .offset(strlen(p) as isize)
                                        .offset(1 as libc::c_int as isize),
                                ) as libc::c_long as libc::c_int;
                            if right > 0 as libc::c_int {
                                bcopy(
                                    p as *const libc::c_void,
                                    p.offset(right as isize) as *mut libc::c_void,
                                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                                );
                                p = p.offset(right as isize);
                            }
                        }
                        if (p.offset_from(pp) as libc::c_long) < vl as libc::c_long {
                            Msg(
                                0 as libc::c_int,
                                b"%s: no space left for variable expansion.\0" as *const u8
                                    as *const libc::c_char,
                                rc_name,
                            );
                            return 0 as libc::c_int;
                        }
                        bcopy(
                            v as *const libc::c_void,
                            pp as *mut libc::c_void,
                            vl as size_t,
                        );
                        pp = pp.offset(vl as isize);
                    }
                    continue;
                } else if delim != '\'' as i32 && *p as libc::c_int == '^' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
                {
                    p = p.offset(1);
                    p;
                    let fresh31 = pp;
                    pp = pp.offset(1);
                    *fresh31 = (if *p as libc::c_int == '?' as i32 {
                        '\u{7f}' as i32
                    } else {
                        *p as libc::c_int & 0x1f as libc::c_int
                    }) as libc::c_char;
                } else if delim == 0 as libc::c_int
                    && (*p as libc::c_int == '\'' as i32
                        || *p as libc::c_int == '"' as i32)
                {
                    delim = *p as libc::c_int;
                } else {
                    if delim == 0 as libc::c_int
                        && (*p as libc::c_int == ' ' as i32
                            || *p as libc::c_int == '\t' as i32
                            || *p as libc::c_int == '\n' as i32)
                    {
                        break;
                    }
                    let fresh32 = pp;
                    pp = pp.offset(1);
                    *fresh32 = *p;
                }
                p = p.offset(1);
                p;
            }
            if delim != 0 {
                Msg(
                    0 as libc::c_int,
                    b"%s: Missing %c quote.\0" as *const u8 as *const libc::c_char,
                    rc_name,
                    delim,
                );
                return 0 as libc::c_int;
            }
            if *p != 0 {
                p = p.offset(1);
                p;
            }
            *pp = 0 as libc::c_int as libc::c_char;
            let fresh33 = lp;
            lp = lp.offset(1);
            *fresh33 = pp.offset_from(*ap.offset(-(1 as libc::c_int) as isize))
                as libc::c_long as libc::c_int;
            pp = pp.offset(1);
            pp;
        }
    };
}
pub unsafe extern "C" fn SetEscape(
    mut u: *mut acluser,
    mut e: libc::c_int,
    mut me: libc::c_int,
) {
    if !u.is_null() {
        (*u).u_Esc = e;
        (*u).u_MetaEsc = me;
    } else {
        if !users.is_null() {
            if DefaultEsc >= 0 as libc::c_int {
                ClearAction(&mut *ktab.as_mut_ptr().offset(DefaultEsc as isize));
            }
            if DefaultMetaEsc >= 0 as libc::c_int {
                ClearAction(&mut *ktab.as_mut_ptr().offset(DefaultMetaEsc as isize));
            }
        }
        DefaultEsc = e;
        DefaultMetaEsc = me;
        if !users.is_null() {
            if DefaultEsc >= 0 as libc::c_int {
                ClearAction(&mut *ktab.as_mut_ptr().offset(DefaultEsc as isize));
                ktab[DefaultEsc as usize].nr = 122 as libc::c_int;
            }
            if DefaultMetaEsc >= 0 as libc::c_int {
                ClearAction(&mut *ktab.as_mut_ptr().offset(DefaultMetaEsc as isize));
                ktab[DefaultMetaEsc as usize].nr = 110 as libc::c_int;
            }
        }
    };
}
pub unsafe extern "C" fn ParseSwitch(
    mut act: *mut action,
    mut var: *mut libc::c_int,
) -> libc::c_int {
    if (*(*act).args).is_null() {
        *var ^= 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    return ParseOnOff(act, var);
}
unsafe extern "C" fn ParseOnOff(
    mut act: *mut action,
    mut var: *mut libc::c_int,
) -> libc::c_int {
    let mut num: libc::c_int = -(1 as libc::c_int);
    let mut args: *mut *mut libc::c_char = (*act).args;
    if (*args.offset(1 as libc::c_int as isize)).is_null() {
        if strcmp(
            *args.offset(0 as libc::c_int as isize),
            b"on\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            num = 1 as libc::c_int;
        } else if strcmp(
            *args.offset(0 as libc::c_int as isize),
            b"off\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            num = 0 as libc::c_int;
        }
    }
    if num < 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"%s: %s: invalid argument. Give 'on' or 'off'\0" as *const u8
                as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    *var = num;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ParseSaveStr(
    mut act: *mut action,
    mut var: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut args: *mut *mut libc::c_char = (*act).args;
    if (*args).is_null() || !(*args.offset(1 as libc::c_int as isize)).is_null() {
        Msg(
            0 as libc::c_int,
            b"%s: %s: one argument required.\0" as *const u8 as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    if !(*var).is_null() {
        free(*var as *mut libc::c_void);
    }
    *var = SaveStr(*args);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ParseNum(
    mut act: *mut action,
    mut var: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args: *mut *mut libc::c_char = (*act).args;
    p = *args;
    if p.is_null() || *p as libc::c_int == 0 as libc::c_int
        || !(*args.offset(1 as libc::c_int as isize)).is_null()
    {
        Msg(
            0 as libc::c_int,
            b"%s: %s: invalid argument. Give one argument.\0" as *const u8
                as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while *p != 0 {
        if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            i = 10 as libc::c_int * i + (*p as libc::c_int - '0' as i32);
        } else {
            Msg(
                0 as libc::c_int,
                b"%s: %s: invalid argument. Give numeric argument.\0" as *const u8
                    as *const libc::c_char,
                rc_name,
                (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
            );
            return -(1 as libc::c_int);
        }
        p = p.offset(1);
        p;
    }
    *var = i;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ParseNum1000(
    mut act: *mut action,
    mut var: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args: *mut *mut libc::c_char = (*act).args;
    let mut dig: libc::c_int = 0 as libc::c_int;
    p = *args;
    if p.is_null() || *p as libc::c_int == 0 as libc::c_int
        || !(*args.offset(1 as libc::c_int as isize)).is_null()
    {
        Msg(
            0 as libc::c_int,
            b"%s: %s: invalid argument. Give one argument.\0" as *const u8
                as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while *p != 0 {
        if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            if dig < 4 as libc::c_int {
                i = 10 as libc::c_int * i + (*p as libc::c_int - '0' as i32);
            } else if dig == 4 as libc::c_int && *p as libc::c_int >= '5' as i32 {
                i += 1;
                i;
            }
            if dig != 0 {
                dig += 1;
                dig;
            }
        } else if *p as libc::c_int == '.' as i32 && dig == 0 {
            dig += 1;
            dig;
        } else {
            Msg(
                0 as libc::c_int,
                b"%s: %s: invalid argument. Give floating point argument.\0" as *const u8
                    as *const libc::c_char,
                rc_name,
                (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
            );
            return -(1 as libc::c_int);
        }
        p = p.offset(1);
        p;
    }
    if dig == 0 as libc::c_int {
        i *= 1000 as libc::c_int;
    } else {
        loop {
            let fresh34 = dig;
            dig = dig + 1;
            if !(fresh34 < 4 as libc::c_int) {
                break;
            }
            i *= 10 as libc::c_int;
        }
    }
    if i < 0 as libc::c_int {
        i = (!(0 as libc::c_int) as libc::c_uint >> 1 as libc::c_int) as libc::c_int;
    }
    *var = i;
    return 0 as libc::c_int;
}
unsafe extern "C" fn WindowByName(mut s: *mut libc::c_char) -> *mut win {
    let mut p: *mut win = 0 as *mut win;
    p = windows;
    while !p.is_null() {
        if strcmp((*p).w_title, s) == 0 {
            return p;
        }
        p = (*p).w_next;
    }
    p = windows;
    while !p.is_null() {
        if strncmp((*p).w_title, s, strlen(s)) == 0 {
            return p;
        }
        p = (*p).w_next;
    }
    return 0 as *mut win;
}
unsafe extern "C" fn WindowByNumber(mut str: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    s = str;
    while *s != 0 {
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
            break;
        }
        i = i * 10 as libc::c_int + (*s as libc::c_int - '0' as i32);
        s = s.offset(1);
        s;
    }
    return if *s as libc::c_int != 0 { -(1 as libc::c_int) } else { i };
}
pub unsafe extern "C" fn WindowByNoN(mut str: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut win = 0 as *mut win;
    i = WindowByNumber(str);
    if i < 0 as libc::c_int || i >= maxwin {
        p = WindowByName(str);
        if !p.is_null() {
            return (*p).w_number;
        }
        return -(1 as libc::c_int);
    }
    return i;
}
unsafe extern "C" fn ParseWinNum(
    mut act: *mut action,
    mut var: *mut libc::c_int,
) -> libc::c_int {
    let mut args: *mut *mut libc::c_char = (*act).args;
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*args).is_null() || !(*args.offset(1 as libc::c_int as isize)).is_null() {
        Msg(
            0 as libc::c_int,
            b"%s: %s: one argument required.\0" as *const u8 as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    i = WindowByNoN(*args);
    if i < 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"%s: %s: invalid argument. Give window number or name.\0" as *const u8
                as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    *var = i;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ParseBase(
    mut act: *mut action,
    mut p: *mut libc::c_char,
    mut var: *mut libc::c_int,
    mut base: libc::c_int,
    mut bname: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    if *p as libc::c_int == 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"%s: %s: empty argument.\0" as *const u8 as *const libc::c_char,
            rc_name,
            (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
        );
        return -(1 as libc::c_int);
    }
    loop {
        let fresh35 = p;
        p = p.offset(1);
        c = *fresh35 as libc::c_int;
        if !(c != 0) {
            break;
        }
        if c >= 'a' as i32 && c <= 'z' as i32 {
            c -= 'a' as i32 - 'A' as i32;
        }
        if c >= 'A' as i32 && c <= 'Z' as i32 {
            c -= 'A' as i32 - ('0' as i32 + 10 as libc::c_int);
        }
        c -= '0' as i32;
        if c < 0 as libc::c_int || c >= base {
            Msg(
                0 as libc::c_int,
                b"%s: %s: argument is not %s.\0" as *const u8 as *const libc::c_char,
                rc_name,
                (*comms.as_mut_ptr().offset((*act).nr as isize)).name,
                bname,
            );
            return -(1 as libc::c_int);
        }
        i = base * i + c;
    }
    *var = i;
    return 0 as libc::c_int;
}
unsafe extern "C" fn IsNum(
    mut s: *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_int {
    base += '0' as i32;
    while *s != 0 {
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > base {
            return 0 as libc::c_int;
        }
        s = s.offset(1);
        s;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn IsNumColon(
    mut s: *mut libc::c_char,
    mut base: libc::c_int,
    mut p: *mut libc::c_char,
    mut psize: libc::c_int,
) -> libc::c_int {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    q = rindex(s, ':' as i32);
    if !q.is_null() {
        strncpy(
            p,
            q.offset(1 as libc::c_int as isize),
            (psize - 1 as libc::c_int) as libc::c_ulong,
        );
        *p.offset((psize - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        *q = '\0' as i32 as libc::c_char;
    } else {
        *p = '\0' as i32 as libc::c_char;
    }
    return IsNum(s, base);
}
pub unsafe extern "C" fn SwitchWindow(mut n: libc::c_int) {
    let mut p: *mut win = 0 as *mut win;
    if n < 0 as libc::c_int || n >= maxwin {
        ShowWindows(-(1 as libc::c_int));
        return;
    }
    p = *wtab.offset(n as isize);
    if p.is_null() {
        ShowWindows(n);
        return;
    }
    if display.is_null() {
        fore = p;
        return;
    }
    if p == (*display).d_fore {
        Msg(
            0 as libc::c_int,
            b"This IS window %d (%s).\0" as *const u8 as *const libc::c_char,
            n,
            (*p).w_title,
        );
        return;
    }
    if AclCheckPermWin((*display).d_user, 2 as libc::c_int, p) != 0 {
        Msg(
            0 as libc::c_int,
            b"Access to window %d denied.\0" as *const u8 as *const libc::c_char,
            (*p).w_number,
        );
        return;
    }
    SetForeWindow(p);
    Activate((*fore).w_norefresh as libc::c_int);
}
pub unsafe extern "C" fn SetForeWindow(mut wi: *mut win) {
    let mut p: *mut win = 0 as *mut win;
    if display.is_null() {
        fore = wi;
        return;
    }
    p = (*(*(*(*display).d_forecv).c_layer).l_bottom).l_data as *mut win;
    SetCanvasWindow((*display).d_forecv, wi);
    if !p.is_null() {
        WindowChanged(p, 'u' as i32);
    }
    if !wi.is_null() {
        WindowChanged(wi, 'u' as i32);
    }
    flayer = (*(*display).d_forecv).c_layer;
}
pub unsafe extern "C" fn Activate(mut norefresh: libc::c_int) {
    if display.is_null() {
        return;
    }
    if (*display).d_status != 0 {
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        RemoveStatus();
    }
    if MayResizeLayer((*(*display).d_forecv).c_layer) != 0 {
        ResizeLayer(
            (*(*display).d_forecv).c_layer,
            (*(*display).d_forecv).c_xe - (*(*display).d_forecv).c_xs + 1 as libc::c_int,
            (*(*display).d_forecv).c_ye - (*(*display).d_forecv).c_ys + 1 as libc::c_int,
            display,
        );
    }
    fore = (*display).d_fore;
    if !fore.is_null() {
        if (*fore).w_monitor != 0 as libc::c_int {
            (*fore).w_monitor = 1 as libc::c_int;
        }
        (*fore).w_bell = 0 as libc::c_int;
        WindowChanged(fore, 'f' as i32);
    }
    Redisplay(norefresh + all_norefresh);
}
unsafe extern "C" fn NextWindow() -> libc::c_int {
    let mut pp: *mut *mut win = 0 as *mut *mut win;
    let mut n: libc::c_int = if !fore.is_null() { (*fore).w_number } else { maxwin };
    let mut group: *mut win = if !fore.is_null() {
        (*fore).w_group
    } else {
        0 as *mut win
    };
    pp = if !fore.is_null() {
        wtab.offset(n as isize).offset(1 as libc::c_int as isize)
    } else {
        wtab
    };
    while pp != wtab.offset(n as isize) {
        if pp == wtab.offset(maxwin as isize) {
            pp = wtab;
        }
        if !(*pp).is_null() {
            if fore.is_null() || group == (**pp).w_group {
                break;
            }
        }
        pp = pp.offset(1);
        pp;
    }
    if pp == wtab.offset(n as isize) {
        return -(1 as libc::c_int);
    }
    return pp.offset_from(wtab) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn PreviousWindow() -> libc::c_int {
    let mut pp: *mut *mut win = 0 as *mut *mut win;
    let mut n: libc::c_int = if !fore.is_null() {
        (*fore).w_number
    } else {
        -(1 as libc::c_int)
    };
    let mut group: *mut win = if !fore.is_null() {
        (*fore).w_group
    } else {
        0 as *mut win
    };
    pp = wtab.offset(n as isize).offset(-(1 as libc::c_int as isize));
    while pp != wtab.offset(n as isize) {
        if pp == wtab.offset(-(1 as libc::c_int as isize)) {
            pp = wtab.offset(maxwin as isize).offset(-(1 as libc::c_int as isize));
        }
        if !(*pp).is_null() {
            if fore.is_null() || group == (**pp).w_group {
                break;
            }
        }
        pp = pp.offset(-1);
        pp;
    }
    if pp == wtab.offset(n as isize) {
        return -(1 as libc::c_int);
    }
    return pp.offset_from(wtab) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn MoreWindows() -> libc::c_int {
    let mut m: *mut libc::c_char = b"No other window.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    if !windows.is_null() && (fore.is_null() || !((*windows).w_next).is_null()) {
        return 1 as libc::c_int;
    }
    if fore.is_null() {
        Msg(
            0 as libc::c_int,
            b"No window available\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    Msg(0 as libc::c_int, m, (*fore).w_number);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn KillWindow(mut wi: *mut win) {
    let mut pp: *mut *mut win = 0 as *mut *mut win;
    let mut p: *mut win = 0 as *mut win;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut gotone: libc::c_int = 0;
    let mut lay: *mut layout = 0 as *mut layout;
    pp = &mut windows;
    loop {
        p = *pp;
        if p.is_null() {
            break;
        }
        if p == wi {
            break;
        }
        pp = &mut (*p).w_next;
    }
    *pp = (*p).w_next;
    (*wi).w_inlen = 0 as libc::c_int;
    let ref mut fresh36 = *wtab.offset((*wi).w_number as isize);
    *fresh36 = 0 as *mut win;
    if windows.is_null() {
        FreeWindow(wi);
        Finit(0 as libc::c_int);
    }
    display = displays;
    while !display.is_null() {
        gotone = 0 as libc::c_int;
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if !((*(*(*cv).c_layer).l_bottom).l_data as *mut win != wi) {
                SetCanvasWindow(
                    cv,
                    FindNiceWindow((*display).d_other, 0 as *mut libc::c_char),
                );
                gotone = 1 as libc::c_int;
            }
            cv = (*cv).c_next;
        }
        if gotone != 0 {
            if (*wi).w_zdisplay == display {
                (*display).d_blocked = 0 as libc::c_int;
                (*display).d_readev.condneg = 0 as *mut libc::c_int;
                (*display).d_readev.condpos = (*display).d_readev.condneg;
            }
            Activate(-(1 as libc::c_int));
        }
        display = (*display).d_next;
    }
    lay = layouts;
    while !lay.is_null() {
        UpdateLayoutCanvas(&mut (*lay).lay_canvas, wi);
        lay = (*lay).lay_next;
    }
    FreeWindow(wi);
    WindowChanged(0 as *mut win, 'w' as i32);
    WindowChanged(0 as *mut win, 'W' as i32);
    WindowChanged(0 as *mut win, 0 as libc::c_int);
}
unsafe extern "C" fn LogToggle(mut on: libc::c_int) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if ((*fore).w_log != 0 as *mut logfile) as libc::c_int == on {
        if !display.is_null() && *rc_name == 0 {
            Msg(
                0 as libc::c_int,
                b"You are %s logging.\0" as *const u8 as *const libc::c_char,
                if on != 0 {
                    b"already\0" as *const u8 as *const libc::c_char
                } else {
                    b"not\0" as *const u8 as *const libc::c_char
                },
            );
        }
        return;
    }
    if !((*fore).w_log).is_null() {
        Msg(
            0 as libc::c_int,
            b"Logfile \"%s\" closed.\0" as *const u8 as *const libc::c_char,
            (*(*fore).w_log).name,
        );
        logfclose((*fore).w_log);
        (*fore).w_log = 0 as *mut logfile;
        WindowChanged(fore, 'f' as i32);
        return;
    }
    if DoStartLog(
        fore,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    ) != 0
    {
        Msg(
            *__errno_location(),
            b"Error opening logfile \"%s\"\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        return;
    }
    if ftell((*(*fore).w_log).fp) == 0 as libc::c_int as libc::c_long {
        Msg(
            0 as libc::c_int,
            b"Creating logfile \"%s\".\0" as *const u8 as *const libc::c_char,
            (*(*fore).w_log).name,
        );
    } else {
        Msg(
            0 as libc::c_int,
            b"Appending to logfile \"%s\".\0" as *const u8 as *const libc::c_char,
            (*(*fore).w_log).name,
        );
    }
    WindowChanged(fore, 'f' as i32);
}
pub unsafe extern "C" fn AddWindows(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut flags: libc::c_int,
    mut where_0: libc::c_int,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut *mut win = 0 as *mut *mut win;
    let mut p: *mut win = 0 as *mut win;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    ss = buf;
    s = ss;
    if flags & 8 as libc::c_int != 0 && where_0 < 0 as libc::c_int {
        *s = 0 as libc::c_int as libc::c_char;
        return ss;
    }
    pp = if flags & 4 as libc::c_int != 0 && where_0 >= 0 as libc::c_int {
        wtab.offset(where_0 as isize).offset(1 as libc::c_int as isize)
    } else {
        wtab
    };
    while pp < wtab.offset(maxwin as isize) {
        let mut rend: libc::c_int = -(1 as libc::c_int);
        if pp.offset_from(wtab) as libc::c_long == where_0 as libc::c_long && ss == buf {
            ss = s;
        }
        p = *pp;
        if !p.is_null() {
            if !(flags & 1 as libc::c_int != 0 && !display.is_null()
                && p == (*display).d_fore)
            {
                if !(!display.is_null() && !((*display).d_fore).is_null()
                    && (*(*display).d_fore).w_group != (*p).w_group)
                {
                    cmd = (*p).w_title;
                    l = strlen(cmd) as libc::c_int;
                    if l > 20 as libc::c_int {
                        l = 20 as libc::c_int;
                    }
                    if s.offset_from(buf) as libc::c_long + l as libc::c_long
                        > (len - 24 as libc::c_int) as libc::c_long
                    {
                        break;
                    }
                    if s > buf || flags & 4 as libc::c_int != 0 {
                        let fresh37 = s;
                        s = s.offset(1);
                        *fresh37 = ' ' as i32 as libc::c_char;
                        let fresh38 = s;
                        s = s.offset(1);
                        *fresh38 = ' ' as i32 as libc::c_char;
                    }
                    if (*p).w_number == where_0 {
                        ss = s;
                        if flags & 8 as libc::c_int != 0 {
                            break;
                        }
                    }
                    if flags & 4 as libc::c_int == 0 || where_0 < 0 as libc::c_int
                        || flags & 4 as libc::c_int != 0 && where_0 < (*p).w_number
                    {
                        if (*p).w_monitor == 3 as libc::c_int
                            && *renditions
                                .as_mut_ptr()
                                .offset(REND_MONITOR as libc::c_int as isize)
                                != -(1 as libc::c_int)
                        {
                            rend = *renditions
                                .as_mut_ptr()
                                .offset(REND_MONITOR as libc::c_int as isize);
                        } else if ((*p).w_bell == 2 as libc::c_int
                            || (*p).w_bell == 1 as libc::c_int)
                            && *renditions
                                .as_mut_ptr()
                                .offset(REND_BELL as libc::c_int as isize)
                                != -(1 as libc::c_int)
                        {
                            rend = *renditions
                                .as_mut_ptr()
                                .offset(REND_BELL as libc::c_int as isize);
                        } else if ((*p).w_silence == 2 as libc::c_int
                            || (*p).w_silence == 3 as libc::c_int)
                            && *renditions
                                .as_mut_ptr()
                                .offset(REND_SILENCE as libc::c_int as isize)
                                != -(1 as libc::c_int)
                        {
                            rend = *renditions
                                .as_mut_ptr()
                                .offset(REND_SILENCE as libc::c_int as isize);
                        }
                    }
                    if rend != -(1 as libc::c_int) {
                        AddWinMsgRend(s, rend);
                    }
                    sprintf(
                        s,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        (*p).w_number,
                    );
                    s = s.offset(strlen(s) as isize);
                    if !display.is_null() && p == (*display).d_fore {
                        let fresh39 = s;
                        s = s.offset(1);
                        *fresh39 = '*' as i32 as libc::c_char;
                    }
                    if flags & 2 as libc::c_int == 0 {
                        if !display.is_null() && p == (*display).d_other {
                            let fresh40 = s;
                            s = s.offset(1);
                            *fresh40 = '-' as i32 as libc::c_char;
                        }
                        s = AddWindowFlags(s, len, p);
                    }
                    let fresh41 = s;
                    s = s.offset(1);
                    *fresh41 = ' ' as i32 as libc::c_char;
                    strncpy(s, cmd, l as libc::c_ulong);
                    s = s.offset(l as isize);
                    if rend != -(1 as libc::c_int) {
                        AddWinMsgRend(s, -(1 as libc::c_int));
                    }
                }
            }
        }
        pp = pp.offset(1);
        pp;
    }
    *s = 0 as libc::c_int as libc::c_char;
    return ss;
}
pub unsafe extern "C" fn AddWindowFlags(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut p: *mut win,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = buf;
    if p.is_null() || len < 12 as libc::c_int {
        *s = 0 as libc::c_int as libc::c_char;
        return s;
    }
    if !((*p).w_layer.l_cvlist).is_null()
        && !((*(*p).w_layer.l_cvlist).c_lnext).is_null()
    {
        let fresh42 = s;
        s = s.offset(1);
        *fresh42 = '&' as i32 as libc::c_char;
    }
    if (*p).w_monitor == 3 as libc::c_int && !display.is_null()
        && *((*p).w_mon_notify)
            .offset(((*(*display).d_user).u_id >> 3 as libc::c_int) as isize)
            as libc::c_int
            & 0x80 as libc::c_int >> ((*(*display).d_user).u_id & 7 as libc::c_int) != 0
    {
        let fresh43 = s;
        s = s.offset(1);
        *fresh43 = '@' as i32 as libc::c_char;
    }
    if (*p).w_bell == 2 as libc::c_int {
        let fresh44 = s;
        s = s.offset(1);
        *fresh44 = '!' as i32 as libc::c_char;
    }
    if !((*p).w_slot).is_null() && (*p).w_slot != -(1 as libc::c_int) as slot_t {
        let fresh45 = s;
        s = s.offset(1);
        *fresh45 = '$' as i32 as libc::c_char;
    }
    if !((*p).w_log).is_null() {
        strcpy(s, b"(L)\0" as *const u8 as *const libc::c_char);
        s = s.offset(3 as libc::c_int as isize);
    }
    if (*p).w_ptyfd < 0 as libc::c_int && (*p).w_type != 3 as libc::c_int {
        let fresh46 = s;
        s = s.offset(1);
        *fresh46 = 'Z' as i32 as libc::c_char;
    }
    *s = 0 as libc::c_int as libc::c_char;
    return s;
}
pub unsafe extern "C" fn AddOtherUsers(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut p: *mut win,
) -> *mut libc::c_char {
    let mut d: *mut display = 0 as *mut display;
    let mut olddisplay: *mut display = display;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    s = buf;
    display = displays;
    while !display.is_null() {
        if !(!olddisplay.is_null() && (*display).d_user == (*olddisplay).d_user) {
            cv = (*display).d_cvlist;
            while !cv.is_null() {
                if (*(*(*cv).c_layer).l_bottom).l_data as *mut win == p {
                    break;
                }
                cv = (*cv).c_next;
            }
            if !cv.is_null() {
                d = displays;
                while !d.is_null() && d != display {
                    if (*display).d_user == (*d).d_user {
                        break;
                    }
                    d = (*d).d_next;
                }
                if !(!d.is_null() && d != display) {
                    if len > 1 as libc::c_int && s != buf {
                        let fresh47 = s;
                        s = s.offset(1);
                        *fresh47 = ',' as i32 as libc::c_char;
                        len -= 1;
                        len;
                    }
                    l = strlen(((*(*display).d_user).u_name).as_mut_ptr())
                        as libc::c_int;
                    if l + 1 as libc::c_int > len {
                        break;
                    }
                    strcpy(s, ((*(*display).d_user).u_name).as_mut_ptr());
                    s = s.offset(l as isize);
                    len -= l;
                }
            }
        }
        display = (*display).d_next;
    }
    *s = 0 as libc::c_int as libc::c_char;
    display = olddisplay;
    return s;
}
pub unsafe extern "C" fn ShowWindows(mut where_0: libc::c_int) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    if !display.is_null() && where_0 == -(1 as libc::c_int)
        && !((*display).d_fore).is_null()
    {
        where_0 = (*(*display).d_fore).w_number;
    }
    ss = AddWindows(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        0 as libc::c_int,
        where_0,
    );
    s = buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
    if !display.is_null()
        && ss.offset_from(buf.as_mut_ptr()) as libc::c_long
            > ((*display).d_width / 2 as libc::c_int) as libc::c_long
    {
        ss = ss.offset(-(((*display).d_width / 2 as libc::c_int) as isize));
        if (s.offset_from(ss) as libc::c_long) < (*display).d_width as libc::c_long {
            ss = s.offset(-((*display).d_width as isize));
            if ss < buf.as_mut_ptr() {
                ss = buf.as_mut_ptr();
            }
        }
    } else {
        ss = buf.as_mut_ptr();
    }
    Msg(0 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, ss);
}
unsafe extern "C" fn ShowWindowsX(mut str: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < maxwin {
        if !(*wtab.offset(i as isize)).is_null() {
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                MakeWinMsg(str, *wtab.offset(i as isize), '%' as i32),
            );
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn ShowInfo() {
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wp: *mut win = fore;
    let mut i: libc::c_int = 0;
    if wp.is_null() {
        Msg(
            0 as libc::c_int,
            b"(%d,%d)/(%d,%d) no window\0" as *const u8 as *const libc::c_char,
            (*display).d_x + 1 as libc::c_int,
            (*display).d_y + 1 as libc::c_int,
            (*display).d_width,
            (*display).d_height,
        );
        return;
    }
    p = buf.as_mut_ptr();
    p = p.offset(GetAnsiStatus(wp, p) as isize);
    if buf.as_mut_ptr() < p {
        let fresh48 = p;
        p = p.offset(1);
        *fresh48 = ' ' as i32 as libc::c_char;
    }
    sprintf(
        p,
        b"(%d,%d)/(%d,%d)\0" as *const u8 as *const libc::c_char,
        (*wp).w_layer.l_x + 1 as libc::c_int,
        (*wp).w_layer.l_y + 1 as libc::c_int,
        (*wp).w_layer.l_width,
        (*wp).w_layer.l_height,
    );
    p = p.offset(strlen(p) as isize);
    sprintf(p, b"+%d\0" as *const u8 as *const libc::c_char, (*wp).w_histheight);
    p = p.offset(strlen(p) as isize);
    sprintf(
        p,
        b" %c%sflow\0" as *const u8 as *const libc::c_char,
        if (*wp).w_flow & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            '+' as i32
        } else {
            '-' as i32
        },
        if (*wp).w_flow & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else if (*wp).w_flow & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            b"(+)\0" as *const u8 as *const libc::c_char
        } else {
            b"(-)\0" as *const u8 as *const libc::c_char
        },
    );
    if (*wp).w_wrap == 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" -wrap\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_insert != 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" ins\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_origin != 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" org\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_keypad != 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" app\0" as *const u8 as *const libc::c_char);
    }
    if !((*wp).w_log).is_null() {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" log\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_monitor != 0 as libc::c_int
        && *((*wp).w_mon_notify)
            .offset(((*(*display).d_user).u_id >> 3 as libc::c_int) as isize)
            as libc::c_int
            & 0x80 as libc::c_int >> ((*(*display).d_user).u_id & 7 as libc::c_int) != 0
    {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" mon\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_mouse != 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" mouse\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_bce != 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" bce\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_c1 == 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" -c1\0" as *const u8 as *const libc::c_char);
    }
    if (*wp).w_norefresh != 0 {
        p = p.offset(strlen(p) as isize);
        sprintf(p, b" nored\0" as *const u8 as *const libc::c_char);
    }
    p = p.offset(strlen(p) as isize);
    if (*wp).w_layer.l_encoding != 0
        && (display.is_null() || (*display).d_encoding != (*wp).w_layer.l_encoding
            || EncodingDefFont((*wp).w_layer.l_encoding) <= 0 as libc::c_int)
    {
        let fresh49 = p;
        p = p.offset(1);
        *fresh49 = ' ' as i32 as libc::c_char;
        strcpy(p, EncodingName((*wp).w_layer.l_encoding));
        p = p.offset(strlen(p) as isize);
    }
    if (*wp).w_layer.l_encoding != 8 as libc::c_int {
        if !display.is_null()
            && (!((*display).d_tcs[100 as libc::c_int as usize].str_0).is_null()
                || !((*display).d_tcs[98 as libc::c_int as usize].str_0).is_null()
                    && *(*display).d_tcs[98 as libc::c_int as usize].str_0 as libc::c_int
                        != 0)
        {
            if (*wp).w_gr == 2 as libc::c_int {
                sprintf(
                    p,
                    b" G%c\0" as *const u8 as *const libc::c_char,
                    (*wp).w_Charset + '0' as i32,
                );
                if (*wp).w_FontE as libc::c_int >= ' ' as i32 {
                    *p.offset(3 as libc::c_int as isize) = (*wp).w_FontE;
                } else {
                    *p.offset(3 as libc::c_int as isize) = '^' as i32 as libc::c_char;
                    *p
                        .offset(
                            4 as libc::c_int as isize,
                        ) = ((*wp).w_FontE as libc::c_int ^ 0x40 as libc::c_int)
                        as libc::c_char;
                    p = p.offset(1);
                    p;
                }
                *p.offset(4 as libc::c_int as isize) = '[' as i32 as libc::c_char;
                p = p.offset(1);
                p;
            } else if (*wp).w_gr != 0 {
                let fresh50 = p;
                p = p.offset(1);
                sprintf(
                    fresh50,
                    b" G%c%c[\0" as *const u8 as *const libc::c_char,
                    (*wp).w_Charset + '0' as i32,
                    (*wp).w_CharsetR + '0' as i32,
                );
            } else {
                sprintf(
                    p,
                    b" G%c[\0" as *const u8 as *const libc::c_char,
                    (*wp).w_Charset + '0' as i32,
                );
            }
            p = p.offset(4 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                if (*wp).w_charsets[i as usize] == 0 as libc::c_int {
                    let fresh51 = p;
                    p = p.offset(1);
                    *fresh51 = 'B' as i32 as libc::c_char;
                } else if (*wp).w_charsets[i as usize] >= ' ' as i32 {
                    let fresh52 = p;
                    p = p.offset(1);
                    *fresh52 = (*wp).w_charsets[i as usize] as libc::c_char;
                } else {
                    let fresh53 = p;
                    p = p.offset(1);
                    *fresh53 = '^' as i32 as libc::c_char;
                    let fresh54 = p;
                    p = p.offset(1);
                    *fresh54 = ((*wp).w_charsets[i as usize] ^ 0x40 as libc::c_int)
                        as libc::c_char;
                }
                i += 1;
                i;
            }
            let fresh55 = p;
            p = p.offset(1);
            *fresh55 = ']' as i32 as libc::c_char;
            *p = 0 as libc::c_int as libc::c_char;
        }
    }
    if (*wp).w_type == 1 as libc::c_int {
        let fresh56 = p;
        p = p.offset(1);
        *fresh56 = ' ' as i32 as libc::c_char;
        TtyGetModemStatus((*wp).w_ptyfd, p);
    }
    Msg(
        0 as libc::c_int,
        b"%s %d(%s)\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        (*wp).w_number,
        (*wp).w_title,
    );
}
unsafe extern "C" fn ShowDInfo() {
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if display.is_null() {
        return;
    }
    p = buf.as_mut_ptr();
    sprintf(
        p,
        b"(%d,%d)\0" as *const u8 as *const libc::c_char,
        (*display).d_width,
        (*display).d_height,
    );
    p = p.offset(strlen(p) as isize);
    if (*display).d_encoding != 0 {
        let fresh57 = p;
        p = p.offset(1);
        *fresh57 = ' ' as i32 as libc::c_char;
        strcpy(p, EncodingName((*display).d_encoding));
        p = p.offset(strlen(p) as isize);
    }
    if (*display).d_tcs[96 as libc::c_int as usize].flg != 0 {
        strcpy(p, b" xterm\0" as *const u8 as *const libc::c_char);
        p = p.offset(strlen(p) as isize);
    }
    if (*display).d_hascolor != 0 {
        strcpy(p, b" color\0" as *const u8 as *const libc::c_char);
        p = p.offset(strlen(p) as isize);
    }
    if (*display).d_tcs[97 as libc::c_int as usize].flg != 0 {
        strcpy(p, b" iso2022\0" as *const u8 as *const libc::c_char);
        p = p.offset(strlen(p) as isize);
    } else if !((*display).d_tcs[98 as libc::c_int as usize].str_0).is_null()
        && *(*display).d_tcs[98 as libc::c_int as usize].str_0 as libc::c_int != 0
    {
        strcpy(p, b" altchar\0" as *const u8 as *const libc::c_char);
        p = p.offset(strlen(p) as isize);
    }
    Msg(0 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
}
unsafe extern "C" fn AKAfin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    if len != 0 && !fore.is_null() {
        ChangeAKA(fore, buf, strlen(buf) as libc::c_int);
    }
    enter_window_name_mode = 0 as libc::c_int;
}
unsafe extern "C" fn InputAKA() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if enter_window_name_mode == 1 as libc::c_int {
        return;
    }
    enter_window_name_mode = 1 as libc::c_int;
    Input(
        b"Set window's title to: \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut libc::c_char,
                ) -> (),
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                    unsafe extern "C" fn() -> (),
                >(AKAfin),
            ),
        ),
        0 as *mut libc::c_char,
        0 as libc::c_int,
    );
    s = (*fore).w_title;
    if s.is_null() {
        return;
    }
    while *s != 0 {
        if !((*(s as *mut libc::c_uchar) as libc::c_int & 0x7f as libc::c_int)
            < 0x20 as libc::c_int || *s as libc::c_int == 0x7f as libc::c_int)
        {
            ss = s;
            n = 1 as libc::c_int;
            (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                .unwrap()(&mut ss, &mut n);
        }
        s = s.offset(1);
        s;
    }
}
unsafe extern "C" fn Colonfin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut mbuf: [libc::c_char; 256] = [0; 256];
    RemoveStatus();
    if *buf.offset(len as isize) as libc::c_int == '\t' as i32 {
        let mut m: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        let mut l: libc::c_int = 0 as libc::c_int;
        let mut r: libc::c_int = 189 as libc::c_int;
        let mut showmessage: libc::c_int = 0 as libc::c_int;
        let mut s: *mut libc::c_char = buf;
        while *s as libc::c_int != 0
            && (s.offset_from(buf) as libc::c_long) < len as libc::c_long
        {
            let fresh58 = s;
            s = s.offset(1);
            if *fresh58 as libc::c_int == ' ' as i32 {
                return;
            }
        }
        if !display.is_null()
            && (captionalways != 0 || (*display).d_has_hstatus == 1 as libc::c_int
                || !((*display).d_canvas.c_slperp).is_null()
                    && !((*(*display).d_canvas.c_slperp).c_slnext).is_null())
        {
            showmessage = 1 as libc::c_int;
        }
        while l <= r {
            m = (l + r) / 2 as libc::c_int;
            x = strncmp(
                buf,
                (*comms.as_mut_ptr().offset(m as isize)).name,
                len as libc::c_ulong,
            );
            if x > 0 as libc::c_int {
                l = m + 1 as libc::c_int;
            } else if x < 0 as libc::c_int {
                r = m - 1 as libc::c_int;
            } else {
                s = mbuf.as_mut_ptr();
                l = m - 1 as libc::c_int;
                while l >= 0 as libc::c_int
                    && strncmp(
                        buf,
                        (*comms.as_mut_ptr().offset(l as isize)).name,
                        len as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    l -= 1;
                    l;
                }
                l += 1;
                m = l;
                while m <= r
                    && strncmp(
                        buf,
                        (*comms.as_mut_ptr().offset(m as isize)).name,
                        len as libc::c_ulong,
                    ) == 0 as libc::c_int
                    && (s.offset_from(mbuf.as_mut_ptr()) as libc::c_long
                        as libc::c_ulong)
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                {
                    s = s
                        .offset(
                            snprintf(
                                s,
                                (::std::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(
                                        s.offset_from(mbuf.as_mut_ptr()) as libc::c_long
                                            as libc::c_ulong,
                                    ),
                                b" %s\0" as *const u8 as *const libc::c_char,
                                (*comms.as_mut_ptr().offset(m as isize)).name,
                            ) as isize,
                        );
                    m += 1;
                    m;
                }
                if l < m - 1 as libc::c_int {
                    if showmessage != 0 {
                        Msg(
                            0 as libc::c_int,
                            b"Possible commands:%s\0" as *const u8
                                as *const libc::c_char,
                            mbuf.as_mut_ptr(),
                        );
                    }
                } else {
                    s = mbuf.as_mut_ptr();
                    len = snprintf(
                        mbuf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        b"%s \t\0" as *const u8 as *const libc::c_char,
                        ((*comms.as_mut_ptr().offset(l as isize)).name)
                            .offset(len as isize),
                    );
                    if len > 0 as libc::c_int
                        && (len as libc::c_ulong)
                            < ::std::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong
                    {
                        (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                            .unwrap()(&mut s, &mut len);
                    }
                }
                break;
            }
        }
        if l > r && showmessage != 0 {
            Msg(
                0 as libc::c_int,
                b"No commands matching '%*s'\0" as *const u8 as *const libc::c_char,
                len,
                buf,
            );
        }
        return;
    }
    if len == 0 || *buf.offset(len as isize) as libc::c_int != 0 {
        return;
    }
    len = (strlen(buf)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if len > ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int
    {
        RcLine(buf, len);
    } else {
        bcopy(
            buf as *const libc::c_void,
            mbuf.as_mut_ptr() as *mut libc::c_void,
            len as size_t,
        );
        RcLine(
            mbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
    };
}
unsafe extern "C" fn SelectFin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut n: libc::c_int = 0;
    if len == 0 || display.is_null() {
        return;
    }
    if len == 1 as libc::c_int && *buf as libc::c_int == '-' as i32 {
        SetForeWindow(0 as *mut win);
        Activate(0 as libc::c_int);
        return;
    }
    n = WindowByNoN(buf);
    if n < 0 as libc::c_int {
        return;
    }
    SwitchWindow(n);
}
unsafe extern "C" fn SelectLayoutFin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut lay: *mut layout = 0 as *mut layout;
    if len == 0 || display.is_null() {
        return;
    }
    if len == 1 as libc::c_int && *buf as libc::c_int == '-' as i32 {
        LoadLayout(0 as *mut layout, 0 as *mut canvas);
        Activate(0 as libc::c_int);
        return;
    }
    lay = FindLayout(buf);
    if lay.is_null() {
        Msg(0 as libc::c_int, b"No such layout\n\0" as *const u8 as *const libc::c_char);
    } else if lay == (*display).d_layout {
        Msg(
            0 as libc::c_int,
            b"This IS layout %d (%s).\n\0" as *const u8 as *const libc::c_char,
            (*lay).lay_number,
            (*lay).lay_title,
        );
    } else {
        LoadLayout(lay, &mut (*display).d_canvas);
        Activate(0 as libc::c_int);
    };
}
unsafe extern "C" fn InputSelect() {
    Input(
        b"Switch to window: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        20 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut libc::c_char,
                ) -> (),
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                    unsafe extern "C" fn() -> (),
                >(SelectFin),
            ),
        ),
        0 as *mut libc::c_char,
        0 as libc::c_int,
    );
}
static mut setenv_var: [libc::c_char; 31] = [0; 31];
unsafe extern "C" fn SetenvFin1(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    if len == 0 || display.is_null() {
        return;
    }
    InputSetenv(buf);
}
unsafe extern "C" fn SetenvFin2(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    if len == 0 || display.is_null() {
        return;
    }
    xsetenv(setenv_var.as_mut_ptr(), buf);
    MakeNewEnv();
}
unsafe extern "C" fn InputSetenv(mut arg: *mut libc::c_char) {
    static mut setenv_buf: [libc::c_char; 81] = [0; 81];
    if !arg.is_null() {
        strncpy(
            setenv_var.as_mut_ptr(),
            arg,
            (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        sprintf(
            setenv_buf.as_mut_ptr(),
            b"Enter value for %s: \0" as *const u8 as *const libc::c_char,
            setenv_var.as_mut_ptr(),
        );
        Input(
            setenv_buf.as_mut_ptr(),
            30 as libc::c_int,
            0 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(SetenvFin2),
                ),
            ),
            0 as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else {
        Input(
            b"Setenv: Enter variable name: \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            30 as libc::c_int,
            0 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(SetenvFin1),
                ),
            ),
            0 as *mut libc::c_char,
            0 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn DoScreen(
    mut fn_0: *mut libc::c_char,
    mut av: *mut *mut libc::c_char,
) {
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
    let mut num: libc::c_int = 0;
    let mut buf: [libc::c_char; 20] = [0; 20];
    nwin = nwin_undef;
    while !av.is_null() && !(*av).is_null()
        && *(*av.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        if *(*av.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        {
            av = av.offset(1);
            av;
            break;
        } else {
            match *(*av.offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize) as libc::c_int
            {
                102 => {
                    match *(*av.offset(0 as libc::c_int as isize))
                        .offset(2 as libc::c_int as isize) as libc::c_int
                    {
                        110 | 48 => {
                            nwin
                                .flowflag = ((1 as libc::c_int) << 0 as libc::c_int)
                                * 0 as libc::c_int;
                        }
                        121 | 49 | 0 => {
                            nwin
                                .flowflag = ((1 as libc::c_int) << 0 as libc::c_int)
                                * 1 as libc::c_int;
                        }
                        97 => {
                            nwin.flowflag = (1 as libc::c_int) << 2 as libc::c_int;
                        }
                        _ => {}
                    }
                }
                116 => {
                    if *(*av.offset(0 as libc::c_int as isize))
                        .offset(2 as libc::c_int as isize) != 0
                    {
                        nwin
                            .aka = &mut *(*av.offset(0 as libc::c_int as isize))
                            .offset(2 as libc::c_int as isize) as *mut libc::c_char;
                    } else {
                        av = av.offset(1);
                        if !(*av).is_null() {
                            nwin.aka = *av;
                        } else {
                            av = av.offset(-1);
                            av;
                        }
                    }
                }
                84 => {
                    if *(*av.offset(0 as libc::c_int as isize))
                        .offset(2 as libc::c_int as isize) != 0
                    {
                        nwin
                            .term = &mut *(*av.offset(0 as libc::c_int as isize))
                            .offset(2 as libc::c_int as isize) as *mut libc::c_char;
                    } else {
                        av = av.offset(1);
                        if !(*av).is_null() {
                            nwin.term = *av;
                        } else {
                            av = av.offset(-1);
                            av;
                        }
                    }
                }
                104 => {
                    if *(*av.offset(0 as libc::c_int as isize))
                        .offset(2 as libc::c_int as isize) != 0
                    {
                        nwin
                            .histheight = atoi(
                            (*av.offset(0 as libc::c_int as isize))
                                .offset(2 as libc::c_int as isize),
                        );
                    } else {
                        av = av.offset(1);
                        if !(*av).is_null() {
                            nwin.histheight = atoi(*av);
                        } else {
                            av = av.offset(-1);
                            av;
                        }
                    }
                }
                108 => {
                    match *(*av.offset(0 as libc::c_int as isize))
                        .offset(2 as libc::c_int as isize) as libc::c_int
                    {
                        110 | 48 => {
                            nwin.lflag = 0 as libc::c_int;
                        }
                        121 | 49 | 0 => {
                            nwin.lflag = 1 as libc::c_int;
                        }
                        97 => {
                            nwin.lflag = 3 as libc::c_int;
                        }
                        _ => {}
                    }
                }
                97 => {
                    nwin.aflag = 1 as libc::c_int;
                }
                77 => {
                    nwin.monitor = 1 as libc::c_int;
                }
                76 => {
                    nwin.Lflag = 1 as libc::c_int;
                }
                _ => {
                    Msg(
                        0 as libc::c_int,
                        b"%s: screen: invalid option -%c.\0" as *const u8
                            as *const libc::c_char,
                        fn_0,
                        *(*av.offset(0 as libc::c_int as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int,
                    );
                }
            }
            av = av.offset(1);
            av;
        }
    }
    if !av.is_null() && !(*av).is_null()
        && IsNumColon(
            *av,
            10 as libc::c_int,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_int,
        ) != 0
    {
        if *buf.as_mut_ptr() as libc::c_int != '\0' as i32 {
            nwin.aka = buf.as_mut_ptr();
        }
        num = atoi(*av);
        if num < 0 as libc::c_int || maxwin != 0 && num > maxwin - 1 as libc::c_int
            || maxwin == 0 && num > 100 as libc::c_int - 1 as libc::c_int
        {
            Msg(
                0 as libc::c_int,
                b"%s: illegal screen number %d.\0" as *const u8 as *const libc::c_char,
                fn_0,
                num,
            );
            num = 0 as libc::c_int;
        }
        nwin.StartAt = num;
        av = av.offset(1);
        av;
    }
    if !av.is_null() && !(*av).is_null() {
        nwin.args = av;
        if (nwin.aka).is_null() {
            nwin.aka = Filename(*av);
        }
    }
    MakeWindow(&mut nwin);
}
pub unsafe extern "C" fn CompileKeys(
    mut s: *mut libc::c_char,
    mut sl: libc::c_int,
    mut array: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut key: libc::c_uchar = 0;
    let mut value: libc::c_uchar = 0;
    if sl == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *array.offset(i as isize) = i as libc::c_uchar;
            i += 1;
            i;
        }
        return 0 as libc::c_int;
    }
    while sl != 0 {
        let fresh59 = s;
        s = s.offset(1);
        key = *(fresh59 as *mut libc::c_uchar);
        if *s as libc::c_int != '=' as i32 || sl < 3 as libc::c_int {
            return -(1 as libc::c_int);
        }
        sl -= 1;
        sl;
        loop {
            s = s.offset(1);
            s;
            sl -= 2 as libc::c_int;
            let fresh60 = s;
            s = s.offset(1);
            value = *(fresh60 as *mut libc::c_uchar);
            *array.offset(value as isize) = key;
            if !(*s as libc::c_int == '=' as i32 && sl >= 2 as libc::c_int) {
                break;
            }
        }
        if sl == 0 as libc::c_int {
            break;
        }
        let fresh61 = s;
        s = s.offset(1);
        if *fresh61 as libc::c_int != ':' as i32 {
            return -(1 as libc::c_int);
        }
        sl -= 1;
        sl;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pow_detach_fn(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    if len != 0 {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        return;
    }
    if ktab[*buf as libc::c_uchar as libc::c_int as usize].nr != 128 as libc::c_int {
        if !display.is_null() {
            write(
                (*display).d_userfd,
                b"\x07\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        Msg(0 as libc::c_int, b"Detach aborted.\0" as *const u8 as *const libc::c_char);
    } else {
        Detach(3 as libc::c_int);
    };
}
unsafe extern "C" fn copy_reg_fn(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut pp: *mut plop = plop_tab
        .as_mut_ptr()
        .offset(*buf as libc::c_uchar as libc::c_int as isize);
    if len != 0 {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        return;
    }
    if !((*pp).buf).is_null() {
        free((*pp).buf as *mut libc::c_void);
    }
    (*pp).buf = 0 as *mut libc::c_char;
    (*pp).len = 0 as libc::c_int;
    if (*(*display).d_user).u_plop.len != 0 {
        (*pp)
            .buf = malloc((*(*display).d_user).u_plop.len as libc::c_ulong)
            as *mut libc::c_char;
        if ((*pp).buf).is_null() {
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
            return;
        }
        bcopy(
            (*(*display).d_user).u_plop.buf as *const libc::c_void,
            (*pp).buf as *mut libc::c_void,
            (*(*display).d_user).u_plop.len as size_t,
        );
    }
    (*pp).len = (*(*display).d_user).u_plop.len;
    (*pp).enc = (*(*display).d_user).u_plop.enc;
    Msg(
        0 as libc::c_int,
        b"Copied %d characters into register %c\0" as *const u8 as *const libc::c_char,
        (*(*display).d_user).u_plop.len,
        *buf as libc::c_int,
    );
}
unsafe extern "C" fn ins_reg_fn(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut pp: *mut plop = plop_tab
        .as_mut_ptr()
        .offset(*buf as libc::c_uchar as libc::c_int as isize);
    if len != 0 {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        return;
    }
    if fore.is_null() {
        return;
    }
    if *buf as libc::c_int == '.' as i32 {
        Msg(
            0 as libc::c_int,
            b"ins_reg_fn: Warning: pasting real register '.'!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((*pp).buf).is_null() {
        MakePaster(&mut (*fore).w_paster, (*pp).buf, (*pp).len, 0 as libc::c_int);
        return;
    }
    Msg(0 as libc::c_int, b"Empty register.\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn process_fn(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut pp: *mut plop = plop_tab
        .as_mut_ptr()
        .offset(*buf as libc::c_uchar as libc::c_int as isize);
    if len != 0 {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        return;
    }
    if !((*pp).buf).is_null() {
        ProcessInput((*pp).buf, (*pp).len);
        return;
    }
    Msg(0 as libc::c_int, b"Empty register.\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn confirm_fn(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut act: action = action {
        nr: 0,
        args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        argl: 0 as *const libc::c_int as *mut libc::c_int,
        quiet: 0,
    };
    if len != 0 || *buf as libc::c_int != 'y' as i32 && *buf as libc::c_int != 'Y' as i32
    {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        return;
    }
    act.nr = *(data as *mut libc::c_int);
    act.args = noargs.as_mut_ptr();
    act.argl = 0 as *mut libc::c_int;
    act.quiet = 0 as libc::c_int;
    DoAction(&mut act, -(1 as libc::c_int));
}
unsafe extern "C" fn su_fin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut i: *mut inputsu = data as *mut inputsu;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    if *((*i).name).as_mut_ptr() == 0 {
        p = ((*i).name).as_mut_ptr();
        l = (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    } else if *((*i).pw1).as_mut_ptr() == 0 {
        p = ((*i).pw1).as_mut_ptr();
        strcpy(p, b"\xFF\0" as *const u8 as *const libc::c_char);
        l = (::std::mem::size_of::<[libc::c_char; 130]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    } else {
        p = ((*i).pw2).as_mut_ptr();
        strcpy(p, b"\xFF\0" as *const u8 as *const libc::c_char);
        l = (::std::mem::size_of::<[libc::c_char; 130]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    if !buf.is_null() && len != 0 {
        strncpy(
            p,
            buf,
            (1 as libc::c_int + (if l < len { l } else { len })) as libc::c_ulong,
        );
    }
    if *((*i).name).as_mut_ptr() == 0 {
        Input(
            b"Screen User: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            0 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(su_fin),
                ),
            ),
            i as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else if *((*i).pw1).as_mut_ptr() == 0 {
        Input(
            b"User's UNIX Password: \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 130]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            0 as libc::c_int | 1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(su_fin),
                ),
            ),
            i as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else if *((*i).pw2).as_mut_ptr() == 0 {
        Input(
            b"User's Screen Password: \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 130]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            0 as libc::c_int | 1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(su_fin),
                ),
            ),
            i as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else {
        p = DoSu(
            (*i).up,
            ((*i).name).as_mut_ptr(),
            ((*i).pw2).as_mut_ptr(),
            ((*i).pw1).as_mut_ptr(),
        );
        if !p.is_null() {
            Msg(0 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, p);
        }
        free(i as *mut libc::c_char as *mut libc::c_void);
    };
}
unsafe extern "C" fn InputSu(
    mut w: *mut win,
    mut up: *mut *mut acluser,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut i: *mut inputsu = 0 as *mut inputsu;
    i = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<inputsu>() as libc::c_ulong,
    ) as *mut inputsu;
    if i.is_null() {
        return -(1 as libc::c_int);
    }
    (*i).up = up;
    if !name.is_null() && *name as libc::c_int != 0 {
        su_fin(name, strlen(name) as libc::c_int, i as *mut libc::c_char);
    } else {
        su_fin(0 as *mut libc::c_char, 0 as libc::c_int, i as *mut libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pass1(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut u: *mut acluser = data as *mut acluser;
    if *buf == 0 {
        return;
    }
    if (*u).u_password != NullStr.as_mut_ptr() {
        free((*u).u_password as *mut libc::c_void);
    }
    (*u).u_password = SaveStr(buf);
    bzero(buf as *mut libc::c_void, strlen(buf));
    Input(
        b"Retype new password:\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        100 as libc::c_int,
        1 as libc::c_int,
        Some(
            pass2
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut libc::c_char,
                ) -> (),
        ),
        data,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn pass2(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut st: libc::c_int = 0;
    let mut salt: [libc::c_char; 3] = [0; 3];
    let mut u: *mut acluser = data as *mut acluser;
    if buf.is_null() || strcmp((*u).u_password, buf) != 0 {
        Msg(
            0 as libc::c_int,
            b"[ Passwords don't match - checking turned off ]\0" as *const u8
                as *const libc::c_char,
        );
        if (*u).u_password != NullStr.as_mut_ptr() {
            bzero((*u).u_password as *mut libc::c_void, strlen((*u).u_password));
            free((*u).u_password as *mut libc::c_void);
        }
        (*u).u_password = NullStr.as_mut_ptr();
    } else if *((*u).u_password).offset(0 as libc::c_int as isize) as libc::c_int
        == '\0' as i32
    {
        Msg(
            0 as libc::c_int,
            b"[ No password - no secure ]\0" as *const u8 as *const libc::c_char,
        );
        if !buf.is_null() {
            bzero(buf as *mut libc::c_void, strlen(buf));
        }
    }
    if (*u).u_password != NullStr.as_mut_ptr() {
        st = 0 as libc::c_int;
        while st < 2 as libc::c_int {
            salt[st
                as usize] = ('A' as i32
                + ((time(0 as *mut time_t) >> 6 as libc::c_int * st)
                    % 26 as libc::c_int as libc::c_long) as libc::c_int) as libc::c_char;
            st += 1;
            st;
        }
        salt[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        buf = crypt((*u).u_password, salt.as_mut_ptr());
        bzero((*u).u_password as *mut libc::c_void, strlen((*u).u_password));
        free((*u).u_password as *mut libc::c_void);
        if buf.is_null() {
            Msg(
                0 as libc::c_int,
                b"[ crypt() error - no secure ]\0" as *const u8 as *const libc::c_char,
            );
            (*u).u_password = NullStr.as_mut_ptr();
            return;
        }
        (*u).u_password = SaveStr(buf);
        bzero(buf as *mut libc::c_void, strlen(buf));
        if !((*u).u_plop.buf).is_null() {
            UserFreeCopyBuffer(u);
        }
        (*u).u_plop.len = strlen((*u).u_password) as libc::c_int;
        (*u).u_plop.enc = 0 as libc::c_int;
        (*u).u_plop.buf = SaveStr((*u).u_password);
        if ((*u).u_plop.buf).is_null() {
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
            (*(*display).d_user).u_plop.len = 0 as libc::c_int;
        } else {
            Msg(
                0 as libc::c_int,
                b"[ Password moved into copybuffer ]\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn digraph_find(mut buf: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int
        && digraphs[i as usize].d[0 as libc::c_int as usize] as libc::c_int != 0
    {
        if digraphs[i as usize].d[0 as libc::c_int as usize] as libc::c_int
            == *buf.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            && digraphs[i as usize].d[1 as libc::c_int as usize] as libc::c_int
                == *buf.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            || digraphs[i as usize].d[0 as libc::c_int as usize] as libc::c_int
                == *buf.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                && digraphs[i as usize].d[1 as libc::c_int as usize] as libc::c_int
                    == *buf.offset(0 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    return i;
}
unsafe extern "C" fn digraph_fn(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    ch = *buf.offset(len as isize) as libc::c_int;
    if ch != 0 {
        *buf.offset((len + 1 as libc::c_int) as isize) = ch as libc::c_char;
        if ch < ' ' as i32 || ch == '\u{7f}' as i32 {
            return;
        }
        if len >= 1 as libc::c_int
            && (*buf as libc::c_int == 'U' as i32
                && *buf.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
                || *buf as libc::c_int == '0' as i32
                    && (*buf.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'x' as i32
                        || *buf.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'X' as i32))
        {
            if len == 1 as libc::c_int {
                return;
            }
            if (ch < '0' as i32 || ch > '9' as i32)
                && (ch < 'a' as i32 || ch > 'f' as i32)
                && (ch < 'A' as i32 || ch > 'F' as i32)
            {
                *buf.offset(len as isize) = '\u{1c}' as i32 as libc::c_char;
                return;
            }
            if len
                == (if *buf as libc::c_int == 'U' as i32 {
                    5 as libc::c_int
                } else {
                    3 as libc::c_int
                })
            {
                *buf.offset(len as isize) = '\n' as i32 as libc::c_char;
            }
            return;
        }
        if len != 0 && *buf as libc::c_int == '0' as i32 {
            if ch < '0' as i32 || ch > '7' as i32 {
                *buf.offset(len as isize) = '\u{1c}' as i32 as libc::c_char;
                return;
            }
            if len == 3 as libc::c_int {
                *buf.offset(len as isize) = '\n' as i32 as libc::c_char;
            }
            return;
        }
        if len == 1 as libc::c_int {
            *buf.offset(len as isize) = '\n' as i32 as libc::c_char;
        }
        return;
    }
    if len < 1 as libc::c_int {
        return;
    }
    if *buf.offset((len + 1 as libc::c_int) as isize) != 0 {
        *buf.offset(len as isize) = *buf.offset((len + 1 as libc::c_int) as isize);
        len += 1;
        len;
    }
    if len < 2 as libc::c_int {
        return;
    }
    if parse_input_int(buf, len, &mut x) == 0 {
        i = digraph_find(buf);
        x = digraphs[i as usize].value;
        if x <= 0 as libc::c_int {
            Msg(
                0 as libc::c_int,
                b"Unknown digraph\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    i = 1 as libc::c_int;
    *buf = x as libc::c_char;
    if (*flayer).l_encoding == 8 as libc::c_int {
        i = ToUtf8(buf, x);
    }
    while i != 0 {
        (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap())).unwrap()(&mut buf, &mut i);
    }
}
pub unsafe extern "C" fn StuffKey(mut i: libc::c_int) -> libc::c_int {
    let mut act: *mut action = 0 as *mut action;
    let mut discard: libc::c_int = 0 as libc::c_int;
    let mut keyno: libc::c_int = i;
    if i < 188 as libc::c_int - 106 as libc::c_int && !((*display).d_ESCseen).is_null() {
        let mut act_0: *mut action = &mut *((*display).d_ESCseen)
            .offset((i + 256 as libc::c_int) as isize) as *mut action;
        if (*act_0).nr != -(1 as libc::c_int) {
            (*display).d_ESCseen = 0 as *mut action;
            WindowChanged(fore, 'E' as i32);
            DoAction(act_0, i + 256 as libc::c_int);
            return 0 as libc::c_int;
        }
        discard = 1 as libc::c_int;
    }
    if i >= 166 as libc::c_int - 106 as libc::c_int
        && i < 170 as libc::c_int - 106 as libc::c_int && (*display).d_cursorkeys != 0
    {
        i += 188 as libc::c_int - 166 as libc::c_int;
    } else if i >= 170 as libc::c_int - 106 as libc::c_int
        && i < 188 as libc::c_int - 106 as libc::c_int && (*display).d_keypad != 0
    {
        i += 188 as libc::c_int - 166 as libc::c_int;
    }
    flayer = (*(*display).d_forecv).c_layer;
    fore = (*display).d_fore;
    act = 0 as *mut action;
    if !flayer.is_null() && (*flayer).l_mode == 1 as libc::c_int {
        act = if i
            < 188 as libc::c_int - 106 as libc::c_int
                + (188 as libc::c_int - 166 as libc::c_int)
        {
            &mut *mmtab.as_mut_ptr().offset(i as isize) as *mut action
        } else {
            &mut (*kmap_exts
                .offset(
                    (i
                        - (188 as libc::c_int - 106 as libc::c_int
                            + (188 as libc::c_int - 166 as libc::c_int))) as isize,
                ))
                .mm
        };
    }
    if (act.is_null() || (*act).nr == -(1 as libc::c_int))
        && (*display).d_mapdefault == 0
    {
        act = if i
            < 188 as libc::c_int - 106 as libc::c_int
                + (188 as libc::c_int - 166 as libc::c_int)
        {
            &mut *umtab.as_mut_ptr().offset(i as isize) as *mut action
        } else {
            &mut (*kmap_exts
                .offset(
                    (i
                        - (188 as libc::c_int - 106 as libc::c_int
                            + (188 as libc::c_int - 166 as libc::c_int))) as isize,
                ))
                .um
        };
    }
    if act.is_null() || (*act).nr == -(1 as libc::c_int) {
        act = if i
            < 188 as libc::c_int - 106 as libc::c_int
                + (188 as libc::c_int - 166 as libc::c_int)
        {
            &mut *dmtab.as_mut_ptr().offset(i as isize) as *mut action
        } else {
            &mut (*kmap_exts
                .offset(
                    (i
                        - (188 as libc::c_int - 106 as libc::c_int
                            + (188 as libc::c_int - 166 as libc::c_int))) as isize,
                ))
                .dm
        };
    }
    if discard != 0 && (act.is_null() || (*act).nr != 35 as libc::c_int) {
        if !((*display).d_tcs[(keyno + 106 as libc::c_int) as usize].str_0).is_null()
            && strlen((*display).d_tcs[(keyno + 106 as libc::c_int) as usize].str_0)
                == 1 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        if !((*display).d_ESCseen).is_null() {
            (*display).d_ESCseen = 0 as *mut action;
            WindowChanged(fore, 'E' as i32);
        }
        return 0 as libc::c_int;
    }
    (*display).d_mapdefault = 0 as libc::c_int;
    if act.is_null() || (*act).nr == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    DoAction(act, 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn IsOnDisplay(mut wi: *mut win) -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        if (*(*(*cv).c_layer).l_bottom).l_data as *mut win == wi {
            return 1 as libc::c_int;
        }
        cv = (*cv).c_next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FindNiceWindow(
    mut wi: *mut win,
    mut presel: *mut libc::c_char,
) -> *mut win {
    let mut i: libc::c_int = 0;
    if !presel.is_null() {
        i = WindowByNoN(presel);
        if i >= 0 as libc::c_int {
            wi = *wtab.offset(i as isize);
        }
    }
    if display.is_null() {
        return wi;
    }
    if !wi.is_null() && AclCheckPermWin((*display).d_user, 2 as libc::c_int, wi) != 0 {
        wi = 0 as *mut win;
    }
    if wi.is_null() || IsOnDisplay(wi) != 0 && presel.is_null() {
        wi = 0 as *mut win;
        wi = windows;
        while !wi.is_null() {
            if ((*wi).w_layer.l_cvlist).is_null()
                && AclCheckPermWin((*display).d_user, 1 as libc::c_int, wi) == 0
            {
                break;
            }
            wi = (*wi).w_next;
        }
        if wi.is_null() {
            wi = windows;
            while !wi.is_null() {
                if !((*wi).w_layer.l_cvlist).is_null() && IsOnDisplay(wi) == 0
                    && AclCheckPermWin((*display).d_user, 1 as libc::c_int, wi) == 0
                {
                    break;
                }
                wi = (*wi).w_next;
            }
        }
        if wi.is_null() {
            wi = windows;
            while !wi.is_null() {
                if ((*wi).w_layer.l_cvlist).is_null()
                    && AclCheckPermWin((*display).d_user, 2 as libc::c_int, wi) == 0
                {
                    break;
                }
                wi = (*wi).w_next;
            }
        }
        if wi.is_null() {
            wi = windows;
            while !wi.is_null() {
                if !((*wi).w_layer.l_cvlist).is_null() && IsOnDisplay(wi) == 0
                    && AclCheckPermWin((*display).d_user, 2 as libc::c_int, wi) == 0
                {
                    break;
                }
                wi = (*wi).w_next;
            }
        }
        if wi.is_null() {
            wi = windows;
            while !wi.is_null() {
                if ((*wi).w_layer.l_cvlist).is_null() {
                    break;
                }
                wi = (*wi).w_next;
            }
        }
        if wi.is_null() {
            wi = windows;
            while !wi.is_null() {
                if !((*wi).w_layer.l_cvlist).is_null() && IsOnDisplay(wi) == 0 {
                    break;
                }
                wi = (*wi).w_next;
            }
        }
    }
    if !wi.is_null() && AclCheckPermWin((*display).d_user, 2 as libc::c_int, wi) != 0 {
        wi = 0 as *mut win;
    }
    return wi;
}
unsafe extern "C" fn CalcSlicePercent(
    mut cv: *mut canvas,
    mut percent: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut wsum: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    if cv.is_null() || ((*cv).c_slback).is_null() {
        return percent;
    }
    up = CalcSlicePercent((*(*cv).c_slback).c_slback, percent);
    w = (*cv).c_slweight;
    cv = (*(*cv).c_slback).c_slperp;
    wsum = 0 as libc::c_int;
    while !cv.is_null() {
        wsum += (*cv).c_slweight;
        cv = (*cv).c_slnext;
    }
    if wsum == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return up * w / wsum;
}
unsafe extern "C" fn ChangeCanvasSize(
    mut fcv: *mut canvas,
    mut abs: libc::c_int,
    mut diff: libc::c_int,
    mut gflag: libc::c_int,
    mut percent: libc::c_int,
) -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut done: libc::c_int = 0;
    let mut have: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    if abs == 0 as libc::c_int && diff == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if abs == 2 as libc::c_int {
        if diff == 0 as libc::c_int {
            (*fcv).c_slweight = 0 as libc::c_int;
        } else {
            cv = (*(*fcv).c_slback).c_slperp;
            while !cv.is_null() {
                (*cv).c_slweight = 0 as libc::c_int;
                cv = (*cv).c_slnext;
            }
            (*fcv).c_slweight = 1 as libc::c_int;
            cv = (*(*fcv).c_slback).c_slback;
            if gflag != 0 && !cv.is_null() && !((*cv).c_slback).is_null() {
                ChangeCanvasSize(cv, abs, diff, gflag, percent);
            }
        }
        return diff;
    }
    if abs != 0 {
        if diff < 0 as libc::c_int {
            diff = 0 as libc::c_int;
        }
        if percent != 0 && diff > percent {
            diff = percent;
        }
    }
    if percent != 0 {
        let mut wsum: libc::c_int = 0;
        let mut up: libc::c_int = 0;
        cv = (*(*fcv).c_slback).c_slperp;
        wsum = 0 as libc::c_int;
        while !cv.is_null() {
            wsum += (*cv).c_slweight;
            cv = (*cv).c_slnext;
        }
        if wsum != 0 {
            up = if gflag != 0 {
                CalcSlicePercent((*(*fcv).c_slback).c_slback, percent)
            } else {
                percent
            };
            if wsum < 1000 as libc::c_int {
                let mut scale: libc::c_int = if wsum < 10 as libc::c_int {
                    1000 as libc::c_int
                } else {
                    100 as libc::c_int
                };
                cv = (*(*fcv).c_slback).c_slperp;
                while !cv.is_null() {
                    (*cv).c_slweight *= scale;
                    cv = (*cv).c_slnext;
                }
                wsum *= scale;
            }
            cv = (*(*fcv).c_slback).c_slperp;
            while !cv.is_null() {
                if (*cv).c_slweight != 0 {
                    (*cv).c_slweight = (*cv).c_slweight * up / percent;
                    if (*cv).c_slweight == 0 as libc::c_int {
                        (*cv).c_slweight = 1 as libc::c_int;
                    }
                }
                cv = (*cv).c_slnext;
            }
            diff = diff * wsum / percent;
            percent = wsum;
        }
    } else {
        if abs != 0
            && diff
                == (if (*fcv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int {
                    (*fcv).c_ye - (*fcv).c_ys + 2 as libc::c_int
                } else {
                    (*fcv).c_xe - (*fcv).c_xs + 2 as libc::c_int
                })
        {
            return 0 as libc::c_int;
        }
        cv = (*(*fcv).c_slback).c_slperp;
        while !cv.is_null() {
            (*cv)
                .c_slweight = if (*cv).c_slorient
                == (1 as libc::c_int) << 0 as libc::c_int
            {
                (*cv).c_ye - (*cv).c_ys + 2 as libc::c_int
            } else {
                (*cv).c_xe - (*cv).c_xs + 2 as libc::c_int
            };
            cv = (*cv).c_slnext;
        }
    }
    if abs != 0 {
        diff = diff - (*fcv).c_slweight;
    }
    if diff == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if diff < 0 as libc::c_int {
        cv = if !((*fcv).c_slnext).is_null() {
            (*fcv).c_slnext
        } else {
            (*fcv).c_slprev
        };
        (*fcv).c_slweight += diff;
        (*cv).c_slweight -= diff;
        return diff;
    }
    done = 0 as libc::c_int;
    dir = 1 as libc::c_int;
    cv = (*fcv).c_slnext;
    while diff > 0 as libc::c_int {
        if cv.is_null() {
            if dir == -(1 as libc::c_int) {
                break;
            }
            dir = -(1 as libc::c_int);
            cv = fcv;
        } else {
            if percent != 0 {
                m = 1 as libc::c_int;
            } else {
                m = if !((*cv).c_slperp).is_null() {
                    CountCanvasPerp(cv) * 2 as libc::c_int
                } else {
                    2 as libc::c_int
                };
            }
            if (*cv).c_slweight > m {
                have = (*cv).c_slweight - m;
                if have > diff {
                    have = diff;
                }
                (*cv).c_slweight -= have;
                done += have;
                diff -= have;
            }
        }
        cv = if dir > 0 as libc::c_int { (*cv).c_slnext } else { (*cv).c_slprev };
    }
    if diff != 0 && gflag != 0 {
        cv = (*(*fcv).c_slback).c_slback;
        if !cv.is_null() && !((*cv).c_slback).is_null() {
            done
                += ChangeCanvasSize(
                    (*(*fcv).c_slback).c_slback,
                    0 as libc::c_int,
                    diff,
                    gflag,
                    percent,
                );
        }
    }
    (*fcv).c_slweight += done;
    return done;
}
unsafe extern "C" fn ResizeRegions(mut arg: *mut libc::c_char, mut flags: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut diff: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut gflag: libc::c_int = 0 as libc::c_int;
    let mut abs: libc::c_int = 0 as libc::c_int;
    let mut percent: libc::c_int = 0 as libc::c_int;
    let mut orient: libc::c_int = 0 as libc::c_int;
    if *arg == 0 {
        return;
    }
    if (*(*display).d_forecv).c_slorient == 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"resize: need more than one region\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    gflag = if flags & 4 as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    orient
        |= if flags & 1 as libc::c_int != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    orient
        |= if flags & 2 as libc::c_int != 0 {
            (1 as libc::c_int) << 0 as libc::c_int
        } else {
            0 as libc::c_int
        };
    if orient == 0 as libc::c_int {
        orient = (*(*display).d_forecv).c_slorient;
    }
    l = strlen(arg) as libc::c_int;
    if *arg as libc::c_int == '=' as i32 {
        let mut cv_0: *mut canvas = if gflag != 0 {
            &mut (*display).d_canvas
        } else {
            (*(*display).d_forecv).c_slback
        };
        if (*(*cv_0).c_slperp).c_slorient & orient != 0 {
            EqualizeCanvas((*cv_0).c_slperp, gflag);
        }
        if ((*(*cv_0).c_slperp).c_slorient
            ^ ((1 as libc::c_int) << 1 as libc::c_int
                ^ (1 as libc::c_int) << 0 as libc::c_int)) & orient != 0
        {
            if !((*cv_0).c_slback).is_null() {
                cv_0 = (*cv_0).c_slback;
                EqualizeCanvas((*cv_0).c_slperp, gflag);
            } else {
                EqualizeCanvas(cv_0, gflag);
            }
        }
        ResizeCanvas(cv_0);
        RecreateCanvasChain();
        RethinkDisplayViewports();
        ResizeLayersToCanvases();
        return;
    }
    if strcmp(arg, b"min\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(arg, b"0\0" as *const u8 as *const libc::c_char) == 0
    {
        abs = 2 as libc::c_int;
        diff = 0 as libc::c_int;
    } else if strcmp(arg, b"max\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(arg, b"_\0" as *const u8 as *const libc::c_char) == 0
    {
        abs = 2 as libc::c_int;
        diff = 1 as libc::c_int;
    } else {
        if l > 0 as libc::c_int
            && *arg.offset((l - 1 as libc::c_int) as isize) as libc::c_int == '%' as i32
        {
            percent = 1000 as libc::c_int;
        }
        if *arg as libc::c_int == '+' as i32 {
            diff = atoi(arg.offset(1 as libc::c_int as isize));
        } else if *arg as libc::c_int == '-' as i32 {
            diff = -atoi(arg.offset(1 as libc::c_int as isize));
        } else {
            diff = atoi(arg);
            if diff < 0 as libc::c_int {
                diff = 0 as libc::c_int;
            }
            abs = if diff == 0 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
    }
    if abs == 0 && diff == 0 {
        return;
    }
    if percent != 0 {
        diff = diff * percent / 100 as libc::c_int;
    }
    cv = (*display).d_forecv;
    if (*cv).c_slorient & orient != 0 {
        ChangeCanvasSize(cv, abs, diff, gflag, percent);
    }
    if (*(*cv).c_slback).c_slorient & orient != 0 {
        ChangeCanvasSize((*cv).c_slback, abs, diff, gflag, percent);
    }
    ResizeCanvas(&mut (*display).d_canvas);
    RecreateCanvasChain();
    RethinkDisplayViewports();
    ResizeLayersToCanvases();
}
unsafe extern "C" fn ResizeFin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut ch: libc::c_int = 0;
    let mut flags: libc::c_int = *(data as *mut libc::c_int);
    ch = *(buf as *mut libc::c_uchar).offset(len as isize) as libc::c_int;
    if ch == 0 as libc::c_int {
        ResizeRegions(buf, flags);
        return;
    }
    if ch == 'h' as i32 {
        flags ^= 1 as libc::c_int;
    } else if ch == 'v' as i32 {
        flags ^= 2 as libc::c_int;
    } else if ch == 'b' as i32 {
        flags |= 1 as libc::c_int | 2 as libc::c_int;
    } else if ch == 'p' as i32 {
        flags
            ^= if (*(*display).d_forecv).c_slorient
                == (1 as libc::c_int) << 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                2 as libc::c_int
            };
    } else if ch == 'l' as i32 {
        flags ^= 4 as libc::c_int;
    } else {
        return
    }
    inp_setprompt(resizeprompts[flags as usize], 0 as *mut libc::c_char);
    *(data as *mut libc::c_int) = flags;
    *buf.offset(len as isize) = '\u{1c}' as i32 as libc::c_char;
}
pub unsafe extern "C" fn SetForeCanvas(mut d: *mut display, mut cv: *mut canvas) {
    let mut odisplay: *mut display = display;
    if (*d).d_forecv == cv {
        return;
    }
    display = d;
    (*display).d_forecv = cv;
    if focusminwidth != 0
        && (focusminwidth < 0 as libc::c_int
            || ((*(*display).d_forecv).c_xe - (*(*display).d_forecv).c_xs
                + 1 as libc::c_int) < focusminwidth)
        || focusminheight != 0
            && (focusminheight < 0 as libc::c_int
                || ((*(*display).d_forecv).c_ye - (*(*display).d_forecv).c_ys
                    + 1 as libc::c_int) < focusminheight)
    {
        ResizeCanvas(&mut (*display).d_canvas);
        RecreateCanvasChain();
        RethinkDisplayViewports();
        ResizeLayersToCanvases();
    }
    (*display).d_fore = (*(*(*(*display).d_forecv).c_layer).l_bottom).l_data as *mut win;
    fore = (*display).d_fore;
    if (*display).d_other == fore {
        (*display).d_other = 0 as *mut win;
    }
    flayer = (*(*display).d_forecv).c_layer;
    RefreshHStatus();
    flayer = (*(*display).d_forecv).c_layer;
    let mut olddisplay: *mut display = display;
    let mut oldflayer: *mut layer = flayer;
    let mut l: *mut layer = (*(*display).d_forecv).c_layer;
    let mut cvlist: *mut canvas = (*l).l_cvlist;
    let mut cvlnext: *mut canvas = (*(*display).d_forecv).c_lnext;
    flayer = l;
    (*l).l_cvlist = (*display).d_forecv;
    (*(*display).d_forecv).c_lnext = 0 as *mut canvas;
    (Some(((*(*flayer).l_layfn).lf_LayRestore).unwrap())).unwrap()();
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
    flayer = oldflayer;
    (*l).l_cvlist = cvlist;
    (*(*display).d_forecv).c_lnext = cvlnext;
    display = olddisplay;
    WindowChanged(0 as *mut win, 'F' as i32);
    display = odisplay;
}
pub unsafe extern "C" fn ParseAttrColor(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
    mut msgok: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0 as libc::c_int;
    s = s1;
    while *s as libc::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    ss = s;
    while *ss as libc::c_int != 0 && *ss as libc::c_int != ' ' as i32 {
        ss = ss.offset(1);
        ss;
    }
    while *ss as libc::c_int == ' ' as i32 {
        ss = ss.offset(1);
        ss;
    }
    if *s as libc::c_int != 0
        && (!s2.is_null() || *ss as libc::c_int != 0
            || !(*s as libc::c_int >= 'a' as i32 && *s as libc::c_int <= 'z' as i32
                || *s as libc::c_int >= 'A' as i32 && *s as libc::c_int <= 'Z' as i32
                || *s as libc::c_int == '.' as i32))
    {
        let mut mode: libc::c_int = 0 as libc::c_int;
        let mut n_0: libc::c_int = 0 as libc::c_int;
        if *s as libc::c_int == '+' as i32 {
            mode = 1 as libc::c_int;
            s = s.offset(1);
            s;
        } else if *s as libc::c_int == '-' as i32 {
            mode = -(1 as libc::c_int);
            s = s.offset(1);
            s;
        } else if *s as libc::c_int == '!' as i32 {
            mode = 2 as libc::c_int;
            s = s.offset(1);
            s;
        } else if *s as libc::c_int == '=' as i32 {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
            let fresh62 = s;
            s = s.offset(1);
            n_0 = *fresh62 as libc::c_int - '0' as i32;
            if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
                let fresh63 = s;
                s = s.offset(1);
                n_0 = n_0 * 16 as libc::c_int + (*fresh63 as libc::c_int - '0' as i32);
            } else if *s as libc::c_int >= 'a' as i32 && *s as libc::c_int <= 'f' as i32
            {
                let fresh64 = s;
                s = s.offset(1);
                n_0 = n_0 * 16 as libc::c_int
                    + (*fresh64 as libc::c_int - ('a' as i32 - 10 as libc::c_int));
            } else if *s as libc::c_int >= 'A' as i32 && *s as libc::c_int <= 'F' as i32
            {
                let fresh65 = s;
                s = s.offset(1);
                n_0 = n_0 * 16 as libc::c_int
                    + (*fresh65 as libc::c_int - ('A' as i32 - 10 as libc::c_int));
            } else if *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
                if msgok != 0 {
                    Msg(
                        0 as libc::c_int,
                        b"Illegal attribute hexchar '%c'\0" as *const u8
                            as *const libc::c_char,
                        *s as libc::c_int,
                    );
                }
                return -(1 as libc::c_int);
            }
        } else {
            while *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
                if *s as libc::c_int == 'd' as i32 {
                    n_0 |= (1 as libc::c_int) << 0 as libc::c_int;
                } else if *s as libc::c_int == 'u' as i32 {
                    n_0 |= (1 as libc::c_int) << 1 as libc::c_int;
                } else if *s as libc::c_int == 'b' as i32 {
                    n_0 |= (1 as libc::c_int) << 2 as libc::c_int;
                } else if *s as libc::c_int == 'r' as i32 {
                    n_0 |= (1 as libc::c_int) << 3 as libc::c_int;
                } else if *s as libc::c_int == 's' as i32 {
                    n_0 |= (1 as libc::c_int) << 4 as libc::c_int;
                } else if *s as libc::c_int == 'B' as i32 {
                    n_0 |= (1 as libc::c_int) << 5 as libc::c_int;
                } else {
                    if msgok != 0 {
                        Msg(
                            0 as libc::c_int,
                            b"Illegal attribute specifier '%c'\0" as *const u8
                                as *const libc::c_char,
                            *s as libc::c_int,
                        );
                    }
                    return -(1 as libc::c_int);
                }
                s = s.offset(1);
                s;
            }
        }
        if *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
            if msgok != 0 {
                Msg(
                    0 as libc::c_int,
                    b"junk after attribute description: '%c'\0" as *const u8
                        as *const libc::c_char,
                    *s as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        }
        if mode == -(1 as libc::c_int) {
            r = n_0 << 8 as libc::c_int | n_0;
        } else if mode == 1 as libc::c_int {
            r = n_0 << 8 as libc::c_int;
        } else if mode == 2 as libc::c_int {
            r = n_0;
        } else if mode == 0 as libc::c_int {
            r = 0xffff as libc::c_int ^ n_0;
        }
    }
    while *s as libc::c_int != 0 && *s as libc::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    if !s2.is_null() {
        if *s != 0 {
            if msgok != 0 {
                Msg(
                    0 as libc::c_int,
                    b"junk after description: '%c'\0" as *const u8
                        as *const libc::c_char,
                    *s as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        }
        s = s2;
        while *s as libc::c_int != 0 && *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
    }
    if *s != 0 {
        static mut costr: [libc::c_char; 65] = unsafe {
            *::std::mem::transmute::<
                &[u8; 65],
                &mut [libc::c_char; 65],
            >(b"krgybmcw d    i.01234567 9     f               FKRGYBMCW      I \0")
        };
        let mut numco: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0;
        n = 0 as libc::c_int;
        if *s as libc::c_int == '.' as i32 {
            numco += 1;
            numco;
            n = 0xf as libc::c_int;
            s = s.offset(1);
            s;
        }
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int && *s as libc::c_int != 0
            && *s as libc::c_int != ' ' as i32
        {
            i = 0 as libc::c_int;
            while costr[i as usize] != 0 {
                if *s as libc::c_int == costr[i as usize] as libc::c_int {
                    break;
                }
                i += 1;
                i;
            }
            if costr[i as usize] == 0 {
                if msgok != 0 {
                    Msg(
                        0 as libc::c_int,
                        b"illegal color descriptor: '%c'\0" as *const u8
                            as *const libc::c_char,
                        *s as libc::c_int,
                    );
                }
                return -(1 as libc::c_int);
            }
            numco += 1;
            numco;
            n = n << 4 as libc::c_int | i & 15 as libc::c_int;
            if i >= 48 as libc::c_int {
                n = n & 0x20ff as libc::c_int | 0x200 as libc::c_int;
            }
            s = s.offset(1);
            s;
            j += 1;
            j;
        }
        if n & 0xf00 as libc::c_int == 0xf00 as libc::c_int {
            n ^= 0xf00 as libc::c_int;
        }
        if n & 0x2000 as libc::c_int != 0 {
            n ^= 0x2400 as libc::c_int;
        }
        if numco == 1 as libc::c_int {
            n |= 0xf0 as libc::c_int;
        }
        if numco != 2 as libc::c_int && n != 0xff as libc::c_int {
            n |= 0x100 as libc::c_int;
        }
        if *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
            if msgok != 0 {
                Msg(
                    0 as libc::c_int,
                    b"junk after color description: '%c'\0" as *const u8
                        as *const libc::c_char,
                    *s as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        }
        n ^= 0xff as libc::c_int;
        r |= n << 16 as libc::c_int;
    }
    while *s as libc::c_int != 0 && *s as libc::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    if *s != 0 {
        if msgok != 0 {
            Msg(
                0 as libc::c_int,
                b"junk after description: '%c'\0" as *const u8 as *const libc::c_char,
                *s as libc::c_int,
            );
        }
        return -(1 as libc::c_int);
    }
    return r;
}
pub unsafe extern "C" fn ApplyAttrColor(mut i: libc::c_int, mut mc: *mut mchar) {
    (*mc)
        .attr = ((*mc).attr as libc::c_int | i >> 8 as libc::c_int & 255 as libc::c_int)
        as libc::c_uchar;
    (*mc).attr = ((*mc).attr as libc::c_int ^ i & 255 as libc::c_int) as libc::c_uchar;
    i = i >> 16 as libc::c_int ^ 0xff as libc::c_int;
    if i & 0x100 as libc::c_int != 0 as libc::c_int {
        i &= 0xeff as libc::c_int;
        if (*mc).attr as libc::c_int
            & ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int) != 0
        {
            i = (i & 0xf as libc::c_int) << 4 as libc::c_int
                | (i & 0xf0 as libc::c_int) >> 4 as libc::c_int
                | (i & 0x200 as libc::c_int) << 1 as libc::c_int
                | (i & 0x400 as libc::c_int) >> 1 as libc::c_int;
        }
    }
    if i & 0xf as libc::c_int != 0xf as libc::c_int {
        (*mc)
            .attr = ((*mc).attr as libc::c_int & 0xbf as libc::c_int
            | i >> 3 as libc::c_int & 0x40 as libc::c_int) as libc::c_uchar;
    }
    if i & 0xf0 as libc::c_int != 0xf0 as libc::c_int {
        (*mc)
            .attr = ((*mc).attr as libc::c_int & 0x7f as libc::c_int
            | i >> 3 as libc::c_int & 0x80 as libc::c_int) as libc::c_uchar;
    }
    (*mc).color = (0x99 as libc::c_int ^ (*mc).color as libc::c_int) as libc::c_uchar;
    if i & 0xe as libc::c_int == 0xe as libc::c_int {
        i = i & 0xf0 as libc::c_int | (*mc).color as libc::c_int & 0xf as libc::c_int;
    }
    if i & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
        i = i & 0xf as libc::c_int | (*mc).color as libc::c_int & 0xf0 as libc::c_int;
    }
    (*mc).color = (0x99 as libc::c_int ^ i) as libc::c_uchar;
}
