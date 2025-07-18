use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn bcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn MakeWinMsg(
        _: *mut libc::c_char,
        _: *mut win,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn printpipe(_: *mut win, _: *mut libc::c_char) -> libc::c_int;
    fn WListUpdatecv(_: *mut canvas, _: *mut win);
    fn DoCommand(_: *mut *mut libc::c_char, _: *mut libc::c_int);
    fn Parse(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn AddCStr(_: *mut libc::c_char);
    fn RefreshLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn Redisplay(_: libc::c_int);
    fn RefreshHStatus();
    fn GotoPos(_: libc::c_int, _: libc::c_int);
    fn InsertMode(_: libc::c_int);
    fn ReverseVideo(_: libc::c_int);
    fn MakeStatus(_: *mut libc::c_char);
    fn ResizeDisplay(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn AddStr(_: *mut libc::c_char);
    fn AddStrn(_: *mut libc::c_char, _: libc::c_int);
    fn Flush(_: libc::c_int);
    fn color256to16(_: libc::c_int) -> libc::c_int;
    fn ChangeWindowSize(
        _: *mut win,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn ResizeLayersToCanvases();
    fn EnterAltScreen(_: *mut win);
    fn LeaveAltScreen(_: *mut win);
    fn SetTimeout(_: *mut event, _: libc::c_int);
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn bclear(_: *mut libc::c_char, _: libc::c_int);
    fn AddXChar(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn FindUserPtr(_: *mut libc::c_char) -> *mut *mut acluser;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LPutChar(_: *mut layer, _: *mut mchar, _: libc::c_int, _: libc::c_int);
    fn LInsChar(
        _: *mut layer,
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mline,
    );
    fn LPutStr(
        _: *mut layer,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
    );
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
    fn LClearAll(_: *mut layer, _: libc::c_int);
    fn LClearArea(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LRefreshAll(_: *mut layer, _: libc::c_int);
    fn LSetRendition(_: *mut layer, _: *mut mchar);
    fn LWrapChar(
        _: *mut layer,
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LCursorVisibility(_: *mut layer, _: libc::c_int);
    fn LSetFlow(_: *mut layer, _: libc::c_int);
    fn LKeypadMode(_: *mut layer, _: libc::c_int);
    fn LCursorkeysMode(_: *mut layer, _: libc::c_int);
    fn LMouseMode(_: *mut layer, _: libc::c_int);
    fn LExtMouseMode(_: *mut layer, _: libc::c_int);
    fn LMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn recode_mchar(_: *mut mchar, _: libc::c_int, _: libc::c_int) -> *mut mchar;
    fn FromUtf8(_: libc::c_int, _: *mut libc::c_int) -> libc::c_int;
    fn utf8_isdouble(_: libc::c_int) -> libc::c_int;
    fn utf8_iscomb(_: libc::c_int) -> libc::c_int;
    fn utf8_handle_comb(_: libc::c_int, _: *mut mchar);
    fn ResetEncoding(_: *mut win);
    fn logfclose(_: *mut logfile) -> libc::c_int;
    fn logfwrite(_: *mut logfile, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn logfflush(ifany: *mut logfile) -> libc::c_int;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut fore: *mut win;
    static mut flayer: *mut layer;
    static mut nwin_default: NewWindow;
    static mut nversion: libc::c_int;
    static mut log_flush: libc::c_int;
    static mut logtstamp_on: libc::c_int;
    static mut logtstamp_after: libc::c_int;
    static mut logtstamp_string: *mut libc::c_char;
    static mut captionstring: *mut libc::c_char;
    static mut hstatusstring: *mut libc::c_char;
    static mut wliststr: *mut libc::c_char;
    static mut compacthist: libc::c_int;
    static mut EffectiveAclUser: *mut acluser;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut Z0width: libc::c_int = 132 as libc::c_int;
pub static mut Z1width: libc::c_int = 80 as libc::c_int;
static mut curr: *mut win = 0 as *const win as *mut win;
static mut rows: libc::c_int = 0;
static mut cols: libc::c_int = 0;
pub static mut visual_bell: libc::c_int = 0 as libc::c_int;
pub static mut use_hardstatus: libc::c_int = 1 as libc::c_int;
pub static mut printcmd: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut use_altscreen: libc::c_int = 0 as libc::c_int;
pub static mut blank: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
pub static mut null: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
pub static mut mline_old: mline = mline {
    image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
};
pub static mut mline_blank: mline = mline {
    image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
};
pub static mut mline_null: mline = mline {
    image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
};
pub static mut mchar_null: mchar = mchar {
    image: 0,
    attr: 0,
    font: 0,
    fontx: 0,
    color: 0,
    mbcs: 0,
};
pub static mut mchar_blank: mchar = {
    let mut init = mchar {
        image: ' ' as i32 as libc::c_uchar,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    init
};
pub static mut mchar_so: mchar = {
    let mut init = mchar {
        image: ' ' as i32 as libc::c_uchar,
        attr: ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uchar,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    init
};
pub static mut renditions: [libc::c_int; 3] = [
    65529 as libc::c_int,
    65531 as libc::c_int,
    65533 as libc::c_int,
];
static mut string_t_string: [*mut libc::c_char; 8] = [
    b"NONE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"DCS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OSC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"APC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"AKA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"GM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STATUS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut state_t_string: [*mut libc::c_char; 9] = [
    b"LIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ESC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ASTR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STRESC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CSI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PRIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PRINESC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PRINCSI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PRIN4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn ResetAnsiState(mut p: *mut win) {
    (*p).w_state = LIT;
    (*p).w_StringType = NONE;
}
pub unsafe extern "C" fn ResetWindow(mut p: *mut win) {
    let mut i: libc::c_int = 0;
    (*p).w_wrap = nwin_default.wrap;
    (*p).w_origin = 0 as libc::c_int;
    (*p).w_insert = 0 as libc::c_int;
    (*p).w_revvid = 0 as libc::c_int;
    (*p).w_mouse = 0 as libc::c_int;
    (*p).w_curinv = 0 as libc::c_int;
    (*p).w_curvvis = 0 as libc::c_int;
    (*p).w_autolf = 0 as libc::c_int;
    (*p).w_keypad = 0 as libc::c_int;
    (*p).w_cursorkeys = 0 as libc::c_int;
    (*p).w_top = 0 as libc::c_int;
    (*p).w_bot = (*p).w_layer.l_height - 1 as libc::c_int;
    (*p).w_saved.on = 0 as libc::c_int;
    (*p).w_layer.l_y = 0 as libc::c_int;
    (*p).w_layer.l_x = (*p).w_layer.l_y;
    (*p).w_state = LIT;
    (*p).w_StringType = NONE;
    bzero((*p).w_tabs as *mut libc::c_void, (*p).w_layer.l_width as libc::c_ulong);
    i = 8 as libc::c_int;
    while i < (*p).w_layer.l_width {
        *((*p).w_tabs).offset(i as isize) = 1 as libc::c_int as libc::c_char;
        i += 8 as libc::c_int;
    }
    (*p).w_rend = mchar_null;
    ResetCharsets(p);
    (*p).w_bce = nwin_default.bce;
}
pub unsafe extern "C" fn GetAnsiStatus(
    mut w: *mut win,
    mut buf: *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = buf;
    if (*w).w_state as libc::c_uint == LIT as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    strcpy(p, state_t_string[(*w).w_state as usize]);
    p = p.offset(strlen(p) as isize);
    if (*w).w_intermediate != 0 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '-' as i32 as libc::c_char;
        if (*w).w_intermediate > 0xff as libc::c_int {
            p = p.offset(AddXChar(p, (*w).w_intermediate >> 8 as libc::c_int) as isize);
        }
        p = p.offset(AddXChar(p, (*w).w_intermediate & 0xff as libc::c_int) as isize);
        *p = 0 as libc::c_int as libc::c_char;
    }
    if (*w).w_state as libc::c_uint == ASTR as libc::c_int as libc::c_uint
        || (*w).w_state as libc::c_uint == STRESC as libc::c_int as libc::c_uint
    {
        sprintf(
            p,
            b"-%s\0" as *const u8 as *const libc::c_char,
            string_t_string[(*w).w_StringType as usize],
        );
    }
    p = p.offset(strlen(p) as isize);
    return p.offset_from(buf) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn ResetCharsets(mut p: *mut win) {
    (*p).w_gr = nwin_default.gr;
    (*p).w_c1 = nwin_default.c1;
    SetCharsets(p, b"BBBB02\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !(nwin_default.charset).is_null() {
        SetCharsets(p, nwin_default.charset);
    }
    ResetEncoding(p);
}
pub unsafe extern "C" fn SetCharsets(mut p: *mut win, mut s: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int && *s as libc::c_int != 0 {
        if *s as libc::c_int != '.' as i32 {
            (*p)
                .w_charsets[i
                as usize] = if *s as libc::c_int == 'B' as i32 {
                0 as libc::c_int
            } else {
                *s as libc::c_int
            };
        }
        i += 1;
        i;
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int != 0
        && {
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 as libc::c_int != '.' as i32
        }
    {
        (*p)
            .w_Charset = *s.offset(-(1 as libc::c_int) as isize) as libc::c_int
            - '0' as i32;
    }
    if *s as libc::c_int != 0 && *s as libc::c_int != '.' as i32 {
        (*p).w_CharsetR = *s as libc::c_int - '0' as i32;
    }
    (*p).w_ss = 0 as libc::c_int;
    (*p).w_FontL = (*p).w_charsets[(*p).w_Charset as usize] as libc::c_char;
    (*p).w_FontR = (*p).w_charsets[(*p).w_CharsetR as usize] as libc::c_char;
}
pub unsafe extern "C" fn WriteString(
    mut wp: *mut win,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut font: libc::c_int = 0;
    let mut cv: *mut canvas = 0 as *mut canvas;
    if len == 0 {
        return;
    }
    if !((*wp).w_log).is_null() {
        WLogString(wp, buf, len);
    }
    curr = wp;
    cols = (*curr).w_layer.l_width;
    rows = (*curr).w_layer.l_height;
    if (*curr).w_silence != 0 {
        SetTimeout(
            &mut (*curr).w_silenceev,
            (*curr).w_silencewait * 1000 as libc::c_int,
        );
    }
    if (*curr).w_monitor == 1 as libc::c_int {
        (*curr).w_monitor = 2 as libc::c_int;
    }
    if cols > 0 as libc::c_int && rows > 0 as libc::c_int {
        's_67: loop {
            let fresh2 = buf;
            buf = buf.offset(1);
            c = *fresh2 as libc::c_uchar as libc::c_int;
            if (*curr).w_mbcs == 0 {
                (*curr).w_rend.font = (*curr).w_FontL as libc::c_uchar;
            }
            if (*curr).w_state as libc::c_uint == LIT as libc::c_int as libc::c_uint
                && (*curr).w_layer.l_encoding != 8 as libc::c_int
                && !((*curr).w_rend.font as libc::c_int != 0
                    && (*curr).w_rend.font as libc::c_int & 0x60 as libc::c_int
                        == 0 as libc::c_int)
                && (*curr).w_rend.font as libc::c_int != 'I' as i32
                && (*curr).w_mbcs == 0
                && (*curr).w_rend.font as libc::c_int != '<' as i32 && c >= ' ' as i32
                && c != 0x7f as libc::c_int
                && (c & 0x80 as libc::c_int == 0 as libc::c_int
                    || (c >= 0xa0 as libc::c_int || (*curr).w_c1 == 0)
                        && (*curr).w_gr == 0) && (*curr).w_ss == 0
                && (*curr).w_insert == 0 && (*curr).w_layer.l_x < cols - 1 as libc::c_int
            {
                let mut currx: libc::c_int = (*curr).w_layer.l_x;
                let mut imp: *mut libc::c_char = buf
                    .offset(-(1 as libc::c_int as isize));
                while currx < cols - 1 as libc::c_int {
                    currx += 1;
                    currx;
                    len -= 1;
                    if len == 0 as libc::c_int {
                        break;
                    }
                    let fresh3 = buf;
                    buf = buf.offset(1);
                    c = *fresh3 as libc::c_uchar as libc::c_int;
                    if c < ' ' as i32 || c == 0x7f as libc::c_int
                        || c & 0x80 as libc::c_int != 0
                            && (c < 0xa0 as libc::c_int && (*curr).w_c1 != 0
                                || (*curr).w_gr != 0)
                    {
                        break;
                    }
                }
                currx -= (*curr).w_layer.l_x;
                if currx > 0 as libc::c_int {
                    MPutStr(
                        curr,
                        imp,
                        currx,
                        &mut (*curr).w_rend,
                        (*curr).w_layer.l_x,
                        (*curr).w_layer.l_y,
                    );
                    LPutStr(
                        &mut (*curr).w_layer,
                        imp,
                        currx,
                        &mut (*curr).w_rend,
                        (*curr).w_layer.l_x,
                        (*curr).w_layer.l_y,
                    );
                    (*curr).w_layer.l_x += currx;
                }
                if len == 0 as libc::c_int {
                    break;
                }
            }
            if (*curr).w_layer.l_encoding == 8 as libc::c_int {
                c = FromUtf8(c, &mut (*curr).w_decodestate);
                if c == -(1 as libc::c_int) {
                    current_block = 2979737022853876585;
                } else {
                    if c == -(2 as libc::c_int) {
                        c = 0xfffd as libc::c_int;
                        buf = buf.offset(-1);
                        buf;
                        len += 1;
                        len;
                    }
                    c > 0xff as libc::c_int;
                    current_block = 4650853661393082352;
                }
            } else {
                current_block = 4650853661393082352;
            }
            loop {
                match current_block {
                    2979737022853876585 => {
                        len -= 1;
                        if len != 0 {
                            break;
                        } else {
                            break 's_67;
                        }
                    }
                    _ => {
                        match (*curr).w_state as libc::c_uint {
                            5 => {
                                match c {
                                    27 => {
                                        (*curr).w_state = PRINESC;
                                    }
                                    _ => {
                                        PrintChar(c);
                                    }
                                }
                                current_block = 2979737022853876585;
                                continue;
                            }
                            6 => {
                                match c {
                                    91 => {
                                        (*curr).w_state = PRINCSI;
                                    }
                                    _ => {
                                        PrintChar('\u{1b}' as i32);
                                        PrintChar(c);
                                        (*curr).w_state = PRIN;
                                    }
                                }
                                current_block = 2979737022853876585;
                                continue;
                            }
                            7 => {
                                match c {
                                    52 => {
                                        (*curr).w_state = PRIN4;
                                    }
                                    _ => {
                                        PrintChar('\u{1b}' as i32);
                                        PrintChar('[' as i32);
                                        PrintChar(c);
                                        (*curr).w_state = PRIN;
                                    }
                                }
                                current_block = 2979737022853876585;
                                continue;
                            }
                            8 => {
                                match c {
                                    105 => {
                                        (*curr).w_state = LIT;
                                        PrintFlush();
                                        if !((*curr).w_pdisplay).is_null()
                                            && (*(*curr).w_pdisplay).d_printfd >= 0 as libc::c_int
                                        {
                                            close((*(*curr).w_pdisplay).d_printfd);
                                            (*(*curr).w_pdisplay).d_printfd = -(1 as libc::c_int);
                                        }
                                        (*curr).w_pdisplay = 0 as *mut display;
                                    }
                                    _ => {
                                        PrintChar('\u{1b}' as i32);
                                        PrintChar('[' as i32);
                                        PrintChar('4' as i32);
                                        PrintChar(c);
                                        (*curr).w_state = PRIN;
                                    }
                                }
                                current_block = 2979737022853876585;
                                continue;
                            }
                            2 => {
                                if c == 0 as libc::c_int {
                                    current_block = 2979737022853876585;
                                    continue;
                                }
                                if c == '\u{1b}' as i32 {
                                    (*curr).w_state = STRESC;
                                    current_block = 2979737022853876585;
                                    continue;
                                } else {
                                    if !((*curr).w_StringType as libc::c_uint
                                        == OSC as libc::c_int as libc::c_uint && c < ' ' as i32
                                        && c != '\u{5}' as i32)
                                    {
                                        if (*curr).w_c1 == 0
                                            || c != '\\' as i32 ^ 0xc0 as libc::c_int
                                        {
                                            StringChar(c);
                                            current_block = 2979737022853876585;
                                            continue;
                                        }
                                    }
                                    c = '\\' as i32;
                                }
                            }
                            3 => {}
                            1 => {
                                match c {
                                    91 => {
                                        (*curr).w_NumArgs = 0 as libc::c_int;
                                        (*curr).w_intermediate = 0 as libc::c_int;
                                        bzero(
                                            ((*curr).w_args).as_mut_ptr() as *mut libc::c_char
                                                as *mut libc::c_void,
                                            (64 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(
                                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                                ),
                                        );
                                        (*curr).w_state = CSI;
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    93 => {
                                        StringStart(OSC);
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    95 => {
                                        StringStart(APC);
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    80 => {
                                        StringStart(DCS);
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    94 => {
                                        StringStart(PM);
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    33 => {
                                        StringStart(GM);
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    34 | 107 => {
                                        StringStart(AKA);
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    _ => {
                                        if Special(c) != 0 {
                                            (*curr).w_state = LIT;
                                            current_block = 2979737022853876585;
                                            continue;
                                        } else if c >= ' ' as i32 && c <= '/' as i32 {
                                            if (*curr).w_intermediate != 0 {
                                                if (*curr).w_intermediate == '$' as i32 {
                                                    c |= ('$' as i32) << 8 as libc::c_int;
                                                } else {
                                                    c = -(1 as libc::c_int);
                                                }
                                            }
                                            (*curr).w_intermediate = c;
                                            current_block = 2979737022853876585;
                                            continue;
                                        } else if c >= '0' as i32 && c <= '~' as i32 {
                                            DoESC(c, (*curr).w_intermediate);
                                            (*curr).w_state = LIT;
                                            current_block = 2979737022853876585;
                                            continue;
                                        } else {
                                            (*curr).w_state = LIT;
                                            current_block = 4650853661393082352;
                                            continue;
                                        }
                                    }
                                }
                            }
                            4 => {
                                match c {
                                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                                        if (*curr).w_NumArgs >= 0 as libc::c_int
                                            && (*curr).w_NumArgs < 64 as libc::c_int
                                        {
                                            if (*curr).w_args[(*curr).w_NumArgs as usize]
                                                < 100000000 as libc::c_int
                                            {
                                                (*curr)
                                                    .w_args[(*curr).w_NumArgs
                                                    as usize] = 10 as libc::c_int
                                                    * (*curr).w_args[(*curr).w_NumArgs as usize]
                                                    + (c - '0' as i32);
                                            }
                                        }
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    59 | 58 => {
                                        if (*curr).w_NumArgs < 64 as libc::c_int {
                                            (*curr).w_NumArgs += 1;
                                            (*curr).w_NumArgs;
                                        }
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                    _ => {
                                        if Special(c) != 0 {
                                            current_block = 2979737022853876585;
                                            continue;
                                        }
                                        if c >= '@' as i32 && c <= '~' as i32 {
                                            if (*curr).w_NumArgs < 64 as libc::c_int {
                                                (*curr).w_NumArgs += 1;
                                                (*curr).w_NumArgs;
                                            }
                                            DoCSI(c, (*curr).w_intermediate);
                                            if (*curr).w_state as libc::c_uint
                                                != PRIN as libc::c_int as libc::c_uint
                                            {
                                                (*curr).w_state = LIT;
                                            }
                                            current_block = 2979737022853876585;
                                            continue;
                                        } else if c >= ' ' as i32 && c <= '/' as i32
                                            || c >= '<' as i32 && c <= '?' as i32
                                        {
                                            (*curr)
                                                .w_intermediate = if (*curr).w_intermediate != 0 {
                                                -(1 as libc::c_int)
                                            } else {
                                                c
                                            };
                                            current_block = 2979737022853876585;
                                            continue;
                                        } else {
                                            (*curr).w_state = LIT;
                                            current_block = 4650853661393082352;
                                            continue;
                                        }
                                    }
                                }
                            }
                            0 | _ => {
                                if (*curr).w_mbcs != 0 {
                                    if c <= ' ' as i32 || c == 0x7f as libc::c_int
                                        || c >= 0x80 as libc::c_int && c < 0xa0 as libc::c_int
                                            && (*curr).w_c1 != 0
                                    {
                                        (*curr).w_mbcs = 0 as libc::c_int;
                                    }
                                }
                                if c < ' ' as i32 {
                                    if c == '\u{1b}' as i32 {
                                        (*curr).w_intermediate = 0 as libc::c_int;
                                        (*curr).w_state = ESC;
                                        if (*curr).w_autoaka < 0 as libc::c_int {
                                            (*curr).w_autoaka = 0 as libc::c_int;
                                        }
                                    } else {
                                        Special(c);
                                    }
                                    current_block = 2979737022853876585;
                                    continue;
                                } else {
                                    if c >= 0x80 as libc::c_int && c < 0xa0 as libc::c_int
                                        && (*curr).w_c1 != 0
                                    {
                                        if (*curr).w_FontR as libc::c_int & 0xf0 as libc::c_int
                                            != 0x20 as libc::c_int
                                            || (*curr).w_layer.l_encoding == 8 as libc::c_int
                                        {
                                            match c {
                                                132 | 133 | 136 | 141 | 142 | 143 => {
                                                    DoESC(c ^ 0xc0 as libc::c_int, 0 as libc::c_int);
                                                }
                                                155 => {
                                                    if (*curr).w_autoaka < 0 as libc::c_int {
                                                        (*curr).w_autoaka = 0 as libc::c_int;
                                                    }
                                                    (*curr).w_NumArgs = 0 as libc::c_int;
                                                    (*curr).w_intermediate = 0 as libc::c_int;
                                                    bzero(
                                                        ((*curr).w_args).as_mut_ptr() as *mut libc::c_char
                                                            as *mut libc::c_void,
                                                        (64 as libc::c_int as libc::c_ulong)
                                                            .wrapping_mul(
                                                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                                            ),
                                                    );
                                                    (*curr).w_state = CSI;
                                                }
                                                144 => {
                                                    StringStart(DCS);
                                                }
                                                _ => {}
                                            }
                                            current_block = 2979737022853876585;
                                            continue;
                                        }
                                    }
                                    if (*curr).w_mbcs == 0 {
                                        if c < 0x80 as libc::c_int
                                            || (*curr).w_gr == 0 as libc::c_int
                                        {
                                            (*curr).w_rend.font = (*curr).w_FontL as libc::c_uchar;
                                        } else if (*curr).w_gr == 2 as libc::c_int
                                            && (*curr).w_ss == 0
                                        {
                                            (*curr).w_rend.font = (*curr).w_FontE as libc::c_uchar;
                                        } else {
                                            (*curr).w_rend.font = (*curr).w_FontR as libc::c_uchar;
                                        }
                                    }
                                    if (*curr).w_layer.l_encoding == 8 as libc::c_int {
                                        if (*curr).w_rend.font as libc::c_int == '0' as i32 {
                                            let mut mc: mchar = mchar {
                                                image: 0,
                                                attr: 0,
                                                font: 0,
                                                fontx: 0,
                                                color: 0,
                                                mbcs: 0,
                                            };
                                            let mut mcp: *mut mchar = 0 as *mut mchar;
                                            mc.image = c as libc::c_uchar;
                                            mc.mbcs = 0 as libc::c_int as libc::c_uchar;
                                            mc.font = '0' as i32 as libc::c_uchar;
                                            mc.fontx = 0 as libc::c_int as libc::c_uchar;
                                            mcp = recode_mchar(
                                                &mut mc,
                                                0 as libc::c_int,
                                                8 as libc::c_int,
                                            );
                                            c = (*mcp).image as libc::c_int
                                                | ((*mcp).font as libc::c_int) << 8 as libc::c_int;
                                        }
                                        (*curr).w_rend.font = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if (*curr).w_layer.l_encoding == 8 as libc::c_int
                                        && c >= 0x300 as libc::c_int && utf8_iscomb(c) != 0
                                    {
                                        let mut ox: libc::c_int = 0;
                                        let mut oy: libc::c_int = 0;
                                        let mut omc: mchar = mchar {
                                            image: 0,
                                            attr: 0,
                                            font: 0,
                                            fontx: 0,
                                            color: 0,
                                            mbcs: 0,
                                        };
                                        ox = (*curr).w_layer.l_x - 1 as libc::c_int;
                                        oy = (*curr).w_layer.l_y;
                                        if ox < 0 as libc::c_int {
                                            ox = (*curr).w_layer.l_width - 1 as libc::c_int;
                                            oy -= 1;
                                            oy;
                                        }
                                        if oy < 0 as libc::c_int {
                                            oy = 0 as libc::c_int;
                                        }
                                        omc
                                            .image = *((*((*curr).w_mlines).offset(oy as isize)).image)
                                            .offset(ox as isize);
                                        omc
                                            .attr = *((*((*curr).w_mlines).offset(oy as isize)).attr)
                                            .offset(ox as isize);
                                        omc
                                            .font = *((*((*curr).w_mlines).offset(oy as isize)).font)
                                            .offset(ox as isize);
                                        omc
                                            .fontx = *((*((*curr).w_mlines).offset(oy as isize)).fontx)
                                            .offset(ox as isize);
                                        omc
                                            .color = *((*((*curr).w_mlines).offset(oy as isize)).color)
                                            .offset(ox as isize);
                                        omc.mbcs = 0 as libc::c_int as libc::c_uchar;
                                        if omc.image as libc::c_int == 0xff as libc::c_int
                                            && omc.font as libc::c_int == 0xff as libc::c_int
                                            && omc.fontx as libc::c_int == 0 as libc::c_int
                                        {
                                            ox -= 1;
                                            ox;
                                            if ox >= 0 as libc::c_int {
                                                omc
                                                    .image = *((*((*curr).w_mlines).offset(oy as isize)).image)
                                                    .offset(ox as isize);
                                                omc
                                                    .attr = *((*((*curr).w_mlines).offset(oy as isize)).attr)
                                                    .offset(ox as isize);
                                                omc
                                                    .font = *((*((*curr).w_mlines).offset(oy as isize)).font)
                                                    .offset(ox as isize);
                                                omc
                                                    .fontx = *((*((*curr).w_mlines).offset(oy as isize)).fontx)
                                                    .offset(ox as isize);
                                                omc
                                                    .color = *((*((*curr).w_mlines).offset(oy as isize)).color)
                                                    .offset(ox as isize);
                                                omc.mbcs = 0 as libc::c_int as libc::c_uchar;
                                                omc.mbcs = 0xff as libc::c_int as libc::c_uchar;
                                            }
                                        }
                                        if ox >= 0 as libc::c_int {
                                            utf8_handle_comb(c, &mut omc);
                                            MFixLine(curr, oy, &mut omc);
                                            *((*((*curr).w_mlines).offset(oy as isize)).image)
                                                .offset(ox as isize) = omc.image;
                                            *((*((*curr).w_mlines).offset(oy as isize)).attr)
                                                .offset(ox as isize) = omc.attr;
                                            *((*((*curr).w_mlines).offset(oy as isize)).font)
                                                .offset(ox as isize) = omc.font;
                                            *((*((*curr).w_mlines).offset(oy as isize)).fontx)
                                                .offset(ox as isize) = omc.fontx;
                                            *((*((*curr).w_mlines).offset(oy as isize)).color)
                                                .offset(ox as isize) = omc.color;
                                            LPutChar(&mut (*curr).w_layer, &mut omc, ox, oy);
                                            LGotoPos(
                                                &mut (*curr).w_layer,
                                                (*curr).w_layer.l_x,
                                                (*curr).w_layer.l_y,
                                            );
                                        }
                                        current_block = 2979737022853876585;
                                        continue;
                                    } else {
                                        if (*curr).w_layer.l_encoding == 8 as libc::c_int
                                            && utf8_isdouble(c) != 0
                                        {
                                            (*curr).w_mbcs = 0xff as libc::c_int;
                                        }
                                        font = (*curr).w_rend.font as libc::c_int;
                                        if font == 'I' as i32
                                            && (*curr).w_layer.l_encoding == 2 as libc::c_int
                                            && (*curr).w_mbcs == 0 as libc::c_int
                                        {
                                            if 0x81 as libc::c_int <= c && c <= 0x9f as libc::c_int
                                                || 0xe0 as libc::c_int <= c && c <= 0xef as libc::c_int
                                            {
                                                (*curr).w_mbcs = c;
                                                current_block = 2979737022853876585;
                                                continue;
                                            }
                                        }
                                        if font == 0o31 as libc::c_int && c == 0x80 as libc::c_int
                                            && (*curr).w_mbcs == 0
                                        {
                                            (*curr).w_rend.font = 0 as libc::c_int as libc::c_uchar;
                                            font = (*curr).w_rend.font as libc::c_int;
                                        }
                                        if font != 0
                                            && font & 0x60 as libc::c_int == 0 as libc::c_int
                                            && c == ' ' as i32
                                        {
                                            (*curr).w_rend.font = 0 as libc::c_int as libc::c_uchar;
                                            font = (*curr).w_rend.font as libc::c_int;
                                        }
                                        if font != 0
                                            && font & 0x60 as libc::c_int == 0 as libc::c_int
                                            || (*curr).w_mbcs != 0
                                        {
                                            let mut t: libc::c_int = c;
                                            if (*curr).w_mbcs == 0 as libc::c_int {
                                                (*curr).w_mbcs = c;
                                                current_block = 2979737022853876585;
                                                continue;
                                            } else {
                                                if (*curr).w_layer.l_x == cols - 1 as libc::c_int {
                                                    (*curr).w_layer.l_x
                                                        += if (*curr).w_wrap != 0 {
                                                            1 as libc::c_int
                                                        } else {
                                                            -(1 as libc::c_int)
                                                        };
                                                }
                                                if (*curr).w_layer.l_encoding != 8 as libc::c_int {
                                                    c = (*curr).w_mbcs;
                                                    if font == 'I' as i32
                                                        && (*curr).w_layer.l_encoding == 2 as libc::c_int
                                                    {
                                                        if 0x40 as libc::c_int <= t && t <= 0xfc as libc::c_int
                                                            && t != 0x7f as libc::c_int
                                                        {
                                                            if c <= 0x9f as libc::c_int {
                                                                c = (c - 0x81 as libc::c_int) * 2 as libc::c_int
                                                                    + 0x21 as libc::c_int;
                                                            } else {
                                                                c = (c - 0xc1 as libc::c_int) * 2 as libc::c_int
                                                                    + 0x21 as libc::c_int;
                                                            }
                                                            if t <= 0x7e as libc::c_int {
                                                                t -= 0x1f as libc::c_int;
                                                            } else if t <= 0x9e as libc::c_int {
                                                                t -= 0x20 as libc::c_int;
                                                            } else {
                                                                t -= 0x7e as libc::c_int;
                                                                c += 1;
                                                                c;
                                                            }
                                                            (*curr)
                                                                .w_rend
                                                                .font = ('B' as i32 & 0o37 as libc::c_int) as libc::c_uchar;
                                                        } else {
                                                            c = t;
                                                            t = 0 as libc::c_int;
                                                        }
                                                    }
                                                    if t != 0 && (*curr).w_gr != 0
                                                        && font != 0o30 as libc::c_int
                                                        && font != 0o31 as libc::c_int
                                                    {
                                                        t &= 0x7f as libc::c_int;
                                                        if t < ' ' as i32 {
                                                            current_block = 4650853661393082352;
                                                            continue;
                                                        }
                                                    }
                                                    if t == '\u{7f}' as i32 {
                                                        current_block = 2979737022853876585;
                                                        continue;
                                                    }
                                                    (*curr).w_mbcs = t;
                                                }
                                            }
                                        }
                                        if font == '<' as i32 && c >= ' ' as i32 {
                                            (*curr).w_rend.font = 0 as libc::c_int as libc::c_uchar;
                                            font = (*curr).w_rend.font as libc::c_int;
                                            c |= 0x80 as libc::c_int;
                                        } else if (*curr).w_gr != 0
                                            && (*curr).w_layer.l_encoding != 8 as libc::c_int
                                        {
                                            if c == 0x80 as libc::c_int && font == 0 as libc::c_int
                                                && (*curr).w_layer.l_encoding == 20 as libc::c_int
                                            {
                                                c = 0xa4 as libc::c_int;
                                            } else {
                                                c &= 0x7f as libc::c_int;
                                            }
                                            if c < ' ' as i32 && font != 0o31 as libc::c_int {
                                                current_block = 4650853661393082352;
                                                continue;
                                            }
                                        }
                                        if c == '\u{7f}' as i32 {
                                            current_block = 2979737022853876585;
                                            continue;
                                        }
                                        (*curr).w_rend.image = c as libc::c_uchar;
                                        if (*curr).w_layer.l_encoding == 8 as libc::c_int {
                                            (*curr)
                                                .w_rend
                                                .font = (c >> 8 as libc::c_int) as libc::c_uchar;
                                            (*curr)
                                                .w_rend
                                                .fontx = (c >> 16 as libc::c_int) as libc::c_uchar;
                                        }
                                        (*curr).w_rend.mbcs = (*curr).w_mbcs as libc::c_uchar;
                                        if (*curr).w_layer.l_x < cols - 1 as libc::c_int {
                                            if (*curr).w_insert != 0 {
                                                bcopy(
                                                    (*((*curr).w_mlines).offset((*curr).w_layer.l_y as isize))
                                                        .image as *mut libc::c_char as *const libc::c_void,
                                                    mline_old.image as *mut libc::c_char as *mut libc::c_void,
                                                    cols as size_t,
                                                );
                                                bcopy(
                                                    (*((*curr).w_mlines).offset((*curr).w_layer.l_y as isize))
                                                        .attr as *mut libc::c_char as *const libc::c_void,
                                                    mline_old.attr as *mut libc::c_char as *mut libc::c_void,
                                                    cols as size_t,
                                                );
                                                bcopy(
                                                    (*((*curr).w_mlines).offset((*curr).w_layer.l_y as isize))
                                                        .font as *mut libc::c_char as *const libc::c_void,
                                                    mline_old.font as *mut libc::c_char as *mut libc::c_void,
                                                    cols as size_t,
                                                );
                                                bcopy(
                                                    (*((*curr).w_mlines).offset((*curr).w_layer.l_y as isize))
                                                        .fontx as *mut libc::c_char as *const libc::c_void,
                                                    mline_old.fontx as *mut libc::c_char as *mut libc::c_void,
                                                    cols as size_t,
                                                );
                                                bcopy(
                                                    (*((*curr).w_mlines).offset((*curr).w_layer.l_y as isize))
                                                        .color as *mut libc::c_char as *const libc::c_void,
                                                    mline_old.color as *mut libc::c_char as *mut libc::c_void,
                                                    cols as size_t,
                                                );
                                                MInsChar(
                                                    curr,
                                                    &mut (*curr).w_rend,
                                                    (*curr).w_layer.l_x,
                                                    (*curr).w_layer.l_y,
                                                );
                                                LInsChar(
                                                    &mut (*curr).w_layer,
                                                    &mut (*curr).w_rend,
                                                    (*curr).w_layer.l_x,
                                                    (*curr).w_layer.l_y,
                                                    &mut mline_old,
                                                );
                                                (*curr).w_layer.l_x += 1;
                                                (*curr).w_layer.l_x;
                                            } else {
                                                MPutChar(
                                                    curr,
                                                    &mut (*curr).w_rend,
                                                    (*curr).w_layer.l_x,
                                                    (*curr).w_layer.l_y,
                                                );
                                                LPutChar(
                                                    &mut (*curr).w_layer,
                                                    &mut (*curr).w_rend,
                                                    (*curr).w_layer.l_x,
                                                    (*curr).w_layer.l_y,
                                                );
                                                (*curr).w_layer.l_x += 1;
                                                (*curr).w_layer.l_x;
                                            }
                                        } else if (*curr).w_layer.l_x == cols - 1 as libc::c_int {
                                            MPutChar(
                                                curr,
                                                &mut (*curr).w_rend,
                                                (*curr).w_layer.l_x,
                                                (*curr).w_layer.l_y,
                                            );
                                            LPutChar(
                                                &mut (*curr).w_layer,
                                                &mut (*curr).w_rend,
                                                (*curr).w_layer.l_x,
                                                (*curr).w_layer.l_y,
                                            );
                                            if (*curr).w_wrap != 0 {
                                                (*curr).w_layer.l_x += 1;
                                                (*curr).w_layer.l_x;
                                            }
                                        } else {
                                            MWrapChar(
                                                curr,
                                                &mut (*curr).w_rend,
                                                (*curr).w_layer.l_y,
                                                (*curr).w_top,
                                                (*curr).w_bot,
                                                (*curr).w_insert,
                                            );
                                            LWrapChar(
                                                &mut (*curr).w_layer,
                                                &mut (*curr).w_rend,
                                                (*curr).w_layer.l_y,
                                                (*curr).w_top,
                                                (*curr).w_bot,
                                                (*curr).w_insert,
                                            );
                                            if (*curr).w_layer.l_y != (*curr).w_bot
                                                && (*curr).w_layer.l_y
                                                    != (*curr).w_layer.l_height - 1 as libc::c_int
                                            {
                                                (*curr).w_layer.l_y += 1;
                                                (*curr).w_layer.l_y;
                                            }
                                            (*curr).w_layer.l_x = 1 as libc::c_int;
                                        }
                                        if (*curr).w_mbcs != 0 {
                                            (*curr).w_mbcs = 0 as libc::c_int;
                                            (*curr).w_rend.mbcs = (*curr).w_mbcs as libc::c_uchar;
                                            (*curr).w_layer.l_x += 1;
                                            (*curr).w_layer.l_x;
                                        }
                                        if (*curr).w_ss != 0 {
                                            (*curr)
                                                .w_FontL = (*curr).w_charsets[(*curr).w_Charset as usize]
                                                as libc::c_char;
                                            (*curr)
                                                .w_FontR = (*curr).w_charsets[(*curr).w_CharsetR as usize]
                                                as libc::c_char;
                                            (*curr).w_rend.font = (*curr).w_FontL as libc::c_uchar;
                                            LSetRendition(&mut (*curr).w_layer, &mut (*curr).w_rend);
                                            (*curr).w_ss = 0 as libc::c_int;
                                        }
                                        current_block = 2979737022853876585;
                                        continue;
                                    }
                                }
                            }
                        }
                        match c {
                            92 => {
                                if !(StringEnd() == 0 as libc::c_int
                                    || len <= 1 as libc::c_int)
                                {
                                    cv = (*curr).w_layer.l_cvlist;
                                    while !cv.is_null() {
                                        display = (*cv).c_display;
                                        if (*display).d_status == 1 as libc::c_int {
                                            break;
                                        }
                                        cv = (*cv).c_lnext;
                                    }
                                    if !cv.is_null() {
                                        if len > 4096 as libc::c_int + 1 as libc::c_int {
                                            len = 4096 as libc::c_int + 1 as libc::c_int;
                                        }
                                        (*curr).w_outlen = len - 1 as libc::c_int;
                                        bcopy(
                                            buf as *const libc::c_void,
                                            ((*curr).w_outbuf).as_mut_ptr() as *mut libc::c_void,
                                            (len - 1 as libc::c_int) as size_t,
                                        );
                                        return;
                                    }
                                }
                            }
                            27 => {
                                StringChar('\u{1b}' as i32);
                            }
                            _ => {
                                (*curr).w_state = ASTR;
                                StringChar('\u{1b}' as i32);
                                StringChar(c);
                            }
                        }
                        current_block = 2979737022853876585;
                    }
                }
            }
        }
    }
    if printcmd.is_null()
        && (*curr).w_state as libc::c_uint == PRIN as libc::c_int as libc::c_uint
    {
        PrintFlush();
    }
}
unsafe extern "C" fn WLogString(
    mut p: *mut win,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) {
    if ((*p).w_log).is_null() {
        return;
    }
    if logtstamp_on != 0 && (*p).w_logsilence >= logtstamp_after * 2 as libc::c_int {
        let mut t: *mut libc::c_char = MakeWinMsg(logtstamp_string, p, '%' as i32);
        logfwrite((*p).w_log, t, strlen(t) as libc::c_int);
    }
    (*p).w_logsilence = 0 as libc::c_int;
    if logfwrite((*p).w_log, buf, len) < 1 as libc::c_int {
        WMsg(
            p,
            *__errno_location(),
            b"Error writing logfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        logfclose((*p).w_log);
        (*p).w_log = 0 as *mut logfile;
    }
    if log_flush == 0 {
        logfflush((*p).w_log);
    }
}
unsafe extern "C" fn Special(mut c: libc::c_int) -> libc::c_int {
    's_71: {
        match c {
            8 => {
                BackSpace();
                return 1 as libc::c_int;
            }
            13 => {
                Return();
                return 1 as libc::c_int;
            }
            10 => {
                if (*curr).w_autoaka != 0 {
                    FindAKA();
                }
            }
            11 => {}
            7 => {
                WBell(curr, visual_bell);
                return 1 as libc::c_int;
            }
            9 => {
                ForwardTab();
                return 1 as libc::c_int;
            }
            15 => {
                MapCharset(0 as libc::c_int);
                return 1 as libc::c_int;
            }
            14 => {
                MapCharset(1 as libc::c_int);
                return 1 as libc::c_int;
            }
            _ => {
                break 's_71;
            }
        }
        LineFeed(0 as libc::c_int);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn DoESC(mut c: libc::c_int, mut intermediate: libc::c_int) {
    match intermediate {
        0 => {
            match c {
                69 => {
                    LineFeed(1 as libc::c_int);
                }
                68 => {
                    LineFeed(0 as libc::c_int);
                }
                77 => {
                    ReverseLineFeed();
                }
                72 => {
                    *((*curr).w_tabs)
                        .offset(
                            (*curr).w_layer.l_x as isize,
                        ) = 1 as libc::c_int as libc::c_char;
                }
                90 => {
                    Report(
                        b"\x1B[?%d;%dc\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        1 as libc::c_int,
                        2 as libc::c_int,
                    );
                }
                55 => {
                    SaveCursor(&mut (*curr).w_saved);
                }
                56 => {
                    RestoreCursor(&mut (*curr).w_saved);
                }
                99 => {
                    ClearScreen();
                    ResetWindow(curr);
                    LKeypadMode(&mut (*curr).w_layer, 0 as libc::c_int);
                    LCursorkeysMode(&mut (*curr).w_layer, 0 as libc::c_int);
                    LGotoPos(
                        &mut (*curr).w_layer,
                        (*curr).w_layer.l_x,
                        (*curr).w_layer.l_y,
                    );
                }
                61 => {
                    (*curr).w_keypad = 1 as libc::c_int;
                    LKeypadMode(&mut (*curr).w_layer, (*curr).w_keypad);
                }
                62 => {
                    (*curr).w_keypad = 0 as libc::c_int;
                    LKeypadMode(&mut (*curr).w_layer, (*curr).w_keypad);
                }
                110 => {
                    MapCharset(2 as libc::c_int);
                }
                111 => {
                    MapCharset(3 as libc::c_int);
                }
                126 => {
                    MapCharsetR(1 as libc::c_int);
                }
                125 => {
                    MapCharsetR(2 as libc::c_int);
                }
                124 => {
                    MapCharsetR(3 as libc::c_int);
                }
                78 => {
                    if (*curr).w_charsets[(*curr).w_Charset as usize]
                        != (*curr).w_charsets[2 as libc::c_int as usize]
                        || (*curr).w_charsets[(*curr).w_CharsetR as usize]
                            != (*curr).w_charsets[2 as libc::c_int as usize]
                    {
                        (*curr).w_ss = 2 as libc::c_int;
                        (*curr)
                            .w_FontL = (*curr).w_charsets[(*curr).w_ss as usize]
                            as libc::c_char;
                        (*curr).w_FontR = (*curr).w_FontL;
                    } else {
                        (*curr).w_ss = 0 as libc::c_int;
                    }
                }
                79 => {
                    if (*curr).w_charsets[(*curr).w_Charset as usize]
                        != (*curr).w_charsets[3 as libc::c_int as usize]
                        || (*curr).w_charsets[(*curr).w_CharsetR as usize]
                            != (*curr).w_charsets[3 as libc::c_int as usize]
                    {
                        (*curr).w_ss = 3 as libc::c_int;
                        (*curr)
                            .w_FontL = (*curr).w_charsets[(*curr).w_ss as usize]
                            as libc::c_char;
                        (*curr).w_FontR = (*curr).w_FontL;
                    } else {
                        (*curr).w_ss = 0 as libc::c_int;
                    }
                }
                103 => {
                    WBell(curr, 1 as libc::c_int);
                }
                _ => {}
            }
        }
        35 => {
            match c {
                56 => {
                    FillWithEs();
                }
                _ => {}
            }
        }
        40 => {
            DesignateCharset(c, 0 as libc::c_int);
        }
        41 => {
            DesignateCharset(c, 1 as libc::c_int);
        }
        42 => {
            DesignateCharset(c, 2 as libc::c_int);
        }
        43 => {
            DesignateCharset(c, 3 as libc::c_int);
        }
        36 | 9256 => {
            DesignateCharset(c & 0o37 as libc::c_int, 0 as libc::c_int);
        }
        9257 => {
            DesignateCharset(c & 0o37 as libc::c_int, 1 as libc::c_int);
        }
        9258 => {
            DesignateCharset(c & 0o37 as libc::c_int, 2 as libc::c_int);
        }
        9259 => {
            DesignateCharset(c & 0o37 as libc::c_int, 3 as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn DoCSI(mut c: libc::c_int, mut intermediate: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut a1: libc::c_int = (*curr).w_args[0 as libc::c_int as usize];
    let mut a2: libc::c_int = (*curr).w_args[1 as libc::c_int as usize];
    if (*curr).w_NumArgs > 64 as libc::c_int {
        (*curr).w_NumArgs = 64 as libc::c_int;
    }
    match intermediate {
        0 => {
            match c {
                72 | 102 => {
                    if a1 < 1 as libc::c_int {
                        a1 = 1 as libc::c_int;
                    }
                    if (*curr).w_origin != 0 {
                        a1 += (*curr).w_top;
                    }
                    if a1 > rows {
                        a1 = rows;
                    }
                    if a2 < 1 as libc::c_int {
                        a2 = 1 as libc::c_int;
                    }
                    if a2 > cols {
                        a2 = cols;
                    }
                    a2 -= 1;
                    a1 -= 1;
                    LGotoPos(&mut (*curr).w_layer, a2, a1);
                    (*curr).w_layer.l_x = a2;
                    (*curr).w_layer.l_y = a1;
                    if (*curr).w_autoaka != 0 {
                        (*curr).w_autoaka = a1 + 1 as libc::c_int;
                    }
                }
                74 => {
                    if a1 < 0 as libc::c_int || a1 > 2 as libc::c_int {
                        a1 = 0 as libc::c_int;
                    }
                    match a1 {
                        0 => {
                            ClearToEOS();
                        }
                        1 => {
                            ClearFromBOS();
                        }
                        2 => {
                            ClearScreen();
                            LGotoPos(
                                &mut (*curr).w_layer,
                                (*curr).w_layer.l_x,
                                (*curr).w_layer.l_y,
                            );
                        }
                        _ => {}
                    }
                }
                75 => {
                    if a1 < 0 as libc::c_int || a1 > 2 as libc::c_int {
                        a1 %= 3 as libc::c_int;
                    }
                    match a1 {
                        0 => {
                            ClearLineRegion(
                                (*curr).w_layer.l_x,
                                cols - 1 as libc::c_int,
                            );
                        }
                        1 => {
                            ClearLineRegion(0 as libc::c_int, (*curr).w_layer.l_x);
                        }
                        2 => {
                            ClearLineRegion(0 as libc::c_int, cols - 1 as libc::c_int);
                        }
                        _ => {}
                    }
                }
                88 => {
                    a1 = (*curr).w_layer.l_x
                        + (if a1 != 0 {
                            a1 - 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        });
                    ClearLineRegion(
                        (*curr).w_layer.l_x,
                        if a1 < cols { a1 } else { cols - 1 as libc::c_int },
                    );
                }
                65 => {
                    CursorUp(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                66 => {
                    CursorDown(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                67 => {
                    CursorRight(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                68 => {
                    CursorLeft(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                69 => {
                    (*curr).w_layer.l_x = 0 as libc::c_int;
                    CursorDown(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                70 => {
                    (*curr).w_layer.l_x = 0 as libc::c_int;
                    CursorUp(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                71 | 96 => {
                    (*curr)
                        .w_layer
                        .l_x = if a1 != 0 {
                        a1 - 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if (*curr).w_layer.l_x >= cols {
                        (*curr).w_layer.l_x = cols - 1 as libc::c_int;
                    }
                    LGotoPos(
                        &mut (*curr).w_layer,
                        (*curr).w_layer.l_x,
                        (*curr).w_layer.l_y,
                    );
                }
                100 => {
                    (*curr)
                        .w_layer
                        .l_y = if a1 != 0 {
                        a1 - 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if (*curr).w_layer.l_y >= rows {
                        (*curr).w_layer.l_y = rows - 1 as libc::c_int;
                    }
                    LGotoPos(
                        &mut (*curr).w_layer,
                        (*curr).w_layer.l_x,
                        (*curr).w_layer.l_y,
                    );
                }
                109 => {
                    SelectRendition();
                }
                103 => {
                    if a1 == 0 as libc::c_int {
                        *((*curr).w_tabs)
                            .offset(
                                (*curr).w_layer.l_x as isize,
                            ) = 0 as libc::c_int as libc::c_char;
                    } else if a1 == 3 as libc::c_int {
                        bzero(
                            (*curr).w_tabs as *mut libc::c_void,
                            cols as libc::c_ulong,
                        );
                    }
                }
                114 => {
                    if a1 == 0 {
                        a1 = 1 as libc::c_int;
                    }
                    if a2 == 0 {
                        a2 = rows;
                    }
                    if !(a1 < 1 as libc::c_int || a2 > rows || a1 >= a2) {
                        (*curr).w_top = a1 - 1 as libc::c_int;
                        (*curr).w_bot = a2 - 1 as libc::c_int;
                        if (*curr).w_origin != 0 {
                            (*curr).w_layer.l_y = (*curr).w_top;
                            (*curr).w_layer.l_x = 0 as libc::c_int;
                        } else {
                            (*curr).w_layer.l_x = 0 as libc::c_int;
                            (*curr).w_layer.l_y = (*curr).w_layer.l_x;
                        }
                        LGotoPos(
                            &mut (*curr).w_layer,
                            (*curr).w_layer.l_x,
                            (*curr).w_layer.l_y,
                        );
                    }
                }
                115 => {
                    SaveCursor(&mut (*curr).w_saved);
                }
                116 => {
                    match a1 {
                        11 => {
                            if !((*curr).w_layer.l_cvlist).is_null() {
                                Report(
                                    b"\x1B[1t\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                );
                            } else {
                                Report(
                                    b"\x1B[2t\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                );
                            }
                        }
                        7 => {
                            LRefreshAll(&mut (*curr).w_layer, 0 as libc::c_int);
                        }
                        21 => {
                            a1 = strlen((*curr).w_title) as libc::c_int;
                            if ((*curr).w_inlen + 5 as libc::c_int + a1) as libc::c_uint
                                as libc::c_ulong
                                <= ::std::mem::size_of::<[libc::c_char; 4096]>()
                                    as libc::c_ulong
                            {
                                bcopy(
                                    b"\x1B]l\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    ((*curr).w_inbuf)
                                        .as_mut_ptr()
                                        .offset((*curr).w_inlen as isize) as *mut libc::c_void,
                                    3 as libc::c_int as size_t,
                                );
                                bcopy(
                                    (*curr).w_title as *const libc::c_void,
                                    ((*curr).w_inbuf)
                                        .as_mut_ptr()
                                        .offset((*curr).w_inlen as isize)
                                        .offset(3 as libc::c_int as isize) as *mut libc::c_void,
                                    a1 as size_t,
                                );
                                bcopy(
                                    b"\x1B\\\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    ((*curr).w_inbuf)
                                        .as_mut_ptr()
                                        .offset((*curr).w_inlen as isize)
                                        .offset(3 as libc::c_int as isize)
                                        .offset(a1 as isize) as *mut libc::c_void,
                                    2 as libc::c_int as size_t,
                                );
                                (*curr).w_inlen += 5 as libc::c_int + a1;
                            }
                        }
                        8 => {
                            a1 = (*curr).w_args[2 as libc::c_int as usize];
                            if a1 < 1 as libc::c_int {
                                a1 = (*curr).w_layer.l_width;
                            }
                            if a2 < 1 as libc::c_int {
                                a2 = (*curr).w_layer.l_height;
                            }
                            if !(a1 > 10000 as libc::c_int || a2 > 10000 as libc::c_int)
                            {
                                WChangeSize(curr, a1, a2);
                                cols = (*curr).w_layer.l_width;
                                rows = (*curr).w_layer.l_height;
                            }
                        }
                        _ => {}
                    }
                }
                117 => {
                    RestoreCursor(&mut (*curr).w_saved);
                }
                73 => {
                    if a1 == 0 {
                        a1 = 1 as libc::c_int;
                    }
                    loop {
                        let fresh4 = a1;
                        a1 = a1 - 1;
                        if !(fresh4 != 0) {
                            break;
                        }
                        ForwardTab();
                    }
                }
                90 => {
                    if a1 == 0 {
                        a1 = 1 as libc::c_int;
                    }
                    loop {
                        let fresh5 = a1;
                        a1 = a1 - 1;
                        if !(fresh5 != 0) {
                            break;
                        }
                        BackwardTab();
                    }
                }
                76 => {
                    InsertLine(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                77 => {
                    DeleteLine(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                80 => {
                    DeleteChar(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                64 => {
                    InsertChar(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                104 => {
                    ASetMode(1 as libc::c_int);
                }
                108 => {
                    ASetMode(0 as libc::c_int);
                }
                105 => {
                    if a1 == 5 as libc::c_int {
                        PrintStart();
                    }
                }
                110 => {
                    if a1 == 5 as libc::c_int {
                        Report(
                            b"\x1B[0n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        );
                    } else if a1 == 6 as libc::c_int {
                        Report(
                            b"\x1B[%d;%dR\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*curr).w_layer.l_y + 1 as libc::c_int,
                            (*curr).w_layer.l_x + 1 as libc::c_int,
                        );
                    }
                }
                99 => {
                    if a1 == 0 as libc::c_int {
                        Report(
                            b"\x1B[?%d;%dc\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            1 as libc::c_int,
                            2 as libc::c_int,
                        );
                    }
                }
                120 => {
                    if a1 == 0 as libc::c_int || a1 == 1 as libc::c_int {
                        Report(
                            b"\x1B[%d;1;1;112;112;1;0x\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            a1 + 2 as libc::c_int,
                            0 as libc::c_int,
                        );
                    }
                }
                112 => {
                    if a1 == 6 as libc::c_int || a1 == 7 as libc::c_int {
                        (*curr).w_curinv = 7 as libc::c_int - a1;
                        LCursorVisibility(
                            &mut (*curr).w_layer,
                            if (*curr).w_curinv != 0 {
                                -(1 as libc::c_int)
                            } else {
                                (*curr).w_curvvis
                            },
                        );
                    }
                }
                83 => {
                    ScrollRegion(if a1 != 0 { a1 } else { 1 as libc::c_int });
                }
                84 | 94 => {
                    ScrollRegion(if a1 != 0 { -a1 } else { -(1 as libc::c_int) });
                }
                _ => {}
            }
        }
        63 => {
            a2 = 0 as libc::c_int;
            while a2 < (*curr).w_NumArgs {
                a1 = (*curr).w_args[a2 as usize];
                if c != 'h' as i32 && c != 'l' as i32 {
                    break;
                }
                i = (c == 'h' as i32) as libc::c_int;
                let mut current_block_172: u64;
                match a1 {
                    1 => {
                        (*curr).w_cursorkeys = i;
                        LCursorkeysMode(&mut (*curr).w_layer, (*curr).w_cursorkeys);
                        current_block_172 = 6125050927267093382;
                    }
                    2 => {
                        if i != 0 {
                            if (*curr).w_layer.l_encoding != 0 {
                                current_block_172 = 6125050927267093382;
                            } else {
                                (*curr).w_FontR = 0 as libc::c_int as libc::c_char;
                                (*curr).w_FontL = (*curr).w_FontR;
                                (*curr)
                                    .w_charsets[3 as libc::c_int
                                    as usize] = (*curr).w_FontL as libc::c_int;
                                (*curr)
                                    .w_charsets[2 as libc::c_int
                                    as usize] = (*curr).w_charsets[3 as libc::c_int as usize];
                                (*curr)
                                    .w_charsets[1 as libc::c_int
                                    as usize] = (*curr).w_charsets[2 as libc::c_int as usize];
                                (*curr)
                                    .w_charsets[0 as libc::c_int
                                    as usize] = (*curr).w_charsets[1 as libc::c_int as usize];
                                (*curr).w_Charset = 0 as libc::c_int;
                                (*curr).w_CharsetR = 2 as libc::c_int;
                                (*curr).w_ss = 0 as libc::c_int;
                                current_block_172 = 6125050927267093382;
                            }
                        } else {
                            current_block_172 = 6125050927267093382;
                        }
                    }
                    3 => {
                        i = if i != 0 { Z0width } else { Z1width };
                        ClearScreen();
                        (*curr).w_layer.l_x = 0 as libc::c_int;
                        (*curr).w_layer.l_y = 0 as libc::c_int;
                        WChangeSize(curr, i, (*curr).w_layer.l_height);
                        cols = (*curr).w_layer.l_width;
                        rows = (*curr).w_layer.l_height;
                        current_block_172 = 6125050927267093382;
                    }
                    5 => {
                        if i != (*curr).w_revvid {
                            WReverseVideo(curr, i);
                        }
                        (*curr).w_revvid = i;
                        current_block_172 = 6125050927267093382;
                    }
                    6 => {
                        (*curr).w_origin = i;
                        if (*curr).w_origin != 0 as libc::c_int {
                            (*curr).w_layer.l_y = (*curr).w_top;
                            (*curr).w_layer.l_x = 0 as libc::c_int;
                        } else {
                            (*curr).w_layer.l_x = 0 as libc::c_int;
                            (*curr).w_layer.l_y = (*curr).w_layer.l_x;
                        }
                        LGotoPos(
                            &mut (*curr).w_layer,
                            (*curr).w_layer.l_x,
                            (*curr).w_layer.l_y,
                        );
                        current_block_172 = 6125050927267093382;
                    }
                    7 => {
                        (*curr).w_wrap = i;
                        current_block_172 = 6125050927267093382;
                    }
                    9 => {
                        (*curr)
                            .w_mouse = if i != 0 {
                            9 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        LMouseMode(&mut (*curr).w_layer, (*curr).w_mouse);
                        current_block_172 = 6125050927267093382;
                    }
                    25 => {
                        (*curr).w_curinv = (i == 0) as libc::c_int;
                        LCursorVisibility(
                            &mut (*curr).w_layer,
                            if (*curr).w_curinv != 0 {
                                -(1 as libc::c_int)
                            } else {
                                (*curr).w_curvvis
                            },
                        );
                        current_block_172 = 6125050927267093382;
                    }
                    47 => {
                        current_block_172 = 9142843156959429858;
                    }
                    1047 | 1049 => {
                        current_block_172 = 9142843156959429858;
                    }
                    1048 => {
                        if i != 0 {
                            SaveCursor(&mut (*curr).w_saved);
                        } else {
                            RestoreCursor(&mut (*curr).w_saved);
                        }
                        current_block_172 = 6125050927267093382;
                    }
                    1000 => {
                        current_block_172 = 14694825848963866974;
                    }
                    1001 => {
                        current_block_172 = 14694825848963866974;
                    }
                    1002 | 1003 => {
                        current_block_172 = 17395914038110269350;
                    }
                    1006 => {
                        (*curr).w_extmouse = if i != 0 { a1 } else { 0 as libc::c_int };
                        LExtMouseMode(&mut (*curr).w_layer, (*curr).w_extmouse);
                        current_block_172 = 6125050927267093382;
                    }
                    _ => {
                        current_block_172 = 6125050927267093382;
                    }
                }
                match current_block_172 {
                    9142843156959429858 => {
                        if use_altscreen != 0 {
                            if i != 0 {
                                if (*curr).w_alt.on == 0 {
                                    SaveCursor(&mut (*curr).w_alt.cursor);
                                    EnterAltScreen(curr);
                                }
                            } else if (*curr).w_alt.on != 0 {
                                RestoreCursor(&mut (*curr).w_alt.cursor);
                                LeaveAltScreen(curr);
                            }
                            if a1 == 47 as libc::c_int && i == 0 {
                                (*curr).w_saved.on = 0 as libc::c_int;
                            }
                            LRefreshAll(&mut (*curr).w_layer, 0 as libc::c_int);
                            LGotoPos(
                                &mut (*curr).w_layer,
                                (*curr).w_layer.l_x,
                                (*curr).w_layer.l_y,
                            );
                        }
                        current_block_172 = 6125050927267093382;
                    }
                    14694825848963866974 => {
                        current_block_172 = 17395914038110269350;
                    }
                    _ => {}
                }
                match current_block_172 {
                    17395914038110269350 => {
                        (*curr).w_mouse = if i != 0 { a1 } else { 0 as libc::c_int };
                        LMouseMode(&mut (*curr).w_layer, (*curr).w_mouse);
                    }
                    _ => {}
                }
                a2 += 1;
                a2;
            }
        }
        62 => {
            match c {
                99 => {
                    if a1 == 0 as libc::c_int {
                        Report(
                            b"\x1B[>%d;%d;0c\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            83 as libc::c_int,
                            nversion,
                        );
                    }
                }
                _ => {}
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn StringStart(mut type_0: string_t) {
    (*curr).w_StringType = type_0;
    (*curr).w_stringp = ((*curr).w_string).as_mut_ptr();
    (*curr).w_state = ASTR;
}
unsafe extern "C" fn StringChar(mut c: libc::c_int) {
    if (*curr).w_stringp
        >= ((*curr).w_string)
            .as_mut_ptr()
            .offset(768 as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize))
    {
        (*curr).w_state = LIT;
    } else {
        let fresh6 = (*curr).w_stringp;
        (*curr).w_stringp = ((*curr).w_stringp).offset(1);
        *fresh6 = c as libc::c_char;
    };
}
unsafe extern "C" fn StringEnd() -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut typ: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = (if (*curr).w_state as libc::c_uint == STRESC as libc::c_int as libc::c_uint {
        b"\x1B\\\0" as *const u8 as *const libc::c_char
    } else {
        b"\x07\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    (*curr).w_state = LIT;
    *(*curr).w_stringp = '\0' as i32 as libc::c_char;
    let mut current_block_49: u64;
    match (*curr).w_StringType as libc::c_uint {
        2 => {
            if (*curr).w_string[0 as libc::c_int as usize] as libc::c_int == ';' as i32
                || {
                    p = index(((*curr).w_string).as_mut_ptr(), ';' as i32);
                    p.is_null()
                }
            {
                current_block_49 = 2116367355679836638;
            } else {
                typ = atoi(((*curr).w_string).as_mut_ptr());
                p = p.offset(1);
                p;
                if typ == 83 as libc::c_int {
                    let mut args: [*mut libc::c_char; 64] = [0 as *mut libc::c_char; 64];
                    let mut argl: [libc::c_int; 64] = [0; 64];
                    let mut windowuser: *mut acluser = 0 as *mut acluser;
                    windowuser = *FindUserPtr(
                        b":window:\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    if !windowuser.is_null()
                        && Parse(
                            p,
                            (::std::mem::size_of::<[libc::c_char; 768]>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    p.offset_from(((*curr).w_string).as_mut_ptr())
                                        as libc::c_long as libc::c_ulong,
                                ) as libc::c_int,
                            args.as_mut_ptr(),
                            argl.as_mut_ptr(),
                        ) != 0
                    {
                        display = displays;
                        while !display.is_null() {
                            if (*(*(*display).d_forecv).c_layer).l_bottom
                                == &mut (*curr).w_layer as *mut layer
                            {
                                break;
                            }
                            display = (*display).d_next;
                        }
                        if display.is_null() && !((*curr).w_layer.l_cvlist).is_null() {
                            display = (*(*curr).w_layer.l_cvlist).c_display;
                        }
                        if display.is_null() {
                            display = displays;
                        }
                        EffectiveAclUser = windowuser;
                        fore = curr;
                        flayer = if !((*fore).w_savelayer).is_null() {
                            (*fore).w_savelayer
                        } else {
                            &mut (*fore).w_layer
                        };
                        DoCommand(args.as_mut_ptr(), argl.as_mut_ptr());
                        EffectiveAclUser = 0 as *mut acluser;
                        fore = 0 as *mut win;
                        flayer = 0 as *mut layer;
                    }
                    current_block_49 = 2116367355679836638;
                } else if typ < 0 as libc::c_int || typ > 2 as libc::c_int {
                    current_block_49 = 2116367355679836638;
                } else {
                    (*curr)
                        .w_stringp = ((*curr).w_stringp)
                        .offset(
                            -(p.offset_from(((*curr).w_string).as_mut_ptr())
                                as libc::c_long as isize),
                        );
                    if (*curr).w_stringp > ((*curr).w_string).as_mut_ptr() {
                        bcopy(
                            p as *const libc::c_void,
                            ((*curr).w_string).as_mut_ptr() as *mut libc::c_void,
                            ((*curr).w_stringp)
                                .offset_from(((*curr).w_string).as_mut_ptr())
                                as libc::c_long as size_t,
                        );
                    }
                    *(*curr).w_stringp = '\0' as i32 as libc::c_char;
                    current_block_49 = 4022442114169369898;
                }
            }
        }
        3 => {
            current_block_49 = 4022442114169369898;
        }
        4 | 6 => {
            display = displays;
            while !display.is_null() {
                cv = (*display).d_cvlist;
                while !cv.is_null() {
                    if (*(*cv).c_layer).l_bottom == &mut (*curr).w_layer as *mut layer {
                        break;
                    }
                    cv = (*cv).c_next;
                }
                if !cv.is_null()
                    || (*curr).w_StringType as libc::c_uint
                        == GM as libc::c_int as libc::c_uint
                {
                    MakeStatus(((*curr).w_string).as_mut_ptr());
                }
                display = (*display).d_next;
            }
            return -(1 as libc::c_int);
        }
        1 => {
            let mut olddisplay: *mut display = display;
            let mut cv_0: *mut canvas = 0 as *mut canvas;
            display = displays;
            while !display.is_null() {
                cv_0 = (*display).d_cvlist;
                while !cv_0.is_null() {
                    if (*cv_0).c_layer == &mut (*curr).w_layer as *mut layer {
                        break;
                    }
                    cv_0 = (*cv_0).c_next;
                }
                if !cv_0.is_null() {
                    AddStr(((*curr).w_string).as_mut_ptr());
                }
                display = (*display).d_next;
            }
            display = olddisplay;
            current_block_49 = 2116367355679836638;
        }
        5 => {
            if (*curr).w_title == ((*curr).w_akabuf).as_mut_ptr()
                && *((*curr).w_string).as_mut_ptr() == 0
            {
                current_block_49 = 2116367355679836638;
            } else {
                if (*curr).w_dynamicaka != 0 {
                    ChangeAKA(
                        curr,
                        ((*curr).w_string).as_mut_ptr(),
                        strlen(((*curr).w_string).as_mut_ptr()) as libc::c_int,
                    );
                }
                if *((*curr).w_string).as_mut_ptr() == 0 {
                    (*curr).w_autoaka = (*curr).w_layer.l_y + 1 as libc::c_int;
                }
                current_block_49 = 2116367355679836638;
            }
        }
        _ => {
            current_block_49 = 2116367355679836638;
        }
    }
    match current_block_49 {
        4022442114169369898 => {
            if !((*curr).w_hstatus).is_null() {
                if strcmp((*curr).w_hstatus, ((*curr).w_string).as_mut_ptr())
                    == 0 as libc::c_int
                {
                    current_block_49 = 2116367355679836638;
                } else {
                    free((*curr).w_hstatus as *mut libc::c_void);
                    (*curr).w_hstatus = 0 as *mut libc::c_char;
                    current_block_49 = 5689316957504528238;
                }
            } else {
                current_block_49 = 5689316957504528238;
            }
            match current_block_49 {
                2116367355679836638 => {}
                _ => {
                    if ((*curr).w_string).as_mut_ptr() != (*curr).w_stringp {
                        (*curr).w_hstatus = SaveStr(((*curr).w_string).as_mut_ptr());
                    }
                    WindowChanged(curr, 'h' as i32);
                }
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn PrintStart() {
    (*curr).w_pdisplay = 0 as *mut display;
    display = (*curr).w_lastdisp;
    if !(!display.is_null() && curr == (*display).d_fore
        && (!printcmd.is_null()
            || !((*display).d_tcs[73 as libc::c_int as usize].str_0).is_null()))
    {
        display = displays;
        while !display.is_null() {
            if curr == (*display).d_fore
                && (!printcmd.is_null()
                    || !((*display).d_tcs[73 as libc::c_int as usize].str_0).is_null())
            {
                break;
            }
            display = (*display).d_next;
        }
    }
    if display.is_null() {
        let mut cv: *mut canvas = 0 as *mut canvas;
        cv = (*curr).w_layer.l_cvlist;
        while !cv.is_null() {
            display = (*cv).c_display;
            if !printcmd.is_null()
                || !((*display).d_tcs[73 as libc::c_int as usize].str_0).is_null()
            {
                break;
            }
            cv = (*cv).c_lnext;
        }
        if cv.is_null() {
            display = displays;
            if display.is_null() || !((*display).d_next).is_null()
                || !(!printcmd.is_null()
                    || !((*display).d_tcs[73 as libc::c_int as usize].str_0).is_null())
            {
                return;
            }
        }
    }
    (*curr).w_pdisplay = display;
    (*curr).w_stringp = ((*curr).w_string).as_mut_ptr();
    (*curr).w_state = PRIN;
    if !printcmd.is_null() && (*(*curr).w_pdisplay).d_printfd < 0 as libc::c_int {
        (*(*curr).w_pdisplay).d_printfd = printpipe(curr, printcmd);
    }
}
unsafe extern "C" fn PrintChar(mut c: libc::c_int) {
    if (*curr).w_stringp
        >= ((*curr).w_string)
            .as_mut_ptr()
            .offset(768 as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize))
    {
        PrintFlush();
    }
    let fresh7 = (*curr).w_stringp;
    (*curr).w_stringp = ((*curr).w_stringp).offset(1);
    *fresh7 = c as libc::c_char;
}
unsafe extern "C" fn PrintFlush() {
    display = (*curr).w_pdisplay;
    if !display.is_null() && !printcmd.is_null() {
        let mut bp: *mut libc::c_char = ((*curr).w_string).as_mut_ptr();
        let mut len: libc::c_int = ((*curr).w_stringp)
            .offset_from(((*curr).w_string).as_mut_ptr()) as libc::c_long as libc::c_int;
        let mut r: libc::c_int = 0;
        while len != 0 && (*display).d_printfd >= 0 as libc::c_int {
            r = write((*display).d_printfd, bp as *const libc::c_void, len as size_t)
                as libc::c_int;
            if r <= 0 as libc::c_int {
                WMsg(
                    curr,
                    *__errno_location(),
                    b"printing aborted\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                close((*display).d_printfd);
                (*display).d_printfd = -(1 as libc::c_int);
                break;
            } else {
                bp = bp.offset(r as isize);
                len -= r;
            }
        }
    } else if !display.is_null() && (*curr).w_stringp > ((*curr).w_string).as_mut_ptr() {
        AddCStr((*display).d_tcs[73 as libc::c_int as usize].str_0);
        AddStrn(
            ((*curr).w_string).as_mut_ptr(),
            ((*curr).w_stringp).offset_from(((*curr).w_string).as_mut_ptr())
                as libc::c_long as libc::c_int,
        );
        AddCStr((*display).d_tcs[74 as libc::c_int as usize].str_0);
        Flush(3 as libc::c_int);
    }
    (*curr).w_stringp = ((*curr).w_string).as_mut_ptr();
}
pub unsafe extern "C" fn WNewAutoFlow(mut win: *mut win, mut on: libc::c_int) {
    if (*win).w_flow & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        (*win)
            .w_flow = (1 as libc::c_int) << 2 as libc::c_int
            | ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int) * on;
    } else {
        (*win)
            .w_flow = (*win).w_flow & !((1 as libc::c_int) << 1 as libc::c_int)
            | ((1 as libc::c_int) << 1 as libc::c_int) * on;
    }
    LSetFlow(
        &mut (*win).w_layer,
        (*win).w_flow & (1 as libc::c_int) << 0 as libc::c_int,
    );
}
unsafe extern "C" fn DesignateCharset(mut c: libc::c_int, mut n: libc::c_int) {
    (*curr).w_ss = 0 as libc::c_int;
    if c == '@' as i32 & 0o37 as libc::c_int {
        c = 'B' as i32 & 0o37 as libc::c_int;
    }
    if c == 'B' as i32 {
        c = 0 as libc::c_int;
    }
    if (*curr).w_charsets[n as usize] != c {
        (*curr).w_charsets[n as usize] = c;
        if (*curr).w_Charset == n {
            (*curr).w_FontL = c as libc::c_char;
            (*curr).w_rend.font = (*curr).w_FontL as libc::c_uchar;
            LSetRendition(&mut (*curr).w_layer, &mut (*curr).w_rend);
        }
        if (*curr).w_CharsetR == n {
            (*curr).w_FontR = c as libc::c_char;
        }
    }
}
unsafe extern "C" fn MapCharset(mut n: libc::c_int) {
    (*curr).w_ss = 0 as libc::c_int;
    if (*curr).w_Charset != n {
        (*curr).w_Charset = n;
        (*curr).w_FontL = (*curr).w_charsets[n as usize] as libc::c_char;
        (*curr).w_rend.font = (*curr).w_FontL as libc::c_uchar;
        LSetRendition(&mut (*curr).w_layer, &mut (*curr).w_rend);
    }
}
unsafe extern "C" fn MapCharsetR(mut n: libc::c_int) {
    (*curr).w_ss = 0 as libc::c_int;
    if (*curr).w_CharsetR != n {
        (*curr).w_CharsetR = n;
        (*curr).w_FontR = (*curr).w_charsets[n as usize] as libc::c_char;
    }
    (*curr).w_gr = 1 as libc::c_int;
}
unsafe extern "C" fn SaveCursor(mut cursor: *mut cursor) {
    (*cursor).on = 1 as libc::c_int;
    (*cursor).x = (*curr).w_layer.l_x;
    (*cursor).y = (*curr).w_layer.l_y;
    (*cursor).Rend = (*curr).w_rend;
    (*cursor).Charset = (*curr).w_Charset;
    (*cursor).CharsetR = (*curr).w_CharsetR;
    bcopy(
        ((*curr).w_charsets).as_mut_ptr() as *mut libc::c_char as *const libc::c_void,
        ((*cursor).Charsets).as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
unsafe extern "C" fn RestoreCursor(mut cursor: *mut cursor) {
    if (*cursor).on == 0 {
        return;
    }
    LGotoPos(&mut (*curr).w_layer, (*cursor).x, (*cursor).y);
    (*curr).w_layer.l_x = (*cursor).x;
    (*curr).w_layer.l_y = (*cursor).y;
    (*curr).w_rend = (*cursor).Rend;
    bcopy(
        ((*cursor).Charsets).as_mut_ptr() as *mut libc::c_char as *const libc::c_void,
        ((*curr).w_charsets).as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*curr).w_Charset = (*cursor).Charset;
    (*curr).w_CharsetR = (*cursor).CharsetR;
    (*curr).w_ss = 0 as libc::c_int;
    (*curr).w_FontL = (*curr).w_charsets[(*curr).w_Charset as usize] as libc::c_char;
    (*curr).w_FontR = (*curr).w_charsets[(*curr).w_CharsetR as usize] as libc::c_char;
    LSetRendition(&mut (*curr).w_layer, &mut (*curr).w_rend);
}
unsafe extern "C" fn BackSpace() {
    if (*curr).w_layer.l_x > 0 as libc::c_int {
        (*curr).w_layer.l_x -= 1;
        (*curr).w_layer.l_x;
    } else if (*curr).w_wrap != 0 && (*curr).w_layer.l_y > 0 as libc::c_int {
        (*curr).w_layer.l_x = cols - 1 as libc::c_int;
        (*curr).w_layer.l_y -= 1;
        (*curr).w_layer.l_y;
    }
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn Return() {
    if (*curr).w_layer.l_x == 0 as libc::c_int {
        return;
    }
    (*curr).w_layer.l_x = 0 as libc::c_int;
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn LineFeed(mut out_mode: libc::c_int) {
    if out_mode != 0 {
        (*curr).w_layer.l_x = 0 as libc::c_int;
    }
    if (*curr).w_layer.l_y != (*curr).w_bot {
        if (*curr).w_layer.l_y < rows - 1 as libc::c_int {
            (*curr).w_layer.l_y += 1;
            (*curr).w_layer.l_y;
        }
        LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
        return;
    }
    if (*curr).w_autoaka > 1 as libc::c_int {
        (*curr).w_autoaka -= 1;
        (*curr).w_autoaka;
    }
    MScrollV(
        curr,
        1 as libc::c_int,
        (*curr).w_top,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LScrollV(
        &mut (*curr).w_layer,
        1 as libc::c_int,
        (*curr).w_top,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn ReverseLineFeed() {
    if (*curr).w_layer.l_y == (*curr).w_top {
        MScrollV(
            curr,
            -(1 as libc::c_int),
            (*curr).w_top,
            (*curr).w_bot,
            if (*curr).w_bce != 0 {
                ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                    >> 4 as libc::c_int
                    | (if (*curr).w_rend.attr as libc::c_int
                        & (1 as libc::c_int) << 7 as libc::c_int != 0
                    {
                        0x100 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int
            },
        );
        LScrollV(
            &mut (*curr).w_layer,
            -(1 as libc::c_int),
            (*curr).w_top,
            (*curr).w_bot,
            if (*curr).w_bce != 0 {
                ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                    >> 4 as libc::c_int
                    | (if (*curr).w_rend.attr as libc::c_int
                        & (1 as libc::c_int) << 7 as libc::c_int != 0
                    {
                        0x100 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int
            },
        );
        LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
    } else if (*curr).w_layer.l_y > 0 as libc::c_int {
        CursorUp(1 as libc::c_int);
    }
}
unsafe extern "C" fn InsertChar(mut n: libc::c_int) {
    let mut y: libc::c_int = (*curr).w_layer.l_y;
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    if n <= 0 as libc::c_int {
        return;
    }
    if x == cols {
        x -= 1;
        x;
    }
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).image as *mut libc::c_char
            as *const libc::c_void,
        mline_old.image as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).attr as *mut libc::c_char
            as *const libc::c_void,
        mline_old.attr as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).font as *mut libc::c_char
            as *const libc::c_void,
        mline_old.font as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).fontx as *mut libc::c_char
            as *const libc::c_void,
        mline_old.fontx as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).color as *mut libc::c_char
            as *const libc::c_void,
        mline_old.color as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    MScrollH(
        curr,
        -n,
        y,
        x,
        (*curr).w_layer.l_width - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LScrollH(
        &mut (*curr).w_layer,
        -n,
        y,
        x,
        (*curr).w_layer.l_width - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
        &mut mline_old,
    );
    LGotoPos(&mut (*curr).w_layer, x, y);
}
unsafe extern "C" fn DeleteChar(mut n: libc::c_int) {
    let mut y: libc::c_int = (*curr).w_layer.l_y;
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    if x == cols {
        x -= 1;
        x;
    }
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).image as *mut libc::c_char
            as *const libc::c_void,
        mline_old.image as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).attr as *mut libc::c_char
            as *const libc::c_void,
        mline_old.attr as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).font as *mut libc::c_char
            as *const libc::c_void,
        mline_old.font as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).fontx as *mut libc::c_char
            as *const libc::c_void,
        mline_old.fontx as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    bcopy(
        (*((*curr).w_mlines).offset(y as isize)).color as *mut libc::c_char
            as *const libc::c_void,
        mline_old.color as *mut libc::c_char as *mut libc::c_void,
        cols as size_t,
    );
    MScrollH(
        curr,
        n,
        y,
        x,
        (*curr).w_layer.l_width - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LScrollH(
        &mut (*curr).w_layer,
        n,
        y,
        x,
        (*curr).w_layer.l_width - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
        &mut mline_old,
    );
    LGotoPos(&mut (*curr).w_layer, x, y);
}
unsafe extern "C" fn DeleteLine(mut n: libc::c_int) {
    if (*curr).w_layer.l_y < (*curr).w_top || (*curr).w_layer.l_y > (*curr).w_bot {
        return;
    }
    if n > (*curr).w_bot - (*curr).w_layer.l_y + 1 as libc::c_int {
        n = (*curr).w_bot - (*curr).w_layer.l_y + 1 as libc::c_int;
    }
    MScrollV(
        curr,
        n,
        (*curr).w_layer.l_y,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LScrollV(
        &mut (*curr).w_layer,
        n,
        (*curr).w_layer.l_y,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn InsertLine(mut n: libc::c_int) {
    if (*curr).w_layer.l_y < (*curr).w_top || (*curr).w_layer.l_y > (*curr).w_bot {
        return;
    }
    if n > (*curr).w_bot - (*curr).w_layer.l_y + 1 as libc::c_int {
        n = (*curr).w_bot - (*curr).w_layer.l_y + 1 as libc::c_int;
    }
    MScrollV(
        curr,
        -n,
        (*curr).w_layer.l_y,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LScrollV(
        &mut (*curr).w_layer,
        -n,
        (*curr).w_layer.l_y,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn ScrollRegion(mut n: libc::c_int) {
    MScrollV(
        curr,
        n,
        (*curr).w_top,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LScrollV(
        &mut (*curr).w_layer,
        n,
        (*curr).w_top,
        (*curr).w_bot,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn ForwardTab() {
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    if x == cols {
        LineFeed(1 as libc::c_int);
        x = 0 as libc::c_int;
    }
    if *((*curr).w_tabs).offset(x as isize) as libc::c_int != 0
        && x < cols - 1 as libc::c_int
    {
        x += 1;
        x;
    }
    while x < cols - 1 as libc::c_int && *((*curr).w_tabs).offset(x as isize) == 0 {
        x += 1;
        x;
    }
    (*curr).w_layer.l_x = x;
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn BackwardTab() {
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    if *((*curr).w_tabs).offset(x as isize) as libc::c_int != 0 && x > 0 as libc::c_int {
        x -= 1;
        x;
    }
    while x > 0 as libc::c_int && *((*curr).w_tabs).offset(x as isize) == 0 {
        x -= 1;
        x;
    }
    (*curr).w_layer.l_x = x;
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn ClearScreen() {
    LClearArea(
        &mut (*curr).w_layer,
        0 as libc::c_int,
        0 as libc::c_int,
        (*curr).w_layer.l_width - 1 as libc::c_int,
        (*curr).w_layer.l_height - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
        1 as libc::c_int,
    );
    MScrollV(
        curr,
        (*curr).w_layer.l_height,
        0 as libc::c_int,
        (*curr).w_layer.l_height - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
}
unsafe extern "C" fn ClearFromBOS() {
    let mut y: libc::c_int = (*curr).w_layer.l_y;
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    LClearArea(
        &mut (*curr).w_layer,
        0 as libc::c_int,
        0 as libc::c_int,
        x,
        y,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
        1 as libc::c_int,
    );
    MClearArea(
        curr,
        0 as libc::c_int,
        0 as libc::c_int,
        x,
        y,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    RestorePosRendition();
}
unsafe extern "C" fn ClearToEOS() {
    let mut y: libc::c_int = (*curr).w_layer.l_y;
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    if x == 0 as libc::c_int && y == 0 as libc::c_int {
        ClearScreen();
        RestorePosRendition();
        return;
    }
    LClearArea(
        &mut (*curr).w_layer,
        x,
        y,
        cols - 1 as libc::c_int,
        rows - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
        1 as libc::c_int,
    );
    MClearArea(
        curr,
        x,
        y,
        cols - 1 as libc::c_int,
        rows - 1 as libc::c_int,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    RestorePosRendition();
}
unsafe extern "C" fn ClearLineRegion(mut from: libc::c_int, mut to: libc::c_int) {
    let mut y: libc::c_int = (*curr).w_layer.l_y;
    LClearArea(
        &mut (*curr).w_layer,
        from,
        y,
        to,
        y,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
        1 as libc::c_int,
    );
    MClearArea(
        curr,
        from,
        y,
        to,
        y,
        if (*curr).w_bce != 0 {
            ((*curr).w_rend.color as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int
                | (if (*curr).w_rend.attr as libc::c_int
                    & (1 as libc::c_int) << 7 as libc::c_int != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        } else {
            0 as libc::c_int
        },
    );
    RestorePosRendition();
}
unsafe extern "C" fn CursorRight(mut n: libc::c_int) {
    let mut x: libc::c_int = (*curr).w_layer.l_x;
    if x == cols {
        LineFeed(1 as libc::c_int);
        x = 0 as libc::c_int;
    }
    (*curr).w_layer.l_x += n;
    if (*curr).w_layer.l_x >= cols {
        (*curr).w_layer.l_x = cols - 1 as libc::c_int;
    }
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn CursorUp(mut n: libc::c_int) {
    if (*curr).w_layer.l_y < (*curr).w_top {
        (*curr).w_layer.l_y -= n;
        if (*curr).w_layer.l_y < 0 as libc::c_int {
            (*curr).w_layer.l_y = 0 as libc::c_int;
        }
    } else {
        (*curr).w_layer.l_y -= n;
        if (*curr).w_layer.l_y < (*curr).w_top {
            (*curr).w_layer.l_y = (*curr).w_top;
        }
    }
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn CursorDown(mut n: libc::c_int) {
    if (*curr).w_layer.l_y > (*curr).w_bot {
        (*curr).w_layer.l_y += n;
        if (*curr).w_layer.l_y > rows - 1 as libc::c_int {
            (*curr).w_layer.l_y = rows - 1 as libc::c_int;
        }
    } else {
        (*curr).w_layer.l_y += n;
        if (*curr).w_layer.l_y > (*curr).w_bot {
            (*curr).w_layer.l_y = (*curr).w_bot;
        }
    }
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn CursorLeft(mut n: libc::c_int) {
    (*curr).w_layer.l_x -= n;
    if (*curr).w_layer.l_x < 0 as libc::c_int {
        (*curr).w_layer.l_x = 0 as libc::c_int;
    }
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
}
unsafe extern "C" fn ASetMode(mut on: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*curr).w_NumArgs {
        match (*curr).w_args[i as usize] {
            4 => {
                (*curr).w_insert = on;
                let mut olddisplay: *mut display = display;
                let mut cv: *mut canvas = 0 as *mut canvas;
                display = displays;
                while !display.is_null() {
                    cv = (*display).d_cvlist;
                    while !cv.is_null() {
                        if (*cv).c_layer == &mut (*curr).w_layer as *mut layer {
                            break;
                        }
                        cv = (*cv).c_next;
                    }
                    if !cv.is_null() {
                        InsertMode(on);
                    }
                    display = (*display).d_next;
                }
                display = olddisplay;
            }
            20 => {
                (*curr).w_autolf = on;
            }
            34 => {
                (*curr).w_curvvis = (on == 0) as libc::c_int;
                LCursorVisibility(
                    &mut (*curr).w_layer,
                    if (*curr).w_curinv != 0 {
                        -(1 as libc::c_int)
                    } else {
                        (*curr).w_curvvis
                    },
                );
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
static mut rendlist: [libc::c_char; 28] = [
    !(((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int) as libc::c_char,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_char,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_char,
    ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_char,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_char,
    ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_char,
    0 as libc::c_int as libc::c_char,
    ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    !((1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_char,
    !((1 as libc::c_int) << 4 as libc::c_int) as libc::c_char,
    !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_char,
    !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_char,
    0 as libc::c_int as libc::c_char,
    !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_char,
];
unsafe extern "C" fn SelectRendition() {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = (*curr).w_rend.attr as libc::c_int;
    let mut c: libc::c_int = (*curr).w_rend.color as libc::c_int;
    let mut current_block_28: u64;
    loop {
        j = (*curr).w_args[i as usize];
        if (j == 38 as libc::c_int || j == 48 as libc::c_int)
            && (i + 2 as libc::c_int) < (*curr).w_NumArgs
            && (*curr).w_args[(i + 1 as libc::c_int) as usize] == 5 as libc::c_int
        {
            let mut jj: libc::c_int = 0;
            i += 2 as libc::c_int;
            jj = (*curr).w_args[i as usize];
            if jj < 0 as libc::c_int || jj > 255 as libc::c_int {
                current_block_28 = 792017965103506125;
            } else {
                jj = color256to16(jj) + 30 as libc::c_int;
                if jj >= 38 as libc::c_int {
                    jj += 60 as libc::c_int - 8 as libc::c_int;
                }
                j = if j == 38 as libc::c_int { jj } else { jj + 10 as libc::c_int };
                current_block_28 = 17216689946888361452;
            }
        } else {
            current_block_28 = 17216689946888361452;
        }
        match current_block_28 {
            17216689946888361452 => {
                if j == 0 as libc::c_int
                    || j >= 30 as libc::c_int && j <= 39 as libc::c_int
                        && j != 38 as libc::c_int
                {
                    a &= 0xbf as libc::c_int;
                }
                if j == 0 as libc::c_int
                    || j >= 40 as libc::c_int && j <= 49 as libc::c_int
                        && j != 48 as libc::c_int
                {
                    a &= 0x7f as libc::c_int;
                }
                if j >= 90 as libc::c_int && j <= 97 as libc::c_int {
                    a |= 0x40 as libc::c_int;
                }
                if j >= 100 as libc::c_int && j <= 107 as libc::c_int {
                    a |= 0x80 as libc::c_int;
                }
                if j >= 90 as libc::c_int && j <= 97 as libc::c_int {
                    j -= 60 as libc::c_int;
                }
                if j >= 100 as libc::c_int && j <= 107 as libc::c_int {
                    j -= 60 as libc::c_int;
                }
                if j >= 30 as libc::c_int && j <= 39 as libc::c_int
                    && j != 38 as libc::c_int
                {
                    c = c & 0xf0 as libc::c_int
                        | j - 30 as libc::c_int ^ 9 as libc::c_int;
                } else if j >= 40 as libc::c_int && j <= 49 as libc::c_int
                    && j != 48 as libc::c_int
                {
                    c = c & 0xf as libc::c_int
                        | (j - 40 as libc::c_int ^ 9 as libc::c_int) << 4 as libc::c_int;
                }
                if j == 0 as libc::c_int {
                    c = 0 as libc::c_int;
                }
                if !(j < 0 as libc::c_int
                    || j
                        >= (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int)
                {
                    j = rendlist[j as usize] as libc::c_int;
                    if j & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                        a &= j;
                    } else {
                        a |= j;
                    }
                }
            }
            _ => {}
        }
        i += 1;
        if !(i < (*curr).w_NumArgs) {
            break;
        }
    }
    (*curr).w_rend.attr = a as libc::c_uchar;
    (*curr).w_rend.color = c as libc::c_uchar;
    LSetRendition(&mut (*curr).w_layer, &mut (*curr).w_rend);
}
unsafe extern "C" fn FillWithEs() {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    LClearAll(&mut (*curr).w_layer, 1 as libc::c_int);
    (*curr).w_layer.l_x = 0 as libc::c_int;
    (*curr).w_layer.l_y = (*curr).w_layer.l_x;
    i = 0 as libc::c_int;
    while i < rows {
        bclear(
            ((*((*curr).w_mlines).offset(i as isize)).image as *mut libc::c_char)
                .offset(0 as libc::c_int as isize),
            cols + 1 as libc::c_int,
        );
        if (*((*curr).w_mlines).offset(i as isize)).attr != null {
            bzero(
                ((*((*curr).w_mlines).offset(i as isize)).attr as *mut libc::c_char)
                    .offset(0 as libc::c_int as isize) as *mut libc::c_void,
                (cols + 1 as libc::c_int) as libc::c_ulong,
            );
        }
        if (*((*curr).w_mlines).offset(i as isize)).font != null {
            bzero(
                ((*((*curr).w_mlines).offset(i as isize)).font as *mut libc::c_char)
                    .offset(0 as libc::c_int as isize) as *mut libc::c_void,
                (cols + 1 as libc::c_int) as libc::c_ulong,
            );
        }
        if (*((*curr).w_mlines).offset(i as isize)).fontx != null {
            bzero(
                ((*((*curr).w_mlines).offset(i as isize)).fontx as *mut libc::c_char)
                    .offset(0 as libc::c_int as isize) as *mut libc::c_void,
                (cols + 1 as libc::c_int) as libc::c_ulong,
            );
        }
        if (*((*curr).w_mlines).offset(i as isize)).color != null {
            bzero(
                ((*((*curr).w_mlines).offset(i as isize)).color as *mut libc::c_char)
                    .offset(0 as libc::c_int as isize) as *mut libc::c_void,
                (cols + 1 as libc::c_int) as libc::c_ulong,
            );
        }
        p = (*((*curr).w_mlines).offset(i as isize)).image;
        ep = p.offset(cols as isize);
        while p < ep {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = 'E' as i32 as libc::c_uchar;
        }
        i += 1;
        i;
    }
    LRefreshAll(&mut (*curr).w_layer, 1 as libc::c_int);
}
pub unsafe extern "C" fn ChangeAKA(
    mut p: *mut win,
    mut s: *mut libc::c_char,
    mut l: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    i = 0 as libc::c_int;
    while l > 0 as libc::c_int {
        if ((*p).w_akachange).offset(i as isize)
            == ((*p).w_akabuf)
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong
                        as isize,
                )
                .offset(-(1 as libc::c_int as isize))
        {
            break;
        }
        let fresh9 = s;
        s = s.offset(1);
        c = *fresh9 as libc::c_uchar as libc::c_int;
        if c == 0 as libc::c_int {
            break;
        }
        if !(c < 32 as libc::c_int || c == 127 as libc::c_int
            || c >= 128 as libc::c_int && c < 160 as libc::c_int && (*p).w_c1 != 0)
        {
            let fresh10 = i;
            i = i + 1;
            *((*p).w_akachange).offset(fresh10 as isize) = c as libc::c_char;
        }
        l -= 1;
        l;
    }
    *((*p).w_akachange).offset(i as isize) = 0 as libc::c_int as libc::c_char;
    (*p).w_title = (*p).w_akachange;
    if (*p).w_akachange != ((*p).w_akabuf).as_mut_ptr() {
        if *((*p).w_akachange).offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
            || *((*p).w_akachange).offset(-(1 as libc::c_int) as isize) as libc::c_int
                == ':' as i32
        {
            (*p)
                .w_title = ((*p).w_akabuf)
                .as_mut_ptr()
                .offset(strlen(((*p).w_akabuf).as_mut_ptr()) as isize)
                .offset(1 as libc::c_int as isize);
        }
    }
    WindowChanged(p, 't' as i32);
    WindowChanged(0 as *mut win, 'w' as i32);
    WindowChanged(0 as *mut win, 'W' as i32);
}
unsafe extern "C" fn FindAKA() {
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut line: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wp: *mut win = curr;
    let mut len: libc::c_int = strlen(((*wp).w_akabuf).as_mut_ptr()) as libc::c_int;
    let mut y: libc::c_int = 0;
    y = if (*wp).w_autoaka > 0 as libc::c_int
        && (*wp).w_autoaka <= (*wp).w_layer.l_height
    {
        (*wp).w_autoaka - 1 as libc::c_int
    } else {
        (*wp).w_layer.l_y
    };
    cols = (*wp).w_layer.l_width;
    '_try_line: loop {
        line = (*((*wp).w_mlines).offset(y as isize)).image;
        cp = line;
        if !((*wp).w_autoaka > 0 as libc::c_int
            && *((*wp).w_akabuf).as_mut_ptr() as libc::c_int != '\0' as i32)
        {
            break;
        }
        loop {
            if cp.offset_from(line) as libc::c_long >= (cols - len) as libc::c_long {
                y += 1;
                if y == (*wp).w_autoaka && y < rows {
                    continue '_try_line;
                }
                return;
            } else {
                if strncmp(
                    cp as *mut libc::c_char,
                    ((*wp).w_akabuf).as_mut_ptr(),
                    len as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    break;
                }
                cp = cp.offset(1);
                cp;
            }
        }
        cp = cp.offset(len as isize);
        break;
    }
    len = (cols as libc::c_long - cp.offset_from(line) as libc::c_long) as libc::c_int;
    while len != 0 && *cp as libc::c_int == ' ' as i32 {
        len -= 1;
        len;
        cp = cp.offset(1);
        cp;
    }
    if len != 0 {
        if (*wp).w_autoaka > 0 as libc::c_int
            && (*cp as libc::c_int == '!' as i32 || *cp as libc::c_int == '%' as i32
                || *cp as libc::c_int == '^' as i32)
        {
            (*wp).w_autoaka = -(1 as libc::c_int);
        } else {
            (*wp).w_autoaka = 0 as libc::c_int;
        }
        line = cp;
        while len != 0 && *cp as libc::c_int != ' ' as i32 {
            let fresh11 = cp;
            cp = cp.offset(1);
            if *fresh11 as libc::c_int == '/' as i32 {
                line = cp;
            }
            len -= 1;
            len;
        }
        ChangeAKA(
            wp,
            line as *mut libc::c_char,
            cp.offset_from(line) as libc::c_long as libc::c_int,
        );
    } else {
        (*wp).w_autoaka = 0 as libc::c_int;
    };
}
unsafe extern "C" fn RestorePosRendition() {
    LGotoPos(&mut (*curr).w_layer, (*curr).w_layer.l_x, (*curr).w_layer.l_y);
    LSetRendition(&mut (*curr).w_layer, &mut (*curr).w_rend);
}
unsafe extern "C" fn Report(
    mut fmt: *mut libc::c_char,
    mut n1: libc::c_int,
    mut n2: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    let mut rbuf: [libc::c_char; 40] = [0; 40];
    sprintf(rbuf.as_mut_ptr(), fmt, n1, n2);
    len = strlen(rbuf.as_mut_ptr()) as libc::c_int;
    if !((*curr).w_pwin).is_null()
        && (*(*curr).w_pwin).p_fdpat & 0x1000 as libc::c_int != 0
    {
        if ((*(*curr).w_pwin).p_inlen + len) as libc::c_uint as libc::c_ulong
            <= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        {
            bcopy(
                rbuf.as_mut_ptr() as *const libc::c_void,
                ((*(*curr).w_pwin).p_inbuf)
                    .as_mut_ptr()
                    .offset((*(*curr).w_pwin).p_inlen as isize) as *mut libc::c_void,
                len as size_t,
            );
            (*(*curr).w_pwin).p_inlen += len;
        }
    } else if ((*curr).w_inlen + len) as libc::c_uint as libc::c_ulong
        <= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
    {
        bcopy(
            rbuf.as_mut_ptr() as *const libc::c_void,
            ((*curr).w_inbuf).as_mut_ptr().offset((*curr).w_inlen as isize)
                as *mut libc::c_void,
            len as size_t,
        );
        (*curr).w_inlen += len;
    }
}
unsafe extern "C" fn MFixLine(mut p: *mut win, mut y: libc::c_int, mut mc: *mut mchar) {
    let mut ml: *mut mline = &mut *((*p).w_mlines).offset(y as isize) as *mut mline;
    if (*mc).attr as libc::c_int != 0 && (*ml).attr == null {
        (*ml)
            .attr = calloc(
            ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if ((*ml).attr).is_null() {
            (*ml).attr = null;
            (*p).w_rend.attr = 0 as libc::c_int as libc::c_uchar;
            (*mc).attr = (*p).w_rend.attr;
            WMsg(
                p,
                0 as libc::c_int,
                b"Warning: no space for attr - turned off\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if (*mc).font as libc::c_int != 0 && (*ml).font == null {
        (*ml)
            .font = calloc(
            ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if ((*ml).font).is_null() {
            (*ml).font = null;
            (*p)
                .w_charsets[(if (*p).w_ss != 0 { (*p).w_ss } else { (*p).w_Charset })
                as usize] = 0 as libc::c_int;
            (*p)
                .w_FontL = (*p)
                .w_charsets[(if (*p).w_ss != 0 { (*p).w_ss } else { (*p).w_Charset })
                as usize] as libc::c_char;
            (*p)
                .w_charsets[(if (*p).w_ss != 0 { (*p).w_ss } else { (*p).w_CharsetR })
                as usize] = 0 as libc::c_int;
            (*p)
                .w_FontR = (*p)
                .w_charsets[(if (*p).w_ss != 0 { (*p).w_ss } else { (*p).w_CharsetR })
                as usize] as libc::c_char;
            (*p).w_rend.font = 0 as libc::c_int as libc::c_uchar;
            (*mc).fontx = (*p).w_rend.font;
            (*mc).font = (*mc).fontx;
            WMsg(
                p,
                0 as libc::c_int,
                b"Warning: no space for font - turned off\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if (*mc).fontx as libc::c_int != 0 && (*ml).fontx == null {
        (*ml)
            .fontx = calloc(
            ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if ((*ml).fontx).is_null() {
            (*ml).fontx = null;
            (*mc).fontx = 0 as libc::c_int as libc::c_uchar;
        }
    }
    if (*mc).color as libc::c_int != 0 && (*ml).color == null {
        (*ml)
            .color = calloc(
            ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if ((*ml).color).is_null() {
            (*ml).color = null;
            (*p).w_rend.color = 0 as libc::c_int as libc::c_uchar;
            (*mc).color = (*p).w_rend.color;
            WMsg(
                p,
                0 as libc::c_int,
                b"Warning: no space for color - turned off\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn MScrollH(
    mut p: *mut win,
    mut n: libc::c_int,
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut ml: *mut mline = 0 as *mut mline;
    if n == 0 as libc::c_int {
        return;
    }
    ml = &mut *((*p).w_mlines).offset(y as isize) as *mut mline;
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset(xs as isize) as libc::c_int == 0xff as libc::c_int
            && *((*ml).image).offset(xs as isize) as libc::c_int == 0xff as libc::c_int)
            as libc::c_int
    } else {
        (*((*ml).font).offset(xs as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0x80 as libc::c_int) as libc::c_int
    } != 0
    {
        if xs > 0 as libc::c_int {
            *((*ml).image).offset((xs - 1 as libc::c_int) as isize) = mchar_blank.image;
            *((*ml).attr).offset((xs - 1 as libc::c_int) as isize) = mchar_blank.attr;
            *((*ml).font).offset((xs - 1 as libc::c_int) as isize) = mchar_blank.font;
            *((*ml).fontx).offset((xs - 1 as libc::c_int) as isize) = mchar_blank.fontx;
            *((*ml).color).offset((xs - 1 as libc::c_int) as isize) = mchar_blank.color;
        }
        *((*ml).image).offset(xs as isize) = mchar_blank.image;
        *((*ml).attr).offset(xs as isize) = mchar_blank.attr;
        *((*ml).font).offset(xs as isize) = mchar_blank.font;
        *((*ml).fontx).offset(xs as isize) = mchar_blank.fontx;
        *((*ml).color).offset(xs as isize) = mchar_blank.color;
    }
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset((xe + 1 as libc::c_int) as isize) as libc::c_int
            == 0xff as libc::c_int
            && *((*ml).image).offset((xe + 1 as libc::c_int) as isize) as libc::c_int
                == 0xff as libc::c_int) as libc::c_int
    } else {
        (*((*ml).font).offset(xe as isize) as libc::c_int & 0x1f as libc::c_int
            != 0 as libc::c_int
            && *((*ml).font).offset(xe as isize) as libc::c_int & 0xe0 as libc::c_int
                == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        *((*ml).image).offset(xe as isize) = mchar_blank.image;
        *((*ml).attr).offset(xe as isize) = mchar_blank.attr;
        *((*ml).font).offset(xe as isize) = mchar_blank.font;
        *((*ml).fontx).offset(xe as isize) = mchar_blank.fontx;
        *((*ml).color).offset(xe as isize) = mchar_blank.color;
        *((*ml).image).offset((xe + 1 as libc::c_int) as isize) = mchar_blank.image;
        *((*ml).attr).offset((xe + 1 as libc::c_int) as isize) = mchar_blank.attr;
        *((*ml).font).offset((xe + 1 as libc::c_int) as isize) = mchar_blank.font;
        *((*ml).fontx).offset((xe + 1 as libc::c_int) as isize) = mchar_blank.fontx;
        *((*ml).color).offset((xe + 1 as libc::c_int) as isize) = mchar_blank.color;
    }
    if n > 0 as libc::c_int {
        if xe - xs + 1 as libc::c_int > n {
            if if (*p).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset((xs + n) as isize) as libc::c_int
                    == 0xff as libc::c_int
                    && *((*ml).image).offset((xs + n) as isize) as libc::c_int
                        == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset((xs + n) as isize) as libc::c_int
                    & 0xe0 as libc::c_int == 0x80 as libc::c_int) as libc::c_int
            } != 0
            {
                if xs + n > 0 as libc::c_int {
                    *((*ml).image)
                        .offset(
                            (xs + n - 1 as libc::c_int) as isize,
                        ) = mchar_blank.image;
                    *((*ml).attr)
                        .offset((xs + n - 1 as libc::c_int) as isize) = mchar_blank.attr;
                    *((*ml).font)
                        .offset((xs + n - 1 as libc::c_int) as isize) = mchar_blank.font;
                    *((*ml).fontx)
                        .offset(
                            (xs + n - 1 as libc::c_int) as isize,
                        ) = mchar_blank.fontx;
                    *((*ml).color)
                        .offset(
                            (xs + n - 1 as libc::c_int) as isize,
                        ) = mchar_blank.color;
                }
                *((*ml).image).offset((xs + n) as isize) = mchar_blank.image;
                *((*ml).attr).offset((xs + n) as isize) = mchar_blank.attr;
                *((*ml).font).offset((xs + n) as isize) = mchar_blank.font;
                *((*ml).fontx).offset((xs + n) as isize) = mchar_blank.fontx;
                *((*ml).color).offset((xs + n) as isize) = mchar_blank.color;
            }
            bcopy(
                ((*ml).image as *mut libc::c_char).offset((xs + n) as isize)
                    as *const libc::c_void,
                ((*ml).image as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).attr as *mut libc::c_char).offset((xs + n) as isize)
                    as *const libc::c_void,
                ((*ml).attr as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).font as *mut libc::c_char).offset((xs + n) as isize)
                    as *const libc::c_void,
                ((*ml).font as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).fontx as *mut libc::c_char).offset((xs + n) as isize)
                    as *const libc::c_void,
                ((*ml).fontx as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).color as *mut libc::c_char).offset((xs + n) as isize)
                    as *const libc::c_void,
                ((*ml).color as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
        } else {
            n = xe - xs + 1 as libc::c_int;
        }
        bclear(
            ((*ml).image as *mut libc::c_char)
                .offset((xe + 1 as libc::c_int - n) as isize),
            n,
        );
        if (*ml).attr != null {
            bzero(
                ((*ml).attr as *mut libc::c_char)
                    .offset((xe + 1 as libc::c_int - n) as isize) as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if (*ml).font != null {
            bzero(
                ((*ml).font as *mut libc::c_char)
                    .offset((xe + 1 as libc::c_int - n) as isize) as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if (*ml).fontx != null {
            bzero(
                ((*ml).fontx as *mut libc::c_char)
                    .offset((xe + 1 as libc::c_int - n) as isize) as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if (*ml).color != null {
            bzero(
                ((*ml).color as *mut libc::c_char)
                    .offset((xe + 1 as libc::c_int - n) as isize) as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if bce != 0 {
            MBceLine(p, y, xe + 1 as libc::c_int - n, n, bce);
        }
    } else {
        n = -n;
        if xe - xs + 1 as libc::c_int > n {
            if if (*p).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset((xe - n + 1 as libc::c_int) as isize)
                    as libc::c_int == 0xff as libc::c_int
                    && *((*ml).image).offset((xe - n + 1 as libc::c_int) as isize)
                        as libc::c_int == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset((xe - n) as isize) as libc::c_int
                    & 0x1f as libc::c_int != 0 as libc::c_int
                    && *((*ml).font).offset((xe - n) as isize) as libc::c_int
                        & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
            } != 0
            {
                *((*ml).image).offset((xe - n) as isize) = mchar_blank.image;
                *((*ml).attr).offset((xe - n) as isize) = mchar_blank.attr;
                *((*ml).font).offset((xe - n) as isize) = mchar_blank.font;
                *((*ml).fontx).offset((xe - n) as isize) = mchar_blank.fontx;
                *((*ml).color).offset((xe - n) as isize) = mchar_blank.color;
                *((*ml).image)
                    .offset((xe - n + 1 as libc::c_int) as isize) = mchar_blank.image;
                *((*ml).attr)
                    .offset((xe - n + 1 as libc::c_int) as isize) = mchar_blank.attr;
                *((*ml).font)
                    .offset((xe - n + 1 as libc::c_int) as isize) = mchar_blank.font;
                *((*ml).fontx)
                    .offset((xe - n + 1 as libc::c_int) as isize) = mchar_blank.fontx;
                *((*ml).color)
                    .offset((xe - n + 1 as libc::c_int) as isize) = mchar_blank.color;
            }
            bcopy(
                ((*ml).image as *mut libc::c_char).offset(xs as isize)
                    as *const libc::c_void,
                ((*ml).image as *mut libc::c_char).offset((xs + n) as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).attr as *mut libc::c_char).offset(xs as isize)
                    as *const libc::c_void,
                ((*ml).attr as *mut libc::c_char).offset((xs + n) as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).font as *mut libc::c_char).offset(xs as isize)
                    as *const libc::c_void,
                ((*ml).font as *mut libc::c_char).offset((xs + n) as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).fontx as *mut libc::c_char).offset(xs as isize)
                    as *const libc::c_void,
                ((*ml).fontx as *mut libc::c_char).offset((xs + n) as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
            bcopy(
                ((*ml).color as *mut libc::c_char).offset(xs as isize)
                    as *const libc::c_void,
                ((*ml).color as *mut libc::c_char).offset((xs + n) as isize)
                    as *mut libc::c_void,
                (xe + 1 as libc::c_int - xs - n) as size_t,
            );
        } else {
            n = xe - xs + 1 as libc::c_int;
        }
        bclear(((*ml).image as *mut libc::c_char).offset(xs as isize), n);
        if (*ml).attr != null {
            bzero(
                ((*ml).attr as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if (*ml).font != null {
            bzero(
                ((*ml).font as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if (*ml).fontx != null {
            bzero(
                ((*ml).fontx as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if (*ml).color != null {
            bzero(
                ((*ml).color as *mut libc::c_char).offset(xs as isize)
                    as *mut libc::c_void,
                n as libc::c_ulong,
            );
        }
        if bce != 0 {
            MBceLine(p, y, xs, n, bce);
        }
    };
}
unsafe extern "C" fn MScrollV(
    mut p: *mut win,
    mut n: libc::c_int,
    mut ys: libc::c_int,
    mut ye: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut cnt1: libc::c_int = 0;
    let mut cnt2: libc::c_int = 0;
    let mut tmp: [mline; 256] = [mline {
        image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    }; 256];
    let mut ml: *mut mline = 0 as *mut mline;
    if n == 0 as libc::c_int {
        return;
    }
    if n > 0 as libc::c_int {
        if (ye - ys + 1 as libc::c_int) < n {
            n = ye - ys + 1 as libc::c_int;
        }
        if n > 256 as libc::c_int {
            MScrollV(p, n - 256 as libc::c_int, ys, ye, bce);
            n = 256 as libc::c_int;
        }
        if compacthist != 0 {
            ye = MFindUsedLine(p, ye, ys);
            if (ye - ys + 1 as libc::c_int) < n {
                n = ye - ys + 1 as libc::c_int;
            }
            if n <= 0 as libc::c_int {
                return;
            }
        }
        ml = ((*p).w_mlines).offset(ys as isize);
        i = ys;
        while i < ys + n {
            if ys == (*p).w_top {
                WAddLineToHist(p, ml);
            }
            if (*ml).attr != null {
                free((*ml).attr as *mut libc::c_void);
            }
            (*ml).attr = null;
            if (*ml).font != null {
                free((*ml).font as *mut libc::c_void);
            }
            (*ml).font = null;
            if (*ml).fontx != null {
                free((*ml).fontx as *mut libc::c_void);
            }
            (*ml).fontx = null;
            if (*ml).color != null {
                free((*ml).color as *mut libc::c_void);
            }
            (*ml).color = null;
            bclear(
                (*ml).image as *mut libc::c_char,
                (*p).w_layer.l_width + 1 as libc::c_int,
            );
            if bce != 0 {
                MBceLine(p, i, 0 as libc::c_int, (*p).w_layer.l_width, bce);
            }
            i += 1;
            i;
            ml = ml.offset(1);
            ml;
        }
        cnt1 = (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mline>() as libc::c_ulong)
            as libc::c_int;
        cnt2 = ((ye - ys + 1 as libc::c_int - n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mline>() as libc::c_ulong)
            as libc::c_int;
        if cnt1 != 0 && cnt2 != 0 {
            Scroll(
                ((*p).w_mlines).offset(ys as isize) as *mut libc::c_char,
                cnt1,
                cnt2,
                tmp.as_mut_ptr() as *mut libc::c_char,
            );
        }
    } else {
        n = -n;
        if (ye - ys + 1 as libc::c_int) < n {
            n = ye - ys + 1 as libc::c_int;
        }
        if n > 256 as libc::c_int {
            MScrollV(p, -(n - 256 as libc::c_int), ys, ye, bce);
            n = 256 as libc::c_int;
        }
        ml = ((*p).w_mlines).offset(ye as isize);
        i = ye;
        while i > ye - n {
            if (*ml).attr != null {
                free((*ml).attr as *mut libc::c_void);
            }
            (*ml).attr = null;
            if (*ml).font != null {
                free((*ml).font as *mut libc::c_void);
            }
            (*ml).font = null;
            if (*ml).fontx != null {
                free((*ml).fontx as *mut libc::c_void);
            }
            (*ml).fontx = null;
            if (*ml).color != null {
                free((*ml).color as *mut libc::c_void);
            }
            (*ml).color = null;
            bclear(
                (*ml).image as *mut libc::c_char,
                (*p).w_layer.l_width + 1 as libc::c_int,
            );
            if bce != 0 {
                MBceLine(p, i, 0 as libc::c_int, (*p).w_layer.l_width, bce);
            }
            i -= 1;
            i;
            ml = ml.offset(-1);
            ml;
        }
        cnt1 = (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mline>() as libc::c_ulong)
            as libc::c_int;
        cnt2 = ((ye - ys + 1 as libc::c_int - n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mline>() as libc::c_ulong)
            as libc::c_int;
        if cnt1 != 0 && cnt2 != 0 {
            Scroll(
                ((*p).w_mlines).offset(ys as isize) as *mut libc::c_char,
                cnt2,
                cnt1,
                tmp.as_mut_ptr() as *mut libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn Scroll(
    mut cp: *mut libc::c_char,
    mut cnt1: libc::c_int,
    mut cnt2: libc::c_int,
    mut tmp: *mut libc::c_char,
) {
    if cnt1 == 0 || cnt2 == 0 {
        return;
    }
    if cnt1 <= cnt2 {
        bcopy(cp as *const libc::c_void, tmp as *mut libc::c_void, cnt1 as size_t);
        bcopy(
            cp.offset(cnt1 as isize) as *const libc::c_void,
            cp as *mut libc::c_void,
            cnt2 as size_t,
        );
        bcopy(
            tmp as *const libc::c_void,
            cp.offset(cnt2 as isize) as *mut libc::c_void,
            cnt1 as size_t,
        );
    } else {
        bcopy(
            cp.offset(cnt1 as isize) as *const libc::c_void,
            tmp as *mut libc::c_void,
            cnt2 as size_t,
        );
        bcopy(
            cp as *const libc::c_void,
            cp.offset(cnt2 as isize) as *mut libc::c_void,
            cnt1 as size_t,
        );
        bcopy(tmp as *const libc::c_void, cp as *mut libc::c_void, cnt2 as size_t);
    };
}
unsafe extern "C" fn MClearArea(
    mut p: *mut win,
    mut xs: libc::c_int,
    mut ys: libc::c_int,
    mut xe: libc::c_int,
    mut ye: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xxe: libc::c_int = 0;
    let mut ml: *mut mline = 0 as *mut mline;
    if ys < 0 as libc::c_int || ye < ys {
        return;
    }
    if xs >= (*p).w_layer.l_width {
        xs = (*p).w_layer.l_width - 1 as libc::c_int;
    }
    if xe >= (*p).w_layer.l_width {
        xe = (*p).w_layer.l_width - 1 as libc::c_int;
    }
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*((*p).w_mlines).offset(ys as isize)).font).offset(xs as isize)
            as libc::c_int == 0xff as libc::c_int
            && *((*((*p).w_mlines).offset(ys as isize)).image).offset(xs as isize)
                as libc::c_int == 0xff as libc::c_int) as libc::c_int
    } else {
        (*((*((*p).w_mlines).offset(ys as isize)).font).offset(xs as isize)
            as libc::c_int & 0xe0 as libc::c_int == 0x80 as libc::c_int) as libc::c_int
    } != 0
    {
        if xs > 0 as libc::c_int {
            *((*((*p).w_mlines).offset(ys as isize)).image)
                .offset((xs - 1 as libc::c_int) as isize) = mchar_blank.image;
            *((*((*p).w_mlines).offset(ys as isize)).attr)
                .offset((xs - 1 as libc::c_int) as isize) = mchar_blank.attr;
            *((*((*p).w_mlines).offset(ys as isize)).font)
                .offset((xs - 1 as libc::c_int) as isize) = mchar_blank.font;
            *((*((*p).w_mlines).offset(ys as isize)).fontx)
                .offset((xs - 1 as libc::c_int) as isize) = mchar_blank.fontx;
            *((*((*p).w_mlines).offset(ys as isize)).color)
                .offset((xs - 1 as libc::c_int) as isize) = mchar_blank.color;
        }
        *((*((*p).w_mlines).offset(ys as isize)).image)
            .offset(xs as isize) = mchar_blank.image;
        *((*((*p).w_mlines).offset(ys as isize)).attr)
            .offset(xs as isize) = mchar_blank.attr;
        *((*((*p).w_mlines).offset(ys as isize)).font)
            .offset(xs as isize) = mchar_blank.font;
        *((*((*p).w_mlines).offset(ys as isize)).fontx)
            .offset(xs as isize) = mchar_blank.fontx;
        *((*((*p).w_mlines).offset(ys as isize)).color)
            .offset(xs as isize) = mchar_blank.color;
    }
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*((*p).w_mlines).offset(ye as isize)).font)
            .offset((xe + 1 as libc::c_int) as isize) as libc::c_int
            == 0xff as libc::c_int
            && *((*((*p).w_mlines).offset(ye as isize)).image)
                .offset((xe + 1 as libc::c_int) as isize) as libc::c_int
                == 0xff as libc::c_int) as libc::c_int
    } else {
        (*((*((*p).w_mlines).offset(ye as isize)).font).offset(xe as isize)
            as libc::c_int & 0x1f as libc::c_int != 0 as libc::c_int
            && *((*((*p).w_mlines).offset(ye as isize)).font).offset(xe as isize)
                as libc::c_int & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        *((*((*p).w_mlines).offset(ye as isize)).image)
            .offset(xe as isize) = mchar_blank.image;
        *((*((*p).w_mlines).offset(ye as isize)).attr)
            .offset(xe as isize) = mchar_blank.attr;
        *((*((*p).w_mlines).offset(ye as isize)).font)
            .offset(xe as isize) = mchar_blank.font;
        *((*((*p).w_mlines).offset(ye as isize)).fontx)
            .offset(xe as isize) = mchar_blank.fontx;
        *((*((*p).w_mlines).offset(ye as isize)).color)
            .offset(xe as isize) = mchar_blank.color;
        *((*((*p).w_mlines).offset(ye as isize)).image)
            .offset((xe + 1 as libc::c_int) as isize) = mchar_blank.image;
        *((*((*p).w_mlines).offset(ye as isize)).attr)
            .offset((xe + 1 as libc::c_int) as isize) = mchar_blank.attr;
        *((*((*p).w_mlines).offset(ye as isize)).font)
            .offset((xe + 1 as libc::c_int) as isize) = mchar_blank.font;
        *((*((*p).w_mlines).offset(ye as isize)).fontx)
            .offset((xe + 1 as libc::c_int) as isize) = mchar_blank.fontx;
        *((*((*p).w_mlines).offset(ye as isize)).color)
            .offset((xe + 1 as libc::c_int) as isize) = mchar_blank.color;
    }
    ml = ((*p).w_mlines).offset(ys as isize);
    y = ys;
    while y <= ye {
        xxe = if y == ye { xe } else { (*p).w_layer.l_width - 1 as libc::c_int };
        n = xxe - xs + 1 as libc::c_int;
        if n > 0 as libc::c_int {
            bclear(((*ml).image as *mut libc::c_char).offset(xs as isize), n);
            if (*ml).attr != null {
                bzero(
                    ((*ml).attr as *mut libc::c_char).offset(xs as isize)
                        as *mut libc::c_void,
                    n as libc::c_ulong,
                );
            }
            if (*ml).font != null {
                bzero(
                    ((*ml).font as *mut libc::c_char).offset(xs as isize)
                        as *mut libc::c_void,
                    n as libc::c_ulong,
                );
            }
            if (*ml).fontx != null {
                bzero(
                    ((*ml).fontx as *mut libc::c_char).offset(xs as isize)
                        as *mut libc::c_void,
                    n as libc::c_ulong,
                );
            }
            if (*ml).color != null {
                bzero(
                    ((*ml).color as *mut libc::c_char).offset(xs as isize)
                        as *mut libc::c_void,
                    n as libc::c_ulong,
                );
            }
        }
        if n > 0 as libc::c_int && bce != 0 {
            MBceLine(p, y, xs, xs + n - 1 as libc::c_int, bce);
        }
        xs = 0 as libc::c_int;
        y += 1;
        y;
        ml = ml.offset(1);
        ml;
    }
}
unsafe extern "C" fn MInsChar(
    mut p: *mut win,
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut ml: *mut mline = 0 as *mut mline;
    MFixLine(p, y, c);
    ml = ((*p).w_mlines).offset(y as isize);
    n = (*p).w_layer.l_width - x - 1 as libc::c_int;
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset(x as isize) as libc::c_int == 0xff as libc::c_int
            && *((*ml).image).offset(x as isize) as libc::c_int == 0xff as libc::c_int)
            as libc::c_int
    } else {
        (*((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0x80 as libc::c_int) as libc::c_int
    } != 0
    {
        if x > 0 as libc::c_int {
            *((*ml).image).offset((x - 1 as libc::c_int) as isize) = mchar_blank.image;
            *((*ml).attr).offset((x - 1 as libc::c_int) as isize) = mchar_blank.attr;
            *((*ml).font).offset((x - 1 as libc::c_int) as isize) = mchar_blank.font;
            *((*ml).fontx).offset((x - 1 as libc::c_int) as isize) = mchar_blank.fontx;
            *((*ml).color).offset((x - 1 as libc::c_int) as isize) = mchar_blank.color;
        }
        *((*ml).image).offset(x as isize) = mchar_blank.image;
        *((*ml).attr).offset(x as isize) = mchar_blank.attr;
        *((*ml).font).offset(x as isize) = mchar_blank.font;
        *((*ml).fontx).offset(x as isize) = mchar_blank.fontx;
        *((*ml).color).offset(x as isize) = mchar_blank.color;
    }
    if n > 0 as libc::c_int {
        if if (*p).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset(((*p).w_layer.l_width - 1 as libc::c_int) as isize)
                as libc::c_int == 0xff as libc::c_int
                && *((*ml).image)
                    .offset(((*p).w_layer.l_width - 1 as libc::c_int) as isize)
                    as libc::c_int == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(((*p).w_layer.l_width - 1 as libc::c_int) as isize)
                as libc::c_int & 0xe0 as libc::c_int == 0x80 as libc::c_int)
                as libc::c_int
        } != 0
        {
            if (*p).w_layer.l_width - 1 as libc::c_int > 0 as libc::c_int {
                *((*ml).image)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                            as isize,
                    ) = mchar_blank.image;
                *((*ml).attr)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                            as isize,
                    ) = mchar_blank.attr;
                *((*ml).font)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                            as isize,
                    ) = mchar_blank.font;
                *((*ml).fontx)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                            as isize,
                    ) = mchar_blank.fontx;
                *((*ml).color)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                            as isize,
                    ) = mchar_blank.color;
            }
            *((*ml).image)
                .offset(
                    ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                ) = mchar_blank.image;
            *((*ml).attr)
                .offset(
                    ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                ) = mchar_blank.attr;
            *((*ml).font)
                .offset(
                    ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                ) = mchar_blank.font;
            *((*ml).fontx)
                .offset(
                    ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                ) = mchar_blank.fontx;
            *((*ml).color)
                .offset(
                    ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                ) = mchar_blank.color;
        }
        bcopy(
            ((*ml).image as *mut libc::c_char).offset(x as isize) as *const libc::c_void,
            ((*ml).image as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                as *mut libc::c_void,
            n as size_t,
        );
        bcopy(
            ((*ml).attr as *mut libc::c_char).offset(x as isize) as *const libc::c_void,
            ((*ml).attr as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                as *mut libc::c_void,
            n as size_t,
        );
        bcopy(
            ((*ml).font as *mut libc::c_char).offset(x as isize) as *const libc::c_void,
            ((*ml).font as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                as *mut libc::c_void,
            n as size_t,
        );
        bcopy(
            ((*ml).fontx as *mut libc::c_char).offset(x as isize) as *const libc::c_void,
            ((*ml).fontx as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                as *mut libc::c_void,
            n as size_t,
        );
        bcopy(
            ((*ml).color as *mut libc::c_char).offset(x as isize) as *const libc::c_void,
            ((*ml).color as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                as *mut libc::c_void,
            n as size_t,
        );
    }
    *((*ml).image).offset(x as isize) = (*c).image;
    *((*ml).attr).offset(x as isize) = (*c).attr;
    *((*ml).font).offset(x as isize) = (*c).font;
    *((*ml).fontx).offset(x as isize) = (*c).fontx;
    *((*ml).color).offset(x as isize) = (*c).color;
    if (*c).mbcs != 0 {
        n -= 1;
        if n > 0 as libc::c_int {
            if if (*p).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset(((*p).w_layer.l_width - 1 as libc::c_int) as isize)
                    as libc::c_int == 0xff as libc::c_int
                    && *((*ml).image)
                        .offset(((*p).w_layer.l_width - 1 as libc::c_int) as isize)
                        as libc::c_int == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset(((*p).w_layer.l_width - 1 as libc::c_int) as isize)
                    as libc::c_int & 0xe0 as libc::c_int == 0x80 as libc::c_int)
                    as libc::c_int
            } != 0
            {
                if (*p).w_layer.l_width - 1 as libc::c_int > 0 as libc::c_int {
                    *((*ml).image)
                        .offset(
                            ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                                as isize,
                        ) = mchar_blank.image;
                    *((*ml).attr)
                        .offset(
                            ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                                as isize,
                        ) = mchar_blank.attr;
                    *((*ml).font)
                        .offset(
                            ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                                as isize,
                        ) = mchar_blank.font;
                    *((*ml).fontx)
                        .offset(
                            ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                                as isize,
                        ) = mchar_blank.fontx;
                    *((*ml).color)
                        .offset(
                            ((*p).w_layer.l_width - 1 as libc::c_int - 1 as libc::c_int)
                                as isize,
                        ) = mchar_blank.color;
                }
                *((*ml).image)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                    ) = mchar_blank.image;
                *((*ml).attr)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                    ) = mchar_blank.attr;
                *((*ml).font)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                    ) = mchar_blank.font;
                *((*ml).fontx)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                    ) = mchar_blank.fontx;
                *((*ml).color)
                    .offset(
                        ((*p).w_layer.l_width - 1 as libc::c_int) as isize,
                    ) = mchar_blank.color;
            }
            bcopy(
                ((*ml).image as *mut libc::c_char)
                    .offset((x + 1 as libc::c_int) as isize) as *const libc::c_void,
                ((*ml).image as *mut libc::c_char)
                    .offset((x + 2 as libc::c_int) as isize) as *mut libc::c_void,
                n as size_t,
            );
            bcopy(
                ((*ml).attr as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                    as *const libc::c_void,
                ((*ml).attr as *mut libc::c_char).offset((x + 2 as libc::c_int) as isize)
                    as *mut libc::c_void,
                n as size_t,
            );
            bcopy(
                ((*ml).font as *mut libc::c_char).offset((x + 1 as libc::c_int) as isize)
                    as *const libc::c_void,
                ((*ml).font as *mut libc::c_char).offset((x + 2 as libc::c_int) as isize)
                    as *mut libc::c_void,
                n as size_t,
            );
            bcopy(
                ((*ml).fontx as *mut libc::c_char)
                    .offset((x + 1 as libc::c_int) as isize) as *const libc::c_void,
                ((*ml).fontx as *mut libc::c_char)
                    .offset((x + 2 as libc::c_int) as isize) as *mut libc::c_void,
                n as size_t,
            );
            bcopy(
                ((*ml).color as *mut libc::c_char)
                    .offset((x + 1 as libc::c_int) as isize) as *const libc::c_void,
                ((*ml).color as *mut libc::c_char)
                    .offset((x + 2 as libc::c_int) as isize) as *mut libc::c_void,
                n as size_t,
            );
        }
        *((*ml).image).offset((x + 1 as libc::c_int) as isize) = (*c).image;
        *((*ml).attr).offset((x + 1 as libc::c_int) as isize) = (*c).attr;
        *((*ml).font).offset((x + 1 as libc::c_int) as isize) = (*c).font;
        *((*ml).fontx).offset((x + 1 as libc::c_int) as isize) = (*c).fontx;
        *((*ml).color).offset((x + 1 as libc::c_int) as isize) = (*c).color;
        *((*ml).image).offset((x + 1 as libc::c_int) as isize) = (*c).mbcs;
        if (*p).w_layer.l_encoding != 8 as libc::c_int {
            let ref mut fresh12 = *((*ml).font).offset((x + 1 as libc::c_int) as isize);
            *fresh12 = (*fresh12 as libc::c_int | 0x80 as libc::c_int) as libc::c_uchar;
        } else if (*p).w_layer.l_encoding == 8 as libc::c_int
            && (*c).mbcs as libc::c_int != 0
        {
            *((*ml).font).offset((x + 1 as libc::c_int) as isize) = (*c).mbcs;
            *((*ml).fontx)
                .offset(
                    (x + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_uchar;
        }
    }
}
unsafe extern "C" fn MPutChar(
    mut p: *mut win,
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut ml: *mut mline = 0 as *mut mline;
    MFixLine(p, y, c);
    ml = &mut *((*p).w_mlines).offset(y as isize) as *mut mline;
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset(x as isize) as libc::c_int == 0xff as libc::c_int
            && *((*ml).image).offset(x as isize) as libc::c_int == 0xff as libc::c_int)
            as libc::c_int
    } else {
        (*((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0x80 as libc::c_int) as libc::c_int
    } != 0
    {
        if x > 0 as libc::c_int {
            *((*ml).image).offset((x - 1 as libc::c_int) as isize) = mchar_blank.image;
            *((*ml).attr).offset((x - 1 as libc::c_int) as isize) = mchar_blank.attr;
            *((*ml).font).offset((x - 1 as libc::c_int) as isize) = mchar_blank.font;
            *((*ml).fontx).offset((x - 1 as libc::c_int) as isize) = mchar_blank.fontx;
            *((*ml).color).offset((x - 1 as libc::c_int) as isize) = mchar_blank.color;
        }
        *((*ml).image).offset(x as isize) = mchar_blank.image;
        *((*ml).attr).offset(x as isize) = mchar_blank.attr;
        *((*ml).font).offset(x as isize) = mchar_blank.font;
        *((*ml).fontx).offset(x as isize) = mchar_blank.fontx;
        *((*ml).color).offset(x as isize) = mchar_blank.color;
    }
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
            == 0xff as libc::c_int
            && *((*ml).image).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                == 0xff as libc::c_int) as libc::c_int
    } else {
        (*((*ml).font).offset(x as isize) as libc::c_int & 0x1f as libc::c_int
            != 0 as libc::c_int
            && *((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
                == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        *((*ml).image).offset(x as isize) = mchar_blank.image;
        *((*ml).attr).offset(x as isize) = mchar_blank.attr;
        *((*ml).font).offset(x as isize) = mchar_blank.font;
        *((*ml).fontx).offset(x as isize) = mchar_blank.fontx;
        *((*ml).color).offset(x as isize) = mchar_blank.color;
        *((*ml).image).offset((x + 1 as libc::c_int) as isize) = mchar_blank.image;
        *((*ml).attr).offset((x + 1 as libc::c_int) as isize) = mchar_blank.attr;
        *((*ml).font).offset((x + 1 as libc::c_int) as isize) = mchar_blank.font;
        *((*ml).fontx).offset((x + 1 as libc::c_int) as isize) = mchar_blank.fontx;
        *((*ml).color).offset((x + 1 as libc::c_int) as isize) = mchar_blank.color;
    }
    *((*ml).image).offset(x as isize) = (*c).image;
    *((*ml).attr).offset(x as isize) = (*c).attr;
    *((*ml).font).offset(x as isize) = (*c).font;
    *((*ml).fontx).offset(x as isize) = (*c).fontx;
    *((*ml).color).offset(x as isize) = (*c).color;
    if (*c).mbcs != 0 {
        if if (*p).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset((x + 1 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int == 0xff as libc::c_int
                && *((*ml).image)
                    .offset((x + 1 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                & 0x1f as libc::c_int != 0 as libc::c_int
                && *((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                    & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
        } != 0
        {
            *((*ml).image).offset((x + 1 as libc::c_int) as isize) = mchar_blank.image;
            *((*ml).attr).offset((x + 1 as libc::c_int) as isize) = mchar_blank.attr;
            *((*ml).font).offset((x + 1 as libc::c_int) as isize) = mchar_blank.font;
            *((*ml).fontx).offset((x + 1 as libc::c_int) as isize) = mchar_blank.fontx;
            *((*ml).color).offset((x + 1 as libc::c_int) as isize) = mchar_blank.color;
            *((*ml).image)
                .offset(
                    (x + 1 as libc::c_int + 1 as libc::c_int) as isize,
                ) = mchar_blank.image;
            *((*ml).attr)
                .offset(
                    (x + 1 as libc::c_int + 1 as libc::c_int) as isize,
                ) = mchar_blank.attr;
            *((*ml).font)
                .offset(
                    (x + 1 as libc::c_int + 1 as libc::c_int) as isize,
                ) = mchar_blank.font;
            *((*ml).fontx)
                .offset(
                    (x + 1 as libc::c_int + 1 as libc::c_int) as isize,
                ) = mchar_blank.fontx;
            *((*ml).color)
                .offset(
                    (x + 1 as libc::c_int + 1 as libc::c_int) as isize,
                ) = mchar_blank.color;
        }
        *((*ml).image).offset((x + 1 as libc::c_int) as isize) = (*c).image;
        *((*ml).attr).offset((x + 1 as libc::c_int) as isize) = (*c).attr;
        *((*ml).font).offset((x + 1 as libc::c_int) as isize) = (*c).font;
        *((*ml).fontx).offset((x + 1 as libc::c_int) as isize) = (*c).fontx;
        *((*ml).color).offset((x + 1 as libc::c_int) as isize) = (*c).color;
        *((*ml).image).offset((x + 1 as libc::c_int) as isize) = (*c).mbcs;
        if (*p).w_layer.l_encoding != 8 as libc::c_int {
            let ref mut fresh13 = *((*ml).font).offset((x + 1 as libc::c_int) as isize);
            *fresh13 = (*fresh13 as libc::c_int | 0x80 as libc::c_int) as libc::c_uchar;
        } else if (*p).w_layer.l_encoding == 8 as libc::c_int
            && (*c).mbcs as libc::c_int != 0
        {
            *((*ml).font).offset((x + 1 as libc::c_int) as isize) = (*c).mbcs;
            *((*ml).fontx)
                .offset(
                    (x + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_uchar;
        }
    }
}
unsafe extern "C" fn MWrapChar(
    mut p: *mut win,
    mut c: *mut mchar,
    mut y: libc::c_int,
    mut top: libc::c_int,
    mut bot: libc::c_int,
    mut ins: libc::c_int,
) {
    let mut ml: *mut mline = 0 as *mut mline;
    let mut bce: libc::c_int = 0;
    bce = ((*c).color as libc::c_int & 0xf0 as libc::c_int) >> 4 as libc::c_int
        | (if (*c).attr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            0x100 as libc::c_int
        } else {
            0 as libc::c_int
        });
    MFixLine(p, y, c);
    ml = &mut *((*p).w_mlines).offset(y as isize) as *mut mline;
    *((*ml).image).offset((*p).w_layer.l_width as isize) = mchar_null.image;
    *((*ml).attr).offset((*p).w_layer.l_width as isize) = mchar_null.attr;
    *((*ml).font).offset((*p).w_layer.l_width as isize) = mchar_null.font;
    *((*ml).fontx).offset((*p).w_layer.l_width as isize) = mchar_null.fontx;
    *((*ml).color).offset((*p).w_layer.l_width as isize) = mchar_null.color;
    if y == bot {
        MScrollV(p, 1 as libc::c_int, top, bot, bce);
    } else if y < (*p).w_layer.l_height - 1 as libc::c_int {
        y += 1;
        y;
    }
    if ins != 0 {
        MInsChar(p, c, 0 as libc::c_int, y);
    } else {
        MPutChar(p, c, 0 as libc::c_int, y);
    };
}
unsafe extern "C" fn MPutStr(
    mut p: *mut win,
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
    mut r: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut ml: *mut mline = 0 as *mut mline;
    let mut i: libc::c_int = 0;
    let mut b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if n <= 0 as libc::c_int {
        return;
    }
    MFixLine(p, y, r);
    ml = &mut *((*p).w_mlines).offset(y as isize) as *mut mline;
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset(x as isize) as libc::c_int == 0xff as libc::c_int
            && *((*ml).image).offset(x as isize) as libc::c_int == 0xff as libc::c_int)
            as libc::c_int
    } else {
        (*((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0x80 as libc::c_int) as libc::c_int
    } != 0
    {
        if x > 0 as libc::c_int {
            *((*ml).image).offset((x - 1 as libc::c_int) as isize) = mchar_blank.image;
            *((*ml).attr).offset((x - 1 as libc::c_int) as isize) = mchar_blank.attr;
            *((*ml).font).offset((x - 1 as libc::c_int) as isize) = mchar_blank.font;
            *((*ml).fontx).offset((x - 1 as libc::c_int) as isize) = mchar_blank.fontx;
            *((*ml).color).offset((x - 1 as libc::c_int) as isize) = mchar_blank.color;
        }
        *((*ml).image).offset(x as isize) = mchar_blank.image;
        *((*ml).attr).offset(x as isize) = mchar_blank.attr;
        *((*ml).font).offset(x as isize) = mchar_blank.font;
        *((*ml).fontx).offset(x as isize) = mchar_blank.fontx;
        *((*ml).color).offset(x as isize) = mchar_blank.color;
    }
    if if (*p).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset((x + n - 1 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int == 0xff as libc::c_int
            && *((*ml).image)
                .offset((x + n - 1 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int == 0xff as libc::c_int) as libc::c_int
    } else {
        (*((*ml).font).offset((x + n - 1 as libc::c_int) as isize) as libc::c_int
            & 0x1f as libc::c_int != 0 as libc::c_int
            && *((*ml).font).offset((x + n - 1 as libc::c_int) as isize) as libc::c_int
                & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        *((*ml).image).offset((x + n - 1 as libc::c_int) as isize) = mchar_blank.image;
        *((*ml).attr).offset((x + n - 1 as libc::c_int) as isize) = mchar_blank.attr;
        *((*ml).font).offset((x + n - 1 as libc::c_int) as isize) = mchar_blank.font;
        *((*ml).fontx).offset((x + n - 1 as libc::c_int) as isize) = mchar_blank.fontx;
        *((*ml).color).offset((x + n - 1 as libc::c_int) as isize) = mchar_blank.color;
        *((*ml).image)
            .offset(
                (x + n - 1 as libc::c_int + 1 as libc::c_int) as isize,
            ) = mchar_blank.image;
        *((*ml).attr)
            .offset(
                (x + n - 1 as libc::c_int + 1 as libc::c_int) as isize,
            ) = mchar_blank.attr;
        *((*ml).font)
            .offset(
                (x + n - 1 as libc::c_int + 1 as libc::c_int) as isize,
            ) = mchar_blank.font;
        *((*ml).fontx)
            .offset(
                (x + n - 1 as libc::c_int + 1 as libc::c_int) as isize,
            ) = mchar_blank.fontx;
        *((*ml).color)
            .offset(
                (x + n - 1 as libc::c_int + 1 as libc::c_int) as isize,
            ) = mchar_blank.color;
    }
    bcopy(
        s as *const libc::c_void,
        ((*ml).image as *mut libc::c_char).offset(x as isize) as *mut libc::c_void,
        n as size_t,
    );
    if (*ml).attr != null {
        b = ((*ml).attr).offset(x as isize);
        i = n;
        loop {
            let fresh14 = i;
            i = i - 1;
            if !(fresh14 > 0 as libc::c_int) {
                break;
            }
            let fresh15 = b;
            b = b.offset(1);
            *fresh15 = (*r).attr;
        }
    }
    if (*ml).font != null {
        b = ((*ml).font).offset(x as isize);
        i = n;
        loop {
            let fresh16 = i;
            i = i - 1;
            if !(fresh16 > 0 as libc::c_int) {
                break;
            }
            let fresh17 = b;
            b = b.offset(1);
            *fresh17 = (*r).font;
        }
    }
    if (*ml).fontx != null {
        b = ((*ml).fontx).offset(x as isize);
        i = n;
        loop {
            let fresh18 = i;
            i = i - 1;
            if !(fresh18 > 0 as libc::c_int) {
                break;
            }
            let fresh19 = b;
            b = b.offset(1);
            *fresh19 = (*r).fontx;
        }
    }
    if (*ml).color != null {
        b = ((*ml).color).offset(x as isize);
        i = n;
        loop {
            let fresh20 = i;
            i = i - 1;
            if !(fresh20 > 0 as libc::c_int) {
                break;
            }
            let fresh21 = b;
            b = b.offset(1);
            *fresh21 = (*r).color;
        }
    }
}
unsafe extern "C" fn MBceLine(
    mut p: *mut win,
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut mc: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    let mut ml: *mut mline = 0 as *mut mline;
    let mut x: libc::c_int = 0;
    mc = mchar_null;
    mc
        .color = (mc.color as libc::c_int & 0xf as libc::c_int
        | bce << 4 as libc::c_int & 0xf0 as libc::c_int) as libc::c_uchar;
    mc
        .attr = ((mc.attr as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int)
        ^ (if bce & 0x100 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 7 as libc::c_int
        })) as libc::c_uchar;
    MFixLine(p, y, &mut mc);
    ml = ((*p).w_mlines).offset(y as isize);
    if mc.attr != 0 {
        x = xs;
        while x <= xe {
            *((*ml).attr).offset(x as isize) = mc.attr;
            x += 1;
            x;
        }
    }
    if mc.color != 0 {
        x = xs;
        while x <= xe {
            *((*ml).color).offset(x as isize) = mc.color;
            x += 1;
            x;
        }
    }
}
unsafe extern "C" fn WAddLineToHist(mut wp: *mut win, mut ml: *mut mline) {
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut o: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hml: *mut mline = 0 as *mut mline;
    if (*wp).w_histheight == 0 as libc::c_int {
        return;
    }
    hml = &mut *((*wp).w_hlines).offset((*wp).w_histidx as isize) as *mut mline;
    q = (*ml).image;
    (*ml).image = (*hml).image;
    (*hml).image = q;
    q = (*ml).attr;
    o = (*hml).attr;
    (*hml).attr = q;
    (*ml).attr = null;
    if o != null {
        free(o as *mut libc::c_void);
    }
    q = (*ml).font;
    o = (*hml).font;
    (*hml).font = q;
    (*ml).font = null;
    if o != null {
        free(o as *mut libc::c_void);
    }
    q = (*ml).fontx;
    o = (*hml).fontx;
    (*hml).fontx = q;
    (*ml).fontx = null;
    if o != null {
        free(o as *mut libc::c_void);
    }
    q = (*ml).color;
    o = (*hml).color;
    (*hml).color = q;
    (*ml).color = null;
    if o != null {
        free(o as *mut libc::c_void);
    }
    (*wp).w_histidx += 1;
    if (*wp).w_histidx >= (*wp).w_histheight {
        (*wp).w_histidx = 0 as libc::c_int;
    }
    if (*wp).w_scrollback_height < (*wp).w_histheight {
        (*wp).w_scrollback_height += 1;
        (*wp).w_scrollback_height;
    }
}
pub unsafe extern "C" fn MFindUsedLine(
    mut p: *mut win,
    mut ye: libc::c_int,
    mut ys: libc::c_int,
) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut ml: *mut mline = ((*p).w_mlines).offset(ye as isize);
    y = ye;
    while y >= ys {
        if bcmp(
            (*ml).image as *mut libc::c_char as *const libc::c_void,
            blank as *const libc::c_void,
            (*p).w_layer.l_width as libc::c_ulong,
        ) != 0
        {
            break;
        }
        if (*ml).attr != null
            && bcmp(
                (*ml).attr as *mut libc::c_char as *const libc::c_void,
                null as *const libc::c_void,
                (*p).w_layer.l_width as libc::c_ulong,
            ) != 0
        {
            break;
        }
        if (*ml).color != null
            && bcmp(
                (*ml).color as *mut libc::c_char as *const libc::c_void,
                null as *const libc::c_void,
                (*p).w_layer.l_width as libc::c_ulong,
            ) != 0
        {
            break;
        }
        if (*p).w_layer.l_encoding == 8 as libc::c_int {
            if (*ml).font != null
                && bcmp(
                    (*ml).font as *mut libc::c_char as *const libc::c_void,
                    null as *const libc::c_void,
                    (*p).w_layer.l_width as libc::c_ulong,
                ) != 0
            {
                break;
            }
            if (*ml).fontx != null
                && bcmp(
                    (*ml).fontx as *mut libc::c_char as *const libc::c_void,
                    null as *const libc::c_void,
                    (*p).w_layer.l_width as libc::c_ulong,
                ) != 0
            {
                break;
            }
        }
        y -= 1;
        y;
        ml = ml.offset(-1);
        ml;
    }
    return y;
}
pub unsafe extern "C" fn WBell(mut p: *mut win, mut visual: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    if displays.is_null() {
        (*p).w_bell = 2 as libc::c_int;
    }
    display = displays;
    while !display.is_null() {
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if (*(*cv).c_layer).l_bottom == &mut (*p).w_layer as *mut layer {
                break;
            }
            cv = (*cv).c_next;
        }
        if !cv.is_null() && visual == 0 {
            AddCStr((*display).d_tcs[42 as libc::c_int as usize].str_0);
        } else if !cv.is_null()
            && !((*display).d_tcs[43 as libc::c_int as usize].str_0).is_null()
        {
            AddCStr((*display).d_tcs[43 as libc::c_int as usize].str_0);
        } else {
            (*p).w_bell = if visual != 0 { 3 as libc::c_int } else { 1 as libc::c_int };
        }
        display = (*display).d_next;
    }
}
unsafe extern "C" fn WReverseVideo(mut p: *mut win, mut on: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*p).w_layer.l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !(cv != (*display).d_forecv) {
            ReverseVideo(on);
            if on == 0 && (*p).w_revvid != 0
                && ((*display).d_tcs[93 as libc::c_int as usize].str_0).is_null()
            {
                if !((*display).d_tcs[43 as libc::c_int as usize].str_0).is_null() {
                    AddCStr((*display).d_tcs[43 as libc::c_int as usize].str_0);
                } else {
                    (*p).w_bell = 3 as libc::c_int;
                }
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn WMsg(
    mut p: *mut win,
    mut err: libc::c_int,
    mut str: *mut libc::c_char,
) {
    extern "C" {
        #[link_name = "flayer"]
        static mut flayer_0: *mut layer;
    }
    let mut oldflayer: *mut layer = flayer;
    flayer = &mut (*p).w_layer;
    LMsg(err, b"%s\0" as *const u8 as *const libc::c_char, str);
    flayer = oldflayer;
}
pub unsafe extern "C" fn WChangeSize(
    mut p: *mut win,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut wok: libc::c_int = 0 as libc::c_int;
    let mut cv: *mut canvas = 0 as *mut canvas;
    if ((*p).w_layer.l_cvlist).is_null() {
        ChangeWindowSize(p, w, h, (*p).w_histheight);
        return;
    }
    cv = (*p).w_layer.l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !(p != (*display).d_fore) {
            if !((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null() {
                break;
            }
            if !((*display).d_tcs[45 as libc::c_int as usize].str_0).is_null()
                && (w == Z0width || w == Z1width)
            {
                wok = 1 as libc::c_int;
            }
        }
        cv = (*cv).c_lnext;
    }
    if cv.is_null() && wok == 0 as libc::c_int {
        return;
    }
    if ((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null() {
        h = (*p).w_layer.l_height;
    }
    ChangeWindowSize(p, w, h, (*p).w_histheight);
    display = displays;
    while !display.is_null() {
        if p == (*display).d_fore {
            if !((*display).d_cvlist).is_null()
                && ((*(*display).d_cvlist).c_next).is_null()
            {
                ResizeDisplay(w, h);
            } else {
                ResizeDisplay(w, (*display).d_height);
            }
            ResizeLayersToCanvases();
        } else {
            cv = (*display).d_cvlist;
            while !cv.is_null() {
                if (*(*cv).c_layer).l_bottom == &mut (*p).w_layer as *mut layer {
                    break;
                }
                cv = (*cv).c_next;
            }
            if !cv.is_null() {
                Redisplay(0 as libc::c_int);
            }
        }
        display = (*display).d_next;
    }
}
unsafe extern "C" fn WindowChangedCheck(
    mut s: *mut libc::c_char,
    mut what: libc::c_int,
    mut hp: *mut libc::c_int,
) -> libc::c_int {
    let mut h: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    while *s != 0 {
        let fresh22 = s;
        s = s.offset(1);
        if *fresh22 as libc::c_int
            != (if !hp.is_null() { '%' as i32 } else { '\u{5}' as i32 })
        {
            continue;
        }
        l = 0 as libc::c_int;
        s = s.offset((*s as libc::c_int == '+' as i32) as libc::c_int as isize);
        s = s.offset((*s as libc::c_int == '-' as i32) as libc::c_int as isize);
        while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == 'L' as i32 {
            s = s.offset(1);
            s;
            l = 0x100 as libc::c_int;
        }
        if *s as libc::c_int == 'h' as i32 {
            h = 1 as libc::c_int;
        }
        if *s as libc::c_int == what || *s as libc::c_int | l == what
            || what == 'd' as i32
        {
            break;
        }
        if *s != 0 {
            s = s.offset(1);
            s;
        }
    }
    if !hp.is_null() {
        *hp = h;
    }
    return if *s as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn WindowChanged(mut p: *mut win, mut what: libc::c_int) {
    let mut inwstr: libc::c_int = 0;
    let mut inhstr: libc::c_int = 0;
    let mut inlstr: libc::c_int = 0;
    let mut inwstrh: libc::c_int = 0 as libc::c_int;
    let mut inhstrh: libc::c_int = 0 as libc::c_int;
    let mut inlstrh: libc::c_int = 0 as libc::c_int;
    let mut got: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut olddisplay: *mut display = display;
    let mut cv: *mut canvas = 0 as *mut canvas;
    inhstr = 0 as libc::c_int;
    inwstr = inhstr;
    if what == 'f' as i32 {
        WindowChanged(0 as *mut win, 'w' as i32 | 0x100 as libc::c_int);
        WindowChanged(0 as *mut win, 'W' as i32 | 0x100 as libc::c_int);
    }
    if what != 0 {
        inwstr = WindowChangedCheck(captionstring, what, &mut inwstrh);
        inhstr = WindowChangedCheck(hstatusstring, what, &mut inhstrh);
        inlstr = WindowChangedCheck(wliststr, what, &mut inlstrh);
    } else {
        inhstr = 0 as libc::c_int;
        inwstr = inhstr;
        inlstr = 1 as libc::c_int;
    }
    if p.is_null() {
        display = displays;
        while !display.is_null() {
            ox = (*display).d_x;
            oy = (*display).d_y;
            cv = (*display).d_cvlist;
            while !cv.is_null() {
                if inlstr != 0
                    || inlstrh != 0 && !p.is_null() && !((*p).w_hstatus).is_null()
                        && *(*p).w_hstatus as libc::c_int != 0
                        && WindowChangedCheck(
                            (*p).w_hstatus,
                            what,
                            0 as *mut libc::c_int,
                        ) != 0
                {
                    WListUpdatecv(cv, 0 as *mut win);
                }
                p = (*(*(*cv).c_layer).l_bottom).l_data as *mut win;
                if inwstr != 0
                    || inwstrh != 0 && !p.is_null() && !((*p).w_hstatus).is_null()
                        && *(*p).w_hstatus as libc::c_int != 0
                        && WindowChangedCheck(
                            (*p).w_hstatus,
                            what,
                            0 as *mut libc::c_int,
                        ) != 0
                {
                    if ((*cv).c_ye + 1 as libc::c_int) < (*display).d_height {
                        RefreshLine(
                            (*cv).c_ye + 1 as libc::c_int,
                            0 as libc::c_int,
                            (*display).d_width - 1 as libc::c_int,
                            0 as libc::c_int,
                        );
                    }
                }
                cv = (*cv).c_next;
            }
            p = (*display).d_fore;
            if inhstr != 0
                || inhstrh != 0 && !p.is_null() && !((*p).w_hstatus).is_null()
                    && *(*p).w_hstatus as libc::c_int != 0
                    && WindowChangedCheck((*p).w_hstatus, what, 0 as *mut libc::c_int)
                        != 0
            {
                RefreshHStatus();
            }
            if ox != -(1 as libc::c_int) && oy != -(1 as libc::c_int) {
                GotoPos(ox, oy);
            }
            display = (*display).d_next;
        }
        display = olddisplay;
        return;
    }
    if !((*p).w_hstatus).is_null() && *(*p).w_hstatus as libc::c_int != 0
        && (inwstrh != 0 || inhstrh != 0 || inlstrh != 0)
        && WindowChangedCheck((*p).w_hstatus, what, 0 as *mut libc::c_int) != 0
    {
        inwstr |= inwstrh;
        inhstr |= inhstrh;
        inlstr |= inlstrh;
    }
    if inwstr == 0 && inhstr == 0 && inlstr == 0 {
        return;
    }
    display = displays;
    while !display.is_null() {
        got = 0 as libc::c_int;
        ox = (*display).d_x;
        oy = (*display).d_y;
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if inlstr != 0 {
                WListUpdatecv(cv, p);
            }
            if !((*(*(*cv).c_layer).l_bottom).l_data as *mut win != p) {
                got = 1 as libc::c_int;
                if inwstr != 0 && ((*cv).c_ye + 1 as libc::c_int) < (*display).d_height {
                    RefreshLine(
                        (*cv).c_ye + 1 as libc::c_int,
                        0 as libc::c_int,
                        (*display).d_width - 1 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
            }
            cv = (*cv).c_next;
        }
        if got != 0 && inhstr != 0 && p == (*display).d_fore {
            RefreshHStatus();
        }
        if ox != -(1 as libc::c_int) && oy != -(1 as libc::c_int) {
            GotoPos(ox, oy);
        }
        display = (*display).d_next;
    }
    display = olddisplay;
}
