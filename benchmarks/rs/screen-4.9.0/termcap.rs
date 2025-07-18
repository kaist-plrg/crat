use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type logfile;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn tgetent(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int;
    fn tgetstr(_: *mut libc::c_char, _: *mut *mut libc::c_char) -> *mut libc::c_char;
    fn tgetnum(_: *mut libc::c_char) -> libc::c_int;
    fn tgetflag(_: *mut libc::c_char) -> libc::c_int;
    static mut DefaultEsc: libc::c_int;
    static mut DefaultMetaEsc: libc::c_int;
    static mut strnomem: [libc::c_char; 0];
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn SetEscape(_: *mut acluser, _: libc::c_int, _: libc::c_int);
    fn CalcCost(_: *mut libc::c_char) -> libc::c_int;
    fn CheckScreenSize(_: libc::c_int);
    fn xrealloc(_: *mut libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn evdeq(_: *mut event);
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn InStr(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn xseteuid(_: libc::c_int);
    fn xsetegid(_: libc::c_int);
    fn FindEncoding(_: *mut libc::c_char) -> libc::c_int;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut real_uid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut eff_uid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut term: [term; 0];
    static mut nwin_undef: NewWindow;
    static mut nwin_default: NewWindow;
    static mut nwin_options: NewWindow;
    static mut force_vt: libc::c_int;
    static mut hardstatusemu: libc::c_int;
    static mut kmapdef: [*mut libc::c_char; 0];
    static mut umtab: [action; 0];
    static mut mmtab: [action; 0];
    static mut dmtab: [action; 0];
    static mut kmap_exts: *mut kmap_ext;
    static mut kmap_extn: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type pid_t = __pid_t;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut Termcap: [libc::c_char; 1031] = [0; 1031];
static mut Termcaplen: libc::c_int = 0;
static mut tcLineLen: libc::c_int = 0;
pub static mut Term: [libc::c_char; 773] = [0; 773];
pub static mut screenterm: [libc::c_char; 33] = [0; 33];
pub static mut extra_incap: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut extra_outcap: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut TermcapConst: [libc::c_char; 208] = unsafe {
    *::std::mem::transmute::<
        &[u8; 208],
        &[libc::c_char; 208],
    >(
        b"DO=\\E[%dB:LE=\\E[%dD:RI=\\E[%dC:UP=\\E[%dA:bs:bt=\\E[Z:cd=\\E[J:ce=\\E[K:cl=\\E[H\\E[J:cm=\\E[%i%d;%dH:ct=\\E[3g:do=^J:nd=\\E[C:pt:rc=\\E8:rs=\\Ec:sc=\\E7:st=\\EH:up=\\EM:le=^H:bl=^G:cr=^M:it#8:ho=\\E[H:nw=\\EE:ta=^I:is=\\E)0:\0",
    )
};
pub unsafe extern "C" fn gettermcapstring(
    mut s: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if display.is_null() || s.is_null() {
        return 0 as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    while i < 201 as libc::c_int {
        if !((*term.as_mut_ptr().offset(i as isize)).type_0 != 2 as libc::c_int) {
            if strcmp((*term.as_mut_ptr().offset(i as isize)).tcname, s)
                == 0 as libc::c_int
            {
                return (*display).d_tcs[i as usize].str_0;
            }
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn InitTermcap(
    mut wi: libc::c_int,
    mut he: libc::c_int,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tbuf: [libc::c_char; 1023] = [0; 1023];
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_int = 0;
    let mut xue: libc::c_int = 0;
    let mut xse: libc::c_int = 0;
    let mut xme: libc::c_int = 0;
    bzero(
        tbuf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 1023]>() as libc::c_ulong,
    );
    if *((*display).d_termname).as_mut_ptr() as libc::c_int == 0 as libc::c_int
        || e_tgetent(tbuf.as_mut_ptr(), ((*display).d_termname).as_mut_ptr())
            != 1 as libc::c_int
    {
        Msg(
            0 as libc::c_int,
            b"Cannot find terminfo entry for '%s'.\0" as *const u8
                as *const libc::c_char,
            ((*display).d_termname).as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    (*display)
        .d_tentry = malloc(
        (1023 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if !extra_incap.is_null() {
                    (strlen(extra_incap)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }),
            ),
    ) as *mut libc::c_char;
    if ((*display).d_tentry).is_null() {
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    tp = (*display).d_tentry;
    i = 0 as libc::c_int;
    while i < 201 as libc::c_int {
        match (*term.as_mut_ptr().offset(i as isize)).type_0 {
            0 => {
                (*display)
                    .d_tcs[i as usize]
                    .flg = e_tgetflag((*term.as_mut_ptr().offset(i as isize)).tcname);
            }
            1 => {
                (*display)
                    .d_tcs[i as usize]
                    .num = e_tgetnum((*term.as_mut_ptr().offset(i as isize)).tcname);
            }
            2 => {
                (*display)
                    .d_tcs[i as usize]
                    .str_0 = e_tgetstr(
                    (*term.as_mut_ptr().offset(i as isize)).tcname,
                    &mut tp,
                );
                if !((*display).d_tcs[i as usize].str_0).is_null()
                    && *(*display).d_tcs[i as usize].str_0 as libc::c_int
                        == 0 as libc::c_int
                {
                    (*display).d_tcs[i as usize].str_0 = 0 as *mut libc::c_char;
                }
            }
            _ => {
                Panic(
                    0 as libc::c_int,
                    b"Illegal tc type in entry #%d\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
        }
        i += 1;
        i;
    }
    if (*display).d_tcs[2 as libc::c_int as usize].flg != 0 {
        Msg(
            0 as libc::c_int,
            b"You can't run screen on a hardcopy terminal.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*display).d_tcs[3 as libc::c_int as usize].flg != 0 {
        Msg(
            0 as libc::c_int,
            b"You can't run screen on a terminal that overstrikes.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*display).d_tcs[34 as libc::c_int as usize].str_0).is_null() {
        Msg(
            0 as libc::c_int,
            b"Clear screen capability required.\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*display).d_tcs[5 as libc::c_int as usize].str_0).is_null() {
        Msg(
            0 as libc::c_int,
            b"Addressable cursor capability required.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    s = getenv(b"COLUMNS\0" as *const u8 as *const libc::c_char);
    if !s.is_null()
        && {
            i = atoi(s);
            i > 0 as libc::c_int
        }
    {
        (*display).d_tcs[1 as libc::c_int as usize].num = i;
    }
    s = getenv(b"LINES\0" as *const u8 as *const libc::c_char);
    if !s.is_null()
        && {
            i = atoi(s);
            i > 0 as libc::c_int
        }
    {
        (*display).d_tcs[0 as libc::c_int as usize].num = i;
    }
    if wi != 0 {
        (*display).d_tcs[1 as libc::c_int as usize].num = wi;
    }
    if he != 0 {
        (*display).d_tcs[0 as libc::c_int as usize].num = he;
    }
    if (*display).d_tcs[1 as libc::c_int as usize].num <= 0 as libc::c_int {
        (*display).d_tcs[1 as libc::c_int as usize].num = 80 as libc::c_int;
    }
    if (*display).d_tcs[0 as libc::c_int as usize].num <= 0 as libc::c_int {
        (*display).d_tcs[0 as libc::c_int as usize].num = 24 as libc::c_int;
    }
    if (*display).d_tcs[95 as libc::c_int as usize].flg != 0 {
        if ((*display).d_tcs[60 as libc::c_int as usize].str_0).is_null()
            && !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null()
            && (!(InStr(
                (*display).d_tcs[55 as libc::c_int as usize].str_0,
                b"\x1B[m\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
                || !(InStr(
                    (*display).d_tcs[55 as libc::c_int as usize].str_0,
                    b"\x1B[0m\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null())
        {
            (*display)
                .d_tcs[60 as libc::c_int as usize]
                .str_0 = b"\x1B[3%p1%dm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            (*display)
                .d_tcs[61 as libc::c_int as usize]
                .str_0 = b"\x1B[4%p1%dm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if !((*display).d_tcs[64 as libc::c_int as usize].str_0).is_null()
            && !(InStr(
                (*display).d_tcs[64 as libc::c_int as usize].str_0,
                b"\x1B[39;49m\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
        {
            (*display).d_tcs[67 as libc::c_int as usize].flg = 1 as libc::c_int;
        }
        if !((*display).d_tcs[64 as libc::c_int as usize].str_0).is_null()
            && (!(InStr(
                (*display).d_tcs[64 as libc::c_int as usize].str_0,
                b"\x1B[m\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
                || !(InStr(
                    (*display).d_tcs[64 as libc::c_int as usize].str_0,
                    b"\x1B[0m\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null())
        {
            (*display).d_tcs[64 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        }
        if !((*display).d_tcs[104 as libc::c_int as usize].str_0).is_null()
            && !(InStr(
                (*display).d_tcs[104 as libc::c_int as usize].str_0,
                b"\x1B(B\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            || !((*display).d_tcs[101 as libc::c_int as usize].str_0).is_null()
                && !(InStr(
                    (*display).d_tcs[101 as libc::c_int as usize].str_0,
                    b"\x1B(0\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
        {
            (*display).d_tcs[97 as libc::c_int as usize].flg = 1 as libc::c_int;
        }
        if !(InStr(
            ((*display).d_termname).as_mut_ptr(),
            b"xterm\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
            || !(InStr(
                ((*display).d_termname).as_mut_ptr(),
                b"rxvt\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            || !((*display).d_tcs[106 as libc::c_int as usize].str_0).is_null()
                && (!(InStr(
                    (*display).d_tcs[106 as libc::c_int as usize].str_0,
                    b"\x1B[M\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                    || !(InStr(
                        (*display).d_tcs[106 as libc::c_int as usize].str_0,
                        b"\x1B[<\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null())
        {
            (*display).d_tcs[96 as libc::c_int as usize].flg = 1 as libc::c_int;
            let ref mut fresh0 = *kmapdef.as_mut_ptr().offset(0 as libc::c_int as isize);
            *fresh0 = if !((*display).d_tcs[106 as libc::c_int as usize].str_0).is_null()
            {
                SaveStr((*display).d_tcs[106 as libc::c_int as usize].str_0)
            } else {
                0 as *mut libc::c_char
            };
        }
        if (*display).d_tcs[96 as libc::c_int as usize].flg != 0 {
            (*display).d_tcs[66 as libc::c_int as usize].flg = 1 as libc::c_int;
        }
    }
    if nwin_options.flowflag == nwin_undef.flowflag {
        nwin_default
            .flowflag = if (*display).d_tcs[88 as libc::c_int as usize].flg != 0 {
            ((1 as libc::c_int) << 0 as libc::c_int) * 0 as libc::c_int
        } else if (*display).d_tcs[89 as libc::c_int as usize].flg != 0 {
            ((1 as libc::c_int) << 0 as libc::c_int) * 1 as libc::c_int
        } else {
            (1 as libc::c_int) << 2 as libc::c_int
        };
    }
    (*display).d_tcs[87 as libc::c_int as usize].flg
        |= ((*display).d_tcs[83 as libc::c_int as usize].flg == 0
            || (*display).d_tcs[84 as libc::c_int as usize].flg != 0
            || (*display).d_tcs[85 as libc::c_int as usize].flg != 0) as libc::c_int;
    if ((*display).d_tcs[42 as libc::c_int as usize].str_0).is_null() {
        (*display)
            .d_tcs[42 as libc::c_int as usize]
            .str_0 = b"\x07\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if ((*display).d_tcs[13 as libc::c_int as usize].str_0).is_null() {
        if (*display).d_tcs[12 as libc::c_int as usize].flg != 0 {
            (*display)
                .d_tcs[13 as libc::c_int as usize]
                .str_0 = b"\x08\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            (*display)
                .d_tcs[13 as libc::c_int as usize]
                .str_0 = (*display).d_tcs[14 as libc::c_int as usize].str_0;
        }
    }
    if ((*display).d_tcs[7 as libc::c_int as usize].str_0).is_null() {
        (*display)
            .d_tcs[7 as libc::c_int as usize]
            .str_0 = b"\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if ((*display).d_tcs[19 as libc::c_int as usize].str_0).is_null() {
        (*display)
            .d_tcs[19 as libc::c_int as usize]
            .str_0 = b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*display).d_tcs[58 as libc::c_int as usize].num > 0 as libc::c_int {
        (*display).d_tcs[53 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[48 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[53 as libc::c_int as usize].str_0;
    }
    if (*display).d_tcs[57 as libc::c_int as usize].num > 0 as libc::c_int {
        (*display).d_tcs[54 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[51 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[54 as libc::c_int as usize].str_0;
    }
    if (*display).d_tcs[58 as libc::c_int as usize].num > 0 as libc::c_int
        && (*display).d_tcs[57 as libc::c_int as usize].num > 0 as libc::c_int
    {
        (*display).d_tcs[55 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[52 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[55 as libc::c_int as usize].str_0;
        (*display)
            .d_tcs[50 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[52 as libc::c_int as usize].str_0;
        (*display)
            .d_tcs[49 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[50 as libc::c_int as usize].str_0;
        (*display)
            .d_tcs[47 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[49 as libc::c_int as usize].str_0;
    }
    xue = (1 as libc::c_int) << 2 as libc::c_int;
    xse = (1 as libc::c_int) << 1 as libc::c_int;
    xme = (1 as libc::c_int) << 0 as libc::c_int;
    if !((*display).d_tcs[51 as libc::c_int as usize].str_0).is_null()
        && ((*display).d_tcs[54 as libc::c_int as usize].str_0).is_null()
    {
        Msg(
            0 as libc::c_int,
            b"Warning: 'so' but no 'se' capability.\0" as *const u8
                as *const libc::c_char,
        );
        if !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null() {
            xse = xme;
        } else {
            (*display).d_tcs[51 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        }
    }
    if !((*display).d_tcs[48 as libc::c_int as usize].str_0).is_null()
        && ((*display).d_tcs[53 as libc::c_int as usize].str_0).is_null()
    {
        Msg(
            0 as libc::c_int,
            b"Warning: 'us' but no 'ue' capability.\0" as *const u8
                as *const libc::c_char,
        );
        if !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null() {
            xue = xme;
        } else {
            (*display).d_tcs[48 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        }
    }
    if (!((*display).d_tcs[47 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[49 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[50 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[52 as libc::c_int as usize].str_0).is_null())
        && ((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null()
    {
        Msg(
            0 as libc::c_int,
            b"Warning: 'm?' but no 'me' capability.\0" as *const u8
                as *const libc::c_char,
        );
        (*display).d_tcs[52 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[50 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[52 as libc::c_int as usize].str_0;
        (*display)
            .d_tcs[49 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[50 as libc::c_int as usize].str_0;
        (*display)
            .d_tcs[47 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[49 as libc::c_int as usize].str_0;
    }
    if !((*display).d_tcs[53 as libc::c_int as usize].str_0).is_null()
        && !((*display).d_tcs[54 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[54 as libc::c_int as usize].str_0,
            (*display).d_tcs[53 as libc::c_int as usize].str_0,
        ) == 0 as libc::c_int
    {
        xse = xue;
    }
    if !((*display).d_tcs[54 as libc::c_int as usize].str_0).is_null()
        && !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[55 as libc::c_int as usize].str_0,
            (*display).d_tcs[54 as libc::c_int as usize].str_0,
        ) == 0 as libc::c_int
    {
        xse = xme;
    }
    if !((*display).d_tcs[53 as libc::c_int as usize].str_0).is_null()
        && !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[55 as libc::c_int as usize].str_0,
            (*display).d_tcs[53 as libc::c_int as usize].str_0,
        ) == 0 as libc::c_int
    {
        xue = xme;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        (*display)
            .d_attrtab[i
            as usize] = (*display).d_tcs[(47 as libc::c_int + i) as usize].str_0;
        (*display)
            .d_attrtyp[i
            as usize] = (if i == 4 as libc::c_int {
            xse
        } else if i == 1 as libc::c_int {
            xue
        } else {
            xme
        }) as libc::c_char;
        i += 1;
        i;
    }
    s = 0 as *mut libc::c_char;
    t = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        s = (*display).d_attrtab[i as usize];
        if !s.is_null() {
            t = (*display).d_attrtyp[i as usize] as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if ((*display).d_attrtab[i as usize]).is_null() {
            (*display).d_attrtab[i as usize] = s;
            (*display).d_attrtyp[i as usize] = t as libc::c_char;
        } else {
            s = (*display).d_attrtab[i as usize];
            t = (*display).d_attrtyp[i as usize] as libc::c_int;
        }
        i += 1;
        i;
    }
    if !((*display).d_tcs[60 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[61 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[62 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[63 as libc::c_int as usize].str_0).is_null()
    {
        (*display).d_hascolor = 1 as libc::c_int;
    }
    if (*display).d_tcs[33 as libc::c_int as usize].flg != 0 {
        (*display).d_tcs[66 as libc::c_int as usize].flg = 1 as libc::c_int;
    }
    if ((*display).d_tcs[10 as libc::c_int as usize].str_0).is_null() {
        (*display)
            .d_tcs[10 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[19 as libc::c_int as usize].str_0;
    }
    if ((*display).d_tcs[20 as libc::c_int as usize].str_0).is_null() {
        (*display)
            .d_tcs[20 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[19 as libc::c_int as usize].str_0;
    }
    if (*display).d_tcs[26 as libc::c_int as usize].flg != 0 {
        (*display).d_tcs[27 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[29 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[27 as libc::c_int as usize].str_0;
    }
    if ((*display).d_tcs[28 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[27 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null()
        && !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[29 as libc::c_int as usize].str_0,
            (*display).d_tcs[27 as libc::c_int as usize].str_0,
        ) == 0 as libc::c_int
    {
        (*display).d_tcs[29 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if ((*display).d_tcs[70 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[69 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if ((*display).d_tcs[94 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[93 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if ((*display).d_tcs[82 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[81 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[80 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[81 as libc::c_int as usize].str_0;
    }
    if ((*display).d_tcs[72 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[71 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if (*display).d_tcs[97 as libc::c_int as usize].flg != 0 {
        if ((*display).d_tcs[98 as libc::c_int as usize].str_0).is_null() {
            (*display)
                .d_tcs[98 as libc::c_int as usize]
                .str_0 = b"\x1B(%p1%c\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if ((*display).d_tcs[99 as libc::c_int as usize].str_0).is_null() {
            (*display)
                .d_tcs[99 as libc::c_int as usize]
                .str_0 = b"\x1B(B\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        (*display).d_tcs[103 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display).d_tcs[104 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    } else if !((*display).d_tcs[103 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[101 as libc::c_int as usize].str_0).is_null()
            && !((*display).d_tcs[102 as libc::c_int as usize].str_0).is_null()
    {
        (*display)
            .d_tcs[98 as libc::c_int as usize]
            .str_0 = (if !((*display).d_tcs[101 as libc::c_int as usize].str_0).is_null()
            && !((*display).d_tcs[102 as libc::c_int as usize].str_0).is_null()
        {
            (*display).d_tcs[101 as libc::c_int as usize].str_0 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        (*display)
            .d_tcs[99 as libc::c_int as usize]
            .str_0 = (if !((*display).d_tcs[101 as libc::c_int as usize].str_0).is_null()
            && !((*display).d_tcs[102 as libc::c_int as usize].str_0).is_null()
        {
            (*display).d_tcs[102 as libc::c_int as usize].str_0 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        (*display)
            .d_tcs[100 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[103 as libc::c_int as usize].str_0;
    } else {
        (*display)
            .d_tcs[99 as libc::c_int as usize]
            .str_0 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*display)
            .d_tcs[98 as libc::c_int as usize]
            .str_0 = (*display).d_tcs[99 as libc::c_int as usize].str_0;
        (*display).d_tcs[100 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
        (*display)
            .d_tcs[103 as libc::c_int as usize]
            .str_0 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*display).d_c0_tab[i as usize] = i as libc::c_char;
        i += 1;
        i;
    }
    if !((*display).d_tcs[103 as libc::c_int as usize].str_0).is_null() {
        s = b"l+m+k+j+u+t+v+w+q-x|n+o~s_p\"r#`+a:f'g#~o.v-^+<,>h#I#0#y<z>\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        i = ((strlen(s)).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            & !(1 as libc::c_int) as libc::c_ulong) as libc::c_int;
        while i >= 0 as libc::c_int {
            (*display)
                .d_c0_tab[*s.offset(i as isize) as libc::c_uchar as libc::c_int
                as usize] = *s.offset((i + 1 as libc::c_int) as isize);
            i -= 2 as libc::c_int;
        }
    }
    if !((*display).d_tcs[100 as libc::c_int as usize].str_0).is_null() {
        i = ((strlen((*display).d_tcs[100 as libc::c_int as usize].str_0))
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            & !(1 as libc::c_int) as libc::c_ulong) as libc::c_int;
        while i >= 0 as libc::c_int {
            (*display)
                .d_c0_tab[*((*display).d_tcs[100 as libc::c_int as usize].str_0)
                .offset(i as isize) as libc::c_uchar as libc::c_int
                as usize] = *((*display).d_tcs[100 as libc::c_int as usize].str_0)
                .offset((i + 1 as libc::c_int) as isize);
            i -= 2 as libc::c_int;
        }
    }
    if ((*display).d_tcs[74 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[73 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if !((*display).d_tcs[105 as libc::c_int as usize].str_0).is_null() {
        if CreateTransTable((*display).d_tcs[105 as libc::c_int as usize].str_0) != 0 {
            return -(1 as libc::c_int);
        }
    }
    if ((*display).d_tcs[46 as libc::c_int as usize].str_0).is_null() {
        (*display).d_tcs[45 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    CheckScreenSize(0 as libc::c_int);
    if ((*display).d_tcs[77 as libc::c_int as usize].str_0).is_null()
        || ((*display).d_tcs[78 as libc::c_int as usize].str_0).is_null()
        || ((*display).d_tcs[79 as libc::c_int as usize].str_0).is_null()
    {
        (*display).d_tcs[75 as libc::c_int as usize].flg = 0 as libc::c_int;
    }
    if (*display).d_tcs[75 as libc::c_int as usize].flg != 0 {
        if (*display).d_tcs[76 as libc::c_int as usize].num < 0 as libc::c_int {
            (*display).d_tcs[76 as libc::c_int as usize].num = 0 as libc::c_int;
        }
    }
    (*display).d_has_hstatus = hardstatusemu & !((1 as libc::c_int) << 3 as libc::c_int);
    if (*display).d_tcs[75 as libc::c_int as usize].flg != 0
        && hardstatusemu & (1 as libc::c_int) << 3 as libc::c_int == 0
    {
        (*display).d_has_hstatus = 3 as libc::c_int;
    }
    if !((*display).d_tcs[92 as libc::c_int as usize].str_0).is_null() {
        let mut enc: libc::c_int = FindEncoding(
            (*display).d_tcs[92 as libc::c_int as usize].str_0,
        );
        if enc != -(1 as libc::c_int) {
            (*display).d_encoding = enc;
        }
    }
    if ((*display).d_tcs[158 as libc::c_int as usize].str_0).is_null()
        && !((*display).d_tcs[(158 as libc::c_int + 1 as libc::c_int) as usize].str_0)
            .is_null()
    {
        (*display)
            .d_tcs[158 as libc::c_int as usize]
            .str_0 = (*display)
            .d_tcs[(158 as libc::c_int + 1 as libc::c_int) as usize]
            .str_0;
    }
    if ((*display).d_tcs[(158 as libc::c_int + 2 as libc::c_int) as usize].str_0)
        .is_null()
        && !((*display).d_tcs[(158 as libc::c_int + 3 as libc::c_int) as usize].str_0)
            .is_null()
    {
        (*display)
            .d_tcs[(158 as libc::c_int + 2 as libc::c_int) as usize]
            .str_0 = (*display)
            .d_tcs[(158 as libc::c_int + 3 as libc::c_int) as usize]
            .str_0;
    }
    (*display).d_UPcost = CalcCost((*display).d_tcs[8 as libc::c_int as usize].str_0);
    (*display).d_DOcost = CalcCost((*display).d_tcs[10 as libc::c_int as usize].str_0);
    (*display).d_NLcost = CalcCost((*display).d_tcs[19 as libc::c_int as usize].str_0);
    (*display).d_LEcost = CalcCost((*display).d_tcs[13 as libc::c_int as usize].str_0);
    (*display).d_NDcost = CalcCost((*display).d_tcs[16 as libc::c_int as usize].str_0);
    (*display).d_CRcost = CalcCost((*display).d_tcs[7 as libc::c_int as usize].str_0);
    (*display).d_IMcost = CalcCost((*display).d_tcs[27 as libc::c_int as usize].str_0);
    (*display).d_EIcost = CalcCost((*display).d_tcs[28 as libc::c_int as usize].str_0);
    if (*display).d_tcs[90 as libc::c_int as usize].flg != 0 {
        (*display).d_auto_nuke = 1 as libc::c_int;
    }
    if (*display).d_tcs[91 as libc::c_int as usize].num > 0 as libc::c_int {
        (*display).d_obufmax = (*display).d_tcs[91 as libc::c_int as usize].num;
        (*display).d_obuflenmax = (*display).d_obuflen - (*display).d_obufmax;
    }
    if !((*display).d_tcs[106 as libc::c_int as usize].str_0).is_null()
        && !((*display).d_tcs[(106 as libc::c_int + 10 as libc::c_int) as usize].str_0)
            .is_null()
        && strcmp(
            (*display).d_tcs[106 as libc::c_int as usize].str_0,
            (*display).d_tcs[(106 as libc::c_int + 10 as libc::c_int) as usize].str_0,
        ) == 0
    {
        (*display).d_tcs[106 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if !((*display).d_tcs[165 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[165 as libc::c_int as usize].str_0,
            b"\x0F7\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        (*display).d_tcs[165 as libc::c_int as usize].str_0 = 0 as *mut libc::c_char;
    }
    if !((*display).d_tcs[(166 as libc::c_int + 3 as libc::c_int) as usize].str_0)
        .is_null()
        && strcmp(
            (*display).d_tcs[(166 as libc::c_int + 3 as libc::c_int) as usize].str_0,
            b"\08\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        (*display)
            .d_tcs[(166 as libc::c_int + 3 as libc::c_int) as usize]
            .str_0 = 0 as *mut libc::c_char;
    }
    (*display).d_nseqs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 188 as libc::c_int - 106 as libc::c_int {
        remap(i, 1 as libc::c_int);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < kmap_extn {
        remap(
            i
                + (188 as libc::c_int - 106 as libc::c_int
                    + (188 as libc::c_int - 166 as libc::c_int)),
            1 as libc::c_int,
        );
        i += 1;
        i;
    }
    (*display).d_seqp = ((*display).d_kmaps).offset(3 as libc::c_int as isize);
    (*display).d_seql = 0 as libc::c_int;
    (*display).d_seqh = 0 as *mut libc::c_uchar;
    (*display).d_tcinited = 1 as libc::c_int as libc::c_char;
    MakeTermcap(0 as libc::c_int);
    e_tgetent(tbuf.as_mut_ptr(), ((*display).d_termname).as_mut_ptr());
    CheckEscape();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remap(mut n: libc::c_int, mut map: libc::c_int) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fl: libc::c_int = 0 as libc::c_int;
    let mut domap: libc::c_int = 0 as libc::c_int;
    let mut a1: *mut action = 0 as *mut action;
    let mut a2: *mut action = 0 as *mut action;
    let mut tab: *mut action = 0 as *mut action;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut kme: *mut kmap_ext = 0 as *mut kmap_ext;
    a1 = 0 as *mut action;
    if n
        >= 188 as libc::c_int - 106 as libc::c_int
            + (188 as libc::c_int - 166 as libc::c_int)
    {
        kme = kmap_exts
            .offset(
                (n
                    - (188 as libc::c_int - 106 as libc::c_int
                        + (188 as libc::c_int - 166 as libc::c_int))) as isize,
            );
        s = (*kme).str_0;
        l = (*kme).fl & !(0x4000 as libc::c_int);
        fl = (*kme).fl & 0x4000 as libc::c_int;
        a1 = &mut (*kme).um;
    }
    tab = umtab.as_mut_ptr();
    loop {
        a2 = 0 as *mut action;
        if n
            < 188 as libc::c_int - 106 as libc::c_int
                + (188 as libc::c_int - 166 as libc::c_int)
        {
            a1 = &mut *tab.offset(n as isize) as *mut action;
            if n >= 188 as libc::c_int - 106 as libc::c_int {
                n -= 188 as libc::c_int - 166 as libc::c_int;
            }
            s = (*display).d_tcs[(n + 106 as libc::c_int) as usize].str_0;
            l = (if !s.is_null() {
                strlen(s)
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as libc::c_int;
            if n >= 166 as libc::c_int - 106 as libc::c_int {
                a2 = &mut *tab
                    .offset((n + (188 as libc::c_int - 166 as libc::c_int)) as isize)
                    as *mut action;
            }
        }
        if s.is_null() || l == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if !a1.is_null() && (*a1).nr == -(1 as libc::c_int) {
            a1 = 0 as *mut action;
        }
        if !a2.is_null() && (*a2).nr == -(1 as libc::c_int) {
            a2 = 0 as *mut action;
        }
        if !a1.is_null() && (*a1).nr == 160 as libc::c_int
            && !(*((*a1).args).offset(0 as libc::c_int as isize)).is_null()
            && strcmp(*((*a1).args).offset(0 as libc::c_int as isize), s)
                == 0 as libc::c_int
        {
            a1 = 0 as *mut action;
        }
        if !a2.is_null() && (*a2).nr == 160 as libc::c_int
            && !(*((*a2).args).offset(0 as libc::c_int as isize)).is_null()
            && strcmp(*((*a2).args).offset(0 as libc::c_int as isize), s)
                == 0 as libc::c_int
        {
            a2 = 0 as *mut action;
        }
        domap |= (!a1.is_null() || !a2.is_null()) as libc::c_int;
        if tab == umtab.as_mut_ptr() {
            tab = dmtab.as_mut_ptr();
            a1 = if !kme.is_null() { &mut (*kme).dm } else { 0 as *mut action };
        } else {
            if !(tab == dmtab.as_mut_ptr()) {
                break;
            }
            tab = mmtab.as_mut_ptr();
            a1 = if !kme.is_null() { &mut (*kme).mm } else { 0 as *mut action };
        }
    }
    if n < 188 as libc::c_int - 106 as libc::c_int {
        domap = 1 as libc::c_int;
    }
    if map == 0 as libc::c_int && domap != 0 {
        return 0 as libc::c_int;
    }
    if map != 0 && domap == 0 {
        return 0 as libc::c_int;
    }
    if map != 0 { return addmapseq(s, l, n | fl) } else { return remmapseq(s, l) };
}
pub unsafe extern "C" fn CheckEscape() {
    let mut odisplay: *mut display = 0 as *mut display;
    let mut i: libc::c_int = 0;
    let mut nr: libc::c_int = 0;
    if DefaultEsc >= 0 as libc::c_int {
        return;
    }
    odisplay = display;
    display = displays;
    while !display.is_null() {
        i = 0 as libc::c_int;
        while i < (*display).d_nseqs {
            nr = ((*((*display).d_kmaps).offset(i as isize) as libc::c_int)
                << 8 as libc::c_int
                | *((*display).d_kmaps).offset((i + 1 as libc::c_int) as isize)
                    as libc::c_int) & !(0x4000 as libc::c_int);
            if nr
                < 188 as libc::c_int - 106 as libc::c_int
                    + (188 as libc::c_int - 166 as libc::c_int)
            {
                if (*umtab.as_mut_ptr().offset(nr as isize)).nr == 35 as libc::c_int {
                    break;
                }
                if (*umtab.as_mut_ptr().offset(nr as isize)).nr == -(1 as libc::c_int)
                    && (*dmtab.as_mut_ptr().offset(nr as isize)).nr == 35 as libc::c_int
                {
                    break;
                }
            } else {
                let mut kme: *mut kmap_ext = kmap_exts
                    .offset(nr as isize)
                    .offset(
                        -((188 as libc::c_int - 106 as libc::c_int
                            + (188 as libc::c_int - 166 as libc::c_int)) as isize),
                    );
                if (*kme).um.nr == 35 as libc::c_int {
                    break;
                }
                if (*kme).um.nr == -(1 as libc::c_int)
                    && (*kme).dm.nr == 35 as libc::c_int
                {
                    break;
                }
            }
            i
                += *((*display).d_kmaps).offset((i + 2 as libc::c_int) as isize)
                    as libc::c_int * 2 as libc::c_int + 4 as libc::c_int;
        }
        display = (*display).d_next;
    }
    if display.is_null() {
        display = odisplay;
        return;
    }
    SetEscape(0 as *mut acluser, 'a' as i32 & 0o37 as libc::c_int, 'a' as i32);
    if (*(*odisplay).d_user).u_Esc == -(1 as libc::c_int) {
        (*(*odisplay).d_user).u_Esc = DefaultEsc;
    }
    if (*(*odisplay).d_user).u_MetaEsc == -(1 as libc::c_int) {
        (*(*odisplay).d_user).u_MetaEsc = DefaultMetaEsc;
    }
    display = 0 as *mut display;
    Msg(
        0 as libc::c_int,
        b"Warning: escape char set back to ^A\0" as *const u8 as *const libc::c_char,
    );
    display = odisplay;
}
unsafe extern "C" fn findseq_ge(
    mut seq: *mut libc::c_char,
    mut k: libc::c_int,
    mut sp: *mut *mut libc::c_uchar,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    p = (*display).d_kmaps;
    while (p.offset_from((*display).d_kmaps) as libc::c_long)
        < (*display).d_nseqs as libc::c_long
    {
        l = *p.offset(2 as libc::c_int as isize) as libc::c_int;
        p = p.offset(3 as libc::c_int as isize);
        j = 0 as libc::c_int;
        loop {
            if j == k || j == l {
                j = l - k;
                break;
            } else if *p.offset(j as isize) as libc::c_int
                != *(seq as *mut libc::c_uchar).offset(j as isize) as libc::c_int
            {
                j = *p.offset(j as isize) as libc::c_int
                    - *(seq as *mut libc::c_uchar).offset(j as isize) as libc::c_int;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if j >= 0 as libc::c_int {
            *sp = p.offset(-(3 as libc::c_int as isize));
            return j;
        }
        p = p.offset((2 as libc::c_int * l + 1 as libc::c_int) as isize);
    }
    *sp = p;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn setseqoff(
    mut p: *mut libc::c_uchar,
    mut i: libc::c_int,
    mut o: libc::c_int,
) {
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut l: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = *p.offset(2 as libc::c_int as isize) as libc::c_int;
    if o < 256 as libc::c_int {
        *p.offset((k + 4 as libc::c_int + i) as isize) = o as libc::c_uchar;
        return;
    }
    q = p.offset((k * 2 as libc::c_int) as isize).offset(4 as libc::c_int as isize);
    loop {
        l = *q.offset(2 as libc::c_int as isize) as libc::c_int;
        if q.offset((l * 2 as libc::c_int) as isize).offset_from(p) as libc::c_long
            / 2 as libc::c_int as libc::c_long >= 256 as libc::c_int as libc::c_long
        {
            *p
                .offset(
                    (k + 4 as libc::c_int + i) as isize,
                ) = ((q.offset_from(p) as libc::c_long
                - 4 as libc::c_int as libc::c_long) / 2 as libc::c_int as libc::c_long)
                as libc::c_uchar;
            return;
        }
        q = q.offset((l * 2 as libc::c_int + 4 as libc::c_int) as isize);
    };
}
unsafe extern "C" fn addmapseq(
    mut seq: *mut libc::c_char,
    mut k: libc::c_int,
    mut nr: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut mo: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if k >= 254 as libc::c_int {
        return -(1 as libc::c_int);
    }
    j = findseq_ge(seq, k, &mut p);
    if j == 0 as libc::c_int {
        *p.offset(0 as libc::c_int as isize) = (nr >> 8 as libc::c_int) as libc::c_uchar;
        *p.offset(1 as libc::c_int as isize) = nr as libc::c_uchar;
        return 0 as libc::c_int;
    }
    i = p.offset_from((*display).d_kmaps) as libc::c_long as libc::c_int;
    if (*display).d_nseqs + 2 as libc::c_int * k + 4 as libc::c_int >= (*display).d_aseqs
    {
        (*display)
            .d_kmaps = xrealloc(
            (*display).d_kmaps as *mut libc::c_char,
            (*display).d_aseqs + 256 as libc::c_int,
        ) as *mut libc::c_uchar;
        (*display).d_aseqs += 256 as libc::c_int;
        p = ((*display).d_kmaps).offset(i as isize);
    }
    (*display).d_seqp = ((*display).d_kmaps).offset(3 as libc::c_int as isize);
    (*display).d_seql = 0 as libc::c_int;
    (*display).d_seqh = 0 as *mut libc::c_uchar;
    evdeq(&mut (*display).d_mapev);
    if j > 0 as libc::c_int {
        bcopy(
            p as *mut libc::c_char as *const libc::c_void,
            (p as *mut libc::c_char)
                .offset((2 as libc::c_int * k) as isize)
                .offset(4 as libc::c_int as isize) as *mut libc::c_void,
            ((*display).d_nseqs - i) as size_t,
        );
    }
    *p.offset(0 as libc::c_int as isize) = (nr >> 8 as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) = nr as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) = k as libc::c_uchar;
    bcopy(
        seq as *const libc::c_void,
        (p as *mut libc::c_char).offset(3 as libc::c_int as isize) as *mut libc::c_void,
        k as size_t,
    );
    bzero(
        p.offset(k as isize).offset(3 as libc::c_int as isize) as *mut libc::c_void,
        (k + 1 as libc::c_int) as libc::c_ulong,
    );
    (*display).d_nseqs += 2 as libc::c_int * k + 4 as libc::c_int;
    if j > 0 as libc::c_int {
        q = p.offset((2 as libc::c_int * k) as isize).offset(4 as libc::c_int as isize);
        l = *q.offset(2 as libc::c_int as isize) as libc::c_int;
        i = 0 as libc::c_int;
        while i < k {
            if *p.offset((3 as libc::c_int + i) as isize) as libc::c_int
                != *q.offset((3 as libc::c_int + i) as isize) as libc::c_int
            {
                *p.offset((k + 4 as libc::c_int + i) as isize) = k as libc::c_uchar;
                break;
            } else {
                setseqoff(
                    p,
                    i,
                    if *q.offset((l + 4 as libc::c_int + i) as isize) as libc::c_int != 0
                    {
                        *q.offset((l + 4 as libc::c_int + i) as isize) as libc::c_int + k
                            + 2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                );
                i += 1;
                i;
            }
        }
    }
    q = (*display).d_kmaps;
    while q < p {
        l = *q.offset(2 as libc::c_int as isize) as libc::c_int;
        j = 0 as libc::c_int;
        m = j;
        while j < l {
            mo = m;
            if m == 0
                && *q.offset((3 as libc::c_int + j) as isize) as libc::c_int
                    != *seq.offset(j as isize) as libc::c_int
            {
                m = 1 as libc::c_int;
            }
            if *q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int
                == 0 as libc::c_int
            {
                if mo == 0 && m != 0 {
                    setseqoff(
                        q,
                        j,
                        ((p.offset_from(q) as libc::c_long
                            - 4 as libc::c_int as libc::c_long)
                            / 2 as libc::c_int as libc::c_long) as libc::c_int,
                    );
                }
            } else if q
                .offset(
                    (*q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int
                        * 2 as libc::c_int) as isize,
                )
                .offset(4 as libc::c_int as isize) > p
                || q
                    .offset(
                        (*q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int
                            * 2 as libc::c_int) as isize,
                    )
                    .offset(4 as libc::c_int as isize) == p && m == 0
            {
                setseqoff(
                    q,
                    j,
                    *q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int + k
                        + 2 as libc::c_int,
                );
            }
            j += 1;
            j;
        }
        q = q.offset((2 as libc::c_int * l + 4 as libc::c_int) as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn remmapseq(
    mut seq: *mut libc::c_char,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if k >= 254 as libc::c_int
        || {
            j = findseq_ge(seq, k, &mut p);
            j != 0 as libc::c_int
        }
    {
        return -(1 as libc::c_int);
    }
    q = (*display).d_kmaps;
    while q < p {
        l = *q.offset(2 as libc::c_int as isize) as libc::c_int;
        j = 0 as libc::c_int;
        while j < l {
            if q
                .offset(
                    (*q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int
                        * 2 as libc::c_int) as isize,
                )
                .offset(4 as libc::c_int as isize) == p
            {
                setseqoff(
                    q,
                    j,
                    if *p.offset((k + 4 as libc::c_int + j) as isize) as libc::c_int != 0
                    {
                        *q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int
                            + *p.offset((k + 4 as libc::c_int + j) as isize)
                                as libc::c_int - k
                    } else {
                        0 as libc::c_int
                    },
                );
            } else if q
                .offset(
                    (*q.offset((l + 4 as libc::c_int + j) as isize) as libc::c_int
                        * 2 as libc::c_int) as isize,
                )
                .offset(4 as libc::c_int as isize) > p
            {
                let ref mut fresh1 = *q.offset((l + 4 as libc::c_int + j) as isize);
                *fresh1 = (*fresh1 as libc::c_int - (k + 2 as libc::c_int))
                    as libc::c_uchar;
            }
            j += 1;
            j;
        }
        q = q.offset((2 as libc::c_int * l + 4 as libc::c_int) as isize);
    }
    if ((*display).d_kmaps).offset((*display).d_nseqs as isize)
        > p.offset((2 as libc::c_int * k) as isize).offset(4 as libc::c_int as isize)
    {
        bcopy(
            (p as *mut libc::c_char)
                .offset((2 as libc::c_int * k) as isize)
                .offset(4 as libc::c_int as isize) as *const libc::c_void,
            p as *mut libc::c_char as *mut libc::c_void,
            ((*display).d_kmaps)
                .offset((*display).d_nseqs as isize)
                .offset_from(
                    p
                        .offset((2 as libc::c_int * k) as isize)
                        .offset(4 as libc::c_int as isize),
                ) as libc::c_long as size_t,
        );
    }
    (*display).d_nseqs -= 2 as libc::c_int * k + 4 as libc::c_int;
    (*display).d_seqp = ((*display).d_kmaps).offset(3 as libc::c_int as isize);
    (*display).d_seql = 0 as libc::c_int;
    (*display).d_seqh = 0 as *mut libc::c_uchar;
    evdeq(&mut (*display).d_mapev);
    return 0 as libc::c_int;
}
unsafe extern "C" fn AddCap(mut s: *mut libc::c_char) {
    let mut n: libc::c_int = 0;
    n = strlen(s) as libc::c_int;
    if Termcaplen + n < 1023 as libc::c_int - 1 as libc::c_int {
        strcpy(Termcap.as_mut_ptr().offset(Termcaplen as isize), s);
        Termcaplen += n;
        tcLineLen += n;
    }
}
pub unsafe extern "C" fn MakeTermcap(mut aflag: libc::c_int) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 1023] = [0; 1023];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut tname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut wi: libc::c_int = 0;
    let mut he: libc::c_int = 0;
    if !display.is_null() {
        wi = (*display).d_width;
        he = (*display).d_height;
        tname = ((*display).d_termname).as_mut_ptr();
    } else {
        wi = 80 as libc::c_int;
        he = 24 as libc::c_int;
        tname = b"vt100\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    s = getenv(b"SCREENCAP\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && strlen(s) < 1023 as libc::c_int as libc::c_ulong {
        sprintf(
            Termcap.as_mut_ptr(),
            b"TERMCAP=%s\0" as *const u8 as *const libc::c_char,
            s,
        );
        strcpy(Term.as_mut_ptr(), b"TERM=screen\0" as *const u8 as *const libc::c_char);
        return Termcap.as_mut_ptr();
    }
    Termcaplen = 0 as libc::c_int;
    if *screenterm.as_mut_ptr() as libc::c_int == '\0' as i32
        || strlen(screenterm.as_mut_ptr())
            > (768 as libc::c_int - 3 as libc::c_int) as libc::c_ulong
    {
        strncpy(
            screenterm.as_mut_ptr(),
            b"screen\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_ulong,
        );
        screenterm[32 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    let mut current_block_34: u64;
    strcpy(Term.as_mut_ptr(), b"TERM=\0" as *const u8 as *const libc::c_char);
    p = Term.as_mut_ptr().offset(5 as libc::c_int as isize);
    if aflag == 0
        && (strlen(screenterm.as_mut_ptr())).wrapping_add(strlen(tname))
            < (768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
    {
        sprintf(
            p,
            b"%s.%s\0" as *const u8 as *const libc::c_char,
            screenterm.as_mut_ptr(),
            tname,
        );
        if e_tgetent(buf.as_mut_ptr(), p) == 1 as libc::c_int {
            current_block_34 = 7333393191927787629;
        } else {
            current_block_34 = 4775909272756257391;
        }
    } else {
        current_block_34 = 4775909272756257391;
    }
    match current_block_34 {
        4775909272756257391 => {
            if nwin_default.bce != 0 {
                sprintf(
                    p,
                    b"%s-bce\0" as *const u8 as *const libc::c_char,
                    screenterm.as_mut_ptr(),
                );
                if e_tgetent(buf.as_mut_ptr(), p) == 1 as libc::c_int {
                    current_block_34 = 7333393191927787629;
                } else {
                    current_block_34 = 652864300344834934;
                }
            } else {
                current_block_34 = 652864300344834934;
            }
            match current_block_34 {
                7333393191927787629 => {}
                _ => {
                    strcpy(p, screenterm.as_mut_ptr());
                    if !(e_tgetent(buf.as_mut_ptr(), p) == 1 as libc::c_int) {
                        strcpy(p, b"vt100\0" as *const u8 as *const libc::c_char);
                    }
                }
            }
        }
        _ => {}
    }
    tcLineLen = 100 as libc::c_int;
    if strlen(Term.as_mut_ptr())
        > (1023 as libc::c_int - 40 as libc::c_int) as libc::c_ulong
    {
        strcpy(Term.as_mut_ptr(), b"too_long\0" as *const u8 as *const libc::c_char);
    }
    sprintf(
        Termcap.as_mut_ptr(),
        b"TERMCAP=SC|%s|VT 100/ANSI X3.64 virtual terminal:\0" as *const u8
            as *const libc::c_char,
        Term.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    Termcaplen = strlen(Termcap.as_mut_ptr()) as libc::c_int;
    if !extra_outcap.is_null() && *extra_outcap as libc::c_int != 0 {
        cp = extra_outcap;
        loop {
            p = index(cp, ':' as i32);
            if p.is_null() {
                break;
            }
            p = p.offset(1);
            ch = *p;
            *p = '\0' as i32 as libc::c_char;
            AddCap(cp);
            *p = ch;
            cp = p;
        }
        tcLineLen = 100 as libc::c_int;
    }
    if (Termcaplen as libc::c_ulong).wrapping_add(strlen(TermcapConst.as_ptr()))
        < 1023 as libc::c_int as libc::c_ulong
    {
        strcpy(
            Termcap.as_mut_ptr().offset(Termcaplen as isize),
            TermcapConst.as_ptr() as *mut libc::c_char,
        );
        Termcaplen = (Termcaplen as libc::c_ulong)
            .wrapping_add(strlen(TermcapConst.as_ptr())) as libc::c_int as libc::c_int;
    }
    sprintf(
        buf.as_mut_ptr(),
        b"li#%d:co#%d:\0" as *const u8 as *const libc::c_char,
        he,
        wi,
    );
    AddCap(buf.as_mut_ptr());
    AddCap(b"am:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if aflag != 0
        || force_vt != 0 && (*display).d_tcs[86 as libc::c_int as usize].flg == 0
        || (*display).d_tcs[87 as libc::c_int as usize].flg != 0
        || (*display).d_tcs[83 as libc::c_int as usize].flg == 0
    {
        AddCap(b"xn:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(b"xv:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(b"LP:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    if aflag != 0
        || !((*display).d_tcs[18 as libc::c_int as usize].str_0).is_null()
            && !((*display).d_tcs[21 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[22 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[23 as libc::c_int as usize].str_0).is_null()
    {
        AddCap(b"sr=\\EM:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(b"al=\\E[L:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(
            b"AL=\\E[%dL:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if !((*display).d_tcs[21 as libc::c_int as usize].str_0).is_null() {
        AddCap(b"sr=\\EM:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    if aflag != 0 || !((*display).d_tcs[18 as libc::c_int as usize].str_0).is_null() {
        AddCap(
            b"cs=\\E[%i%d;%dr:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if aflag != 0 || !((*display).d_tcs[18 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[24 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[25 as libc::c_int as usize].str_0).is_null()
    {
        AddCap(b"dl=\\E[M:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(
            b"DL=\\E[%dM:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if aflag != 0 || !((*display).d_tcs[31 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[32 as libc::c_int as usize].str_0).is_null()
    {
        AddCap(b"dc=\\E[P:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(
            b"DC=\\E[%dP:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if aflag != 0 || !((*display).d_tcs[30 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null()
    {
        AddCap(b"im=\\E[4h:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(b"ei=\\E[4l:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(b"mi:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        AddCap(
            b"IC=\\E[%d@:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    AddCap(
        b"ks=\\E[?1h\\E=:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AddCap(
        b"ke=\\E[?1l\\E>:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AddCap(b"vi=\\E[?25l:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    AddCap(
        b"ve=\\E[34h\\E[?25h:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AddCap(b"vs=\\E[34l:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    AddCap(b"ti=\\E[?1049h:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    AddCap(b"te=\\E[?1049l:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !display.is_null() {
        if !((*display).d_tcs[48 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"us=\\E[4m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            AddCap(
                b"ue=\\E[24m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[51 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"so=\\E[3m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            AddCap(
                b"se=\\E[23m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[52 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"mb=\\E[5m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[49 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"md=\\E[1m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[47 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"mh=\\E[2m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[50 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"mr=\\E[7m:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[52 as libc::c_int as usize].str_0).is_null()
            || !((*display).d_tcs[49 as libc::c_int as usize].str_0).is_null()
            || !((*display).d_tcs[47 as libc::c_int as usize].str_0).is_null()
            || !((*display).d_tcs[50 as libc::c_int as usize].str_0).is_null()
        {
            AddCap(
                b"me=\\E[m:ms:\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if (*display).d_hascolor != 0 {
            AddCap(
                b"Co#8:pa#64:AF=\\E[3%dm:AB=\\E[4%dm:op=\\E[39;49m:AX:\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[43 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"vb=\\Eg:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*display).d_tcs[97 as libc::c_int as usize].flg != 0 {
            AddCap(b"G0:\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        if !((*display).d_tcs[100 as libc::c_int as usize].str_0).is_null()
            || !((*display).d_tcs[98 as libc::c_int as usize].str_0).is_null()
                && *(*display).d_tcs[98 as libc::c_int as usize].str_0 as libc::c_int
                    != 0
        {
            AddCap(
                b"as=\\E(0:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            AddCap(
                b"ae=\\E(B:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            AddCap(
                b"ac=\\140\\140aaffggjjkkllmmnnooppqqrrssttuuvvwwxxyyzz{{||}}~~..--++,,hhII00:\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[73 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"po=\\E[5i:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            AddCap(
                b"pf=\\E[4i:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[45 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"Z0=\\E[?3h:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            AddCap(
                b"Z1=\\E[?3l:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null() {
            AddCap(
                b"WS=\\E[8;%d;%dt:\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    let mut current_block_161: u64;
    i = 106 as libc::c_int;
    while i < 201 as libc::c_int {
        let mut act: *mut action = 0 as *mut action;
        if i < 188 as libc::c_int {
            if i >= 170 as libc::c_int {
                current_block_161 = 5195798230510548452;
            } else if i >= 120 as libc::c_int && i < 140 as libc::c_int {
                current_block_161 = 5195798230510548452;
            } else if i > 140 as libc::c_int && i < 158 as libc::c_int {
                current_block_161 = 5195798230510548452;
            } else {
                if i >= 166 as libc::c_int && i < 188 as libc::c_int {
                    act = &mut *umtab
                        .as_mut_ptr()
                        .offset(
                            (i
                                - (166 as libc::c_int - 188 as libc::c_int
                                    + 106 as libc::c_int)) as isize,
                        ) as *mut action;
                    if (*act).nr == -(1 as libc::c_int) {
                        act = &mut *dmtab
                            .as_mut_ptr()
                            .offset(
                                (i
                                    - (166 as libc::c_int - 188 as libc::c_int
                                        + 106 as libc::c_int)) as isize,
                            ) as *mut action;
                    }
                } else {
                    act = &mut *umtab
                        .as_mut_ptr()
                        .offset((i - 106 as libc::c_int) as isize) as *mut action;
                    if (*act).nr == -(1 as libc::c_int) {
                        act = &mut *dmtab
                            .as_mut_ptr()
                            .offset((i - 106 as libc::c_int) as isize) as *mut action;
                    }
                }
                if (*act).nr == -(1 as libc::c_int)
                    && (i == 158 as libc::c_int + 1 as libc::c_int
                        || i == 158 as libc::c_int + 3 as libc::c_int)
                {
                    act = &mut *umtab
                        .as_mut_ptr()
                        .offset((i - 106 as libc::c_int - 1 as libc::c_int) as isize)
                        as *mut action;
                    if (*act).nr == -(1 as libc::c_int) {
                        act = &mut *dmtab
                            .as_mut_ptr()
                            .offset((i - 106 as libc::c_int - 1 as libc::c_int) as isize)
                            as *mut action;
                    }
                }
                if (*act).nr != -(1 as libc::c_int) {
                    if (*act).nr == 160 as libc::c_int {
                        MakeString(
                            (*term.as_mut_ptr().offset(i as isize)).tcname,
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1023]>()
                                as libc::c_ulong as libc::c_int,
                            *((*act).args).offset(0 as libc::c_int as isize),
                        );
                        AddCap(buf.as_mut_ptr());
                    }
                    current_block_161 = 5195798230510548452;
                } else {
                    current_block_161 = 7370318721998929769;
                }
            }
        } else {
            current_block_161 = 7370318721998929769;
        }
        match current_block_161 {
            7370318721998929769 => {
                if !display.is_null() {
                    match (*term.as_mut_ptr().offset(i as isize)).type_0 {
                        2 => {
                            if !((*display).d_tcs[i as usize].str_0).is_null() {
                                MakeString(
                                    (*term.as_mut_ptr().offset(i as isize)).tcname,
                                    buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 1023]>()
                                        as libc::c_ulong as libc::c_int,
                                    (*display).d_tcs[i as usize].str_0,
                                );
                                AddCap(buf.as_mut_ptr());
                            }
                        }
                        0 => {
                            if !((*display).d_tcs[i as usize].flg == 0 as libc::c_int) {
                                sprintf(
                                    buf.as_mut_ptr(),
                                    b"%s:\0" as *const u8 as *const libc::c_char,
                                    (*term.as_mut_ptr().offset(i as isize)).tcname,
                                );
                                AddCap(buf.as_mut_ptr());
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    return Termcap.as_mut_ptr();
}
pub unsafe extern "C" fn DumpTermcap(mut aflag: libc::c_int, mut f: *mut FILE) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pe: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut col: libc::c_int = 0 as libc::c_int;
    p = index(MakeTermcap(aflag), '=' as i32);
    if p.is_null() {
        return;
    }
    p = p.offset(1);
    p;
    loop {
        pe = index(p, ':' as i32);
        if pe.is_null() {
            break;
        }
        n = (pe.offset_from(p) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if col > 8 as libc::c_int && col + n > 63 as libc::c_int {
            fwrite(
                b"\\\n\t:\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                4 as libc::c_int as libc::c_ulong,
                f,
            );
            col = 8 as libc::c_int;
        }
        fwrite(
            p as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            n as libc::c_ulong,
            f,
        );
        col += n;
        pe = pe.offset(1);
        p = pe;
    }
    if *p != 0 {
        fwrite(
            p as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            strlen(p),
            f,
        );
    }
    fputc('\n' as i32, f);
}
unsafe extern "C" fn MakeString(
    mut cap: *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut buflen: libc::c_int,
    mut s: *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pmax: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_uint = 0;
    p = buf;
    pmax = p
        .offset(buflen as isize)
        .offset(-((3 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int) as isize));
    let fresh2 = cap;
    cap = cap.offset(1);
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = *fresh2;
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = *cap;
    let fresh5 = p;
    p = p.offset(1);
    *fresh5 = '=' as i32 as libc::c_char;
    loop {
        let fresh6 = s;
        s = s.offset(1);
        c = *fresh6 as libc::c_uint;
        if !(c != 0 && p < pmax) {
            break;
        }
        match c {
            27 => {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = '\\' as i32 as libc::c_char;
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = 'E' as i32 as libc::c_char;
            }
            58 => {
                strcpy(p, b"\\072\0" as *const u8 as *const libc::c_char);
                p = p.offset(4 as libc::c_int as isize);
            }
            94 | 92 => {
                let fresh9 = p;
                p = p.offset(1);
                *fresh9 = '\\' as i32 as libc::c_char;
                let fresh10 = p;
                p = p.offset(1);
                *fresh10 = c as libc::c_char;
            }
            _ => {
                if c >= 200 as libc::c_int as libc::c_uint {
                    sprintf(
                        p,
                        b"\\%03o\0" as *const u8 as *const libc::c_char,
                        c & 0o377 as libc::c_int as libc::c_uint,
                    );
                    p = p.offset(4 as libc::c_int as isize);
                } else if c < ' ' as i32 as libc::c_uint {
                    let fresh11 = p;
                    p = p.offset(1);
                    *fresh11 = '^' as i32 as libc::c_char;
                    let fresh12 = p;
                    p = p.offset(1);
                    *fresh12 = c.wrapping_add('@' as i32 as libc::c_uint)
                        as libc::c_char;
                } else {
                    let fresh13 = p;
                    p = p.offset(1);
                    *fresh13 = c as libc::c_char;
                }
            }
        }
    }
    let fresh14 = p;
    p = p.offset(1);
    *fresh14 = ':' as i32 as libc::c_char;
    *p = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn CreateTransTable(mut s: *mut libc::c_char) -> libc::c_int {
    let mut curchar: libc::c_int = 0;
    let mut templ: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut templlen: libc::c_int = 0;
    let mut templnsub: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sx: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctable: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    (*display)
        .d_xtable = calloc(
        256 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut *mut libc::c_char;
    if ((*display).d_xtable).is_null() {
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    while *s != 0 {
        if *s as libc::c_int == '\\' as i32
            && (*s.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32
                || *s.offset(1 as libc::c_int as isize) as libc::c_int == ',' as i32
                || *s.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32)
        {
            s = s.offset(1);
            s;
        }
        let fresh15 = s;
        s = s.offset(1);
        curchar = *fresh15 as libc::c_uchar as libc::c_int;
        if curchar == 'B' as i32 {
            curchar = 0 as libc::c_int;
        }
        templ = s;
        templlen = 0 as libc::c_int;
        templnsub = 0 as libc::c_int;
        if (*((*display).d_xtable).offset(curchar as isize)).is_null() {
            let ref mut fresh16 = *((*display).d_xtable).offset(curchar as isize);
            *fresh16 = calloc(
                257 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            if (*fresh16).is_null() {
                Msg(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    strnomem.as_mut_ptr(),
                );
                FreeTransTable();
                return -(1 as libc::c_int);
            }
        }
        ctable = *((*display).d_xtable).offset(curchar as isize);
        let mut current_block_22: u64;
        while *s as libc::c_int != 0 && *s as libc::c_int != ',' as i32 {
            if *s as libc::c_int == '\\' as i32
                && (*s.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32
                    || *s.offset(1 as libc::c_int as isize) as libc::c_int == ',' as i32
                    || *s.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32)
            {
                s = s.offset(1);
                s;
                current_block_22 = 2668756484064249700;
            } else if *s as libc::c_int == '%' as i32 {
                templnsub += 1;
                templnsub;
                current_block_22 = 6009453772311597924;
            } else {
                current_block_22 = 2668756484064249700;
            }
            match current_block_22 {
                2668756484064249700 => {
                    templlen += 1;
                    templlen;
                }
                _ => {}
            }
            s = s.offset(1);
            s;
        }
        let fresh17 = s;
        s = s.offset(1);
        if *fresh17 as libc::c_int == 0 as libc::c_int {
            break;
        }
        while *s as libc::c_int != 0 && *s as libc::c_int != ',' as i32 {
            let fresh18 = s;
            s = s.offset(1);
            c = *fresh18 as libc::c_uchar as libc::c_int;
            if *s.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\\' as i32
                && (*s
                    .offset(-(1 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32
                    || *s
                        .offset(-(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == ',' as i32
                    || *s
                        .offset(-(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32)
            {
                let fresh19 = s;
                s = s.offset(1);
                c = *fresh19 as libc::c_uchar as libc::c_int;
            } else if c == '%' as i32 {
                c = 256 as libc::c_int;
            }
            if !(*ctable.offset(c as isize)).is_null() {
                free(*ctable.offset(c as isize) as *mut libc::c_void);
            }
            arg = s;
            l = copyarg(&mut s, 0 as *mut libc::c_char);
            if c != 256 as libc::c_int {
                l = l * templnsub + templlen;
            }
            let ref mut fresh20 = *ctable.offset(c as isize);
            *fresh20 = malloc((l + 1 as libc::c_int) as libc::c_ulong)
                as *mut libc::c_char;
            if (*fresh20).is_null() {
                Msg(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    strnomem.as_mut_ptr(),
                );
                FreeTransTable();
                return -(1 as libc::c_int);
            }
            sx = *ctable.offset(c as isize);
            let mut current_block_45: u64;
            p = (if c == 256 as libc::c_int {
                b"%\0" as *const u8 as *const libc::c_char
            } else {
                templ as *const libc::c_char
            }) as *mut libc::c_char;
            while *p as libc::c_int != 0 && *p as libc::c_int != ',' as i32 {
                if *p as libc::c_int == '\\' as i32
                    && (*p.offset(1 as libc::c_int as isize) as libc::c_int
                        == '\\' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == ',' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '%' as i32)
                {
                    p = p.offset(1);
                    p;
                    current_block_45 = 790185930182612747;
                } else if *p as libc::c_int == '%' as i32 {
                    s = arg;
                    sx = sx.offset(copyarg(&mut s, sx) as isize);
                    current_block_45 = 7659304154607701039;
                } else {
                    current_block_45 = 790185930182612747;
                }
                match current_block_45 {
                    790185930182612747 => {
                        let fresh21 = sx;
                        sx = sx.offset(1);
                        *fresh21 = *p;
                    }
                    _ => {}
                }
                p = p.offset(1);
                p;
            }
            *sx = 0 as libc::c_int as libc::c_char;
        }
        if *s as libc::c_int == ',' as i32 {
            s = s.offset(1);
            s;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FreeTransTable() {
    let mut p: *mut *mut *mut libc::c_char = 0 as *mut *mut *mut libc::c_char;
    let mut q: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    p = (*display).d_xtable;
    if p.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(*p).is_null() {
            q = *p;
            j = 0 as libc::c_int;
            while j < 257 as libc::c_int {
                if !(*q).is_null() {
                    free(*q as *mut libc::c_void);
                }
                j += 1;
                j;
                q = q.offset(1);
                q;
            }
            free(*p as *mut libc::c_void);
        }
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    free((*display).d_xtable as *mut libc::c_void);
    (*display).d_xtable = 0 as *mut *mut *mut libc::c_char;
}
unsafe extern "C" fn copyarg(
    mut pp: *mut *mut libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    l = 0 as libc::c_int;
    p = *pp;
    while *p as libc::c_int != 0 && *p as libc::c_int != ',' as i32 {
        if *p as libc::c_int == '\\' as i32
            && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == ',' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32)
        {
            p = p.offset(1);
            p;
        }
        if !s.is_null() {
            let fresh22 = s;
            s = s.offset(1);
            *fresh22 = *p;
        }
        l += 1;
        l;
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == ',' as i32 {
        p = p.offset(1);
        p;
    }
    *pp = p;
    return l;
}
unsafe extern "C" fn e_tgetent(
    mut bp: *mut libc::c_char,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    xseteuid(real_uid);
    xsetegid(real_gid);
    r = tgetent(bp, name);
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    return r;
}
unsafe extern "C" fn findcap(
    mut cap: *mut libc::c_char,
    mut tepp: *mut *mut libc::c_char,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut tep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut capl: libc::c_int = 0;
    if extra_incap.is_null() {
        return 0 as *mut libc::c_char;
    }
    tep = *tepp;
    capl = strlen(cap) as libc::c_int;
    cp = 0 as *mut libc::c_char;
    mode = 0 as libc::c_int;
    p = extra_incap;
    while *p != 0 {
        if strncmp(p, cap, capl as libc::c_ulong) == 0 as libc::c_int {
            p = p.offset(capl as isize);
            c = *p;
            if c as libc::c_int != 0 && c as libc::c_int != ':' as i32
                && c as libc::c_int != '@' as i32
            {
                p = p.offset(1);
                p;
            }
            if c as libc::c_int == 0 as libc::c_int || c as libc::c_int == '@' as i32
                || c as libc::c_int == '=' as i32 || c as libc::c_int == ':' as i32
                || c as libc::c_int == '#' as i32
            {
                cp = tep;
            }
        }
        loop {
            c = *p;
            if !(c != 0) {
                break;
            }
            p = p.offset(1);
            p;
            if mode == 0 as libc::c_int {
                if c as libc::c_int == ':' as i32 {
                    break;
                }
                if c as libc::c_int == '^' as i32 {
                    mode = 1 as libc::c_int;
                }
                if c as libc::c_int == '\\' as i32 {
                    mode = 2 as libc::c_int;
                }
            } else if mode == 1 as libc::c_int {
                mode = 0 as libc::c_int;
                c = (c as libc::c_int & 0x1f as libc::c_int) as libc::c_char;
            } else if mode == 2 as libc::c_int {
                mode = 0 as libc::c_int;
                match c as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        mode = 3 as libc::c_int;
                        num = 0 as libc::c_int;
                    }
                    69 => {
                        c = 27 as libc::c_int as libc::c_char;
                    }
                    110 => {
                        c = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        c = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        c = '\t' as i32 as libc::c_char;
                    }
                    98 => {
                        c = '\u{8}' as i32 as libc::c_char;
                    }
                    102 => {
                        c = '\u{c}' as i32 as libc::c_char;
                    }
                    _ => {}
                }
            }
            if mode > 2 as libc::c_int {
                num = num * 8 as libc::c_int + (c as libc::c_int - '0' as i32);
                let fresh23 = mode;
                mode = mode + 1;
                if fresh23 == 5 as libc::c_int
                    || ((*p as libc::c_int) < '0' as i32
                        || *p as libc::c_int > '9' as i32)
                {
                    c = num as libc::c_char;
                    mode = 0 as libc::c_int;
                }
            }
            if mode != 0 {
                continue;
            }
            if !cp.is_null() && n != 1 as libc::c_int {
                let fresh24 = cp;
                cp = cp.offset(1);
                *fresh24 = c;
                n -= 1;
                n;
            }
        }
        if !cp.is_null() {
            let fresh25 = cp;
            cp = cp.offset(1);
            *fresh25 = 0 as libc::c_int as libc::c_char;
            *tepp = cp;
            return tep;
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn e_tgetstr(
    mut cap: *mut libc::c_char,
    mut tepp: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut tep: *mut libc::c_char = 0 as *mut libc::c_char;
    tep = findcap(cap, tepp, 0 as libc::c_int);
    if !tep.is_null() {
        return if *tep as libc::c_int == '@' as i32 {
            0 as *mut libc::c_char
        } else {
            tep
        };
    }
    return tgetstr(cap, tepp);
}
unsafe extern "C" fn e_tgetflag(mut cap: *mut libc::c_char) -> libc::c_int {
    let mut buf: [libc::c_char; 2] = [0; 2];
    let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tep: *mut libc::c_char = 0 as *mut libc::c_char;
    bufp = buf.as_mut_ptr();
    tep = findcap(cap, &mut bufp, 2 as libc::c_int);
    if !tep.is_null() {
        return if *tep as libc::c_int == '@' as i32 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    return (tgetflag(cap) > 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn e_tgetnum(mut cap: *mut libc::c_char) -> libc::c_int {
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut res: libc::c_int = 0;
    let mut base: libc::c_int = 10 as libc::c_int;
    bufp = buf.as_mut_ptr();
    tep = findcap(cap, &mut bufp, 20 as libc::c_int);
    if !tep.is_null() {
        c = *tep;
        if c as libc::c_int == '@' as i32 {
            return -(1 as libc::c_int);
        }
        if c as libc::c_int == '0' as i32 {
            base = 8 as libc::c_int;
        }
        res = 0 as libc::c_int;
        loop {
            let fresh26 = tep;
            tep = tep.offset(1);
            c = *fresh26;
            if !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32) {
                break;
            }
            res = res * base + (c as libc::c_int - '0' as i32);
        }
        return res;
    }
    return tgetnum(cap);
}
