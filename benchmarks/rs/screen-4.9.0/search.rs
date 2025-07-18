use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn revto_line(_: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn revto(_: libc::c_int, _: libc::c_int);
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
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    static mut flayer: *mut layer;
    static mut fore: *mut win;
}
pub type __int32_t = libc::c_int;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type pid_t = __pid_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct markdata {
    pub md_window: *mut win,
    pub md_user: *mut acluser,
    pub cx: libc::c_int,
    pub cy: libc::c_int,
    pub x1: libc::c_int,
    pub y1: libc::c_int,
    pub second: libc::c_int,
    pub left_mar: libc::c_int,
    pub right_mar: libc::c_int,
    pub nonl: libc::c_int,
    pub rep_cnt: libc::c_int,
    pub append_mode: libc::c_int,
    pub write_buffer: libc::c_int,
    pub hist_offset: libc::c_int,
    pub isstr: [libc::c_char; 100],
    pub isstrl: libc::c_int,
    pub isistr: [libc::c_char; 200],
    pub isistrl: libc::c_int,
    pub isdir: libc::c_int,
    pub isstartpos: libc::c_int,
    pub isstartdir: libc::c_int,
    pub f_cmd: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub flag: libc::c_int,
    pub target: libc::c_int,
    pub direction: libc::c_int,
}
pub static mut search_ic: libc::c_int = 0;
pub unsafe extern "C" fn Search(mut dir: libc::c_int) {
    let mut markdata: *mut markdata = 0 as *mut markdata;
    if dir == 0 as libc::c_int {
        markdata = (*flayer).l_data as *mut markdata;
        if (*markdata).isdir > 0 as libc::c_int {
            searchend(0 as *mut libc::c_char, 0 as libc::c_int, 0 as *mut libc::c_char);
        } else if (*markdata).isdir < 0 as libc::c_int {
            backsearchend(
                0 as *mut libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_char,
            );
        } else {
            LMsg(
                0 as libc::c_int,
                b"No previous pattern\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        Input(
            (if dir > 0 as libc::c_int {
                b"/\0" as *const u8 as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            0 as libc::c_int,
            if dir > 0 as libc::c_int {
                Some(
                    searchend
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                )
            } else {
                Some(
                    backsearchend
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                )
            },
            0 as *mut libc::c_char,
            0 as libc::c_int,
        );
    };
}
unsafe extern "C" fn searchend(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut sx: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut markdata: *mut markdata = 0 as *mut markdata;
    let mut p: *mut win = 0 as *mut win;
    markdata = (*flayer).l_data as *mut markdata;
    p = (*markdata).md_window;
    (*markdata).isdir = 1 as libc::c_int;
    if len != 0 {
        strcpy(((*markdata).isstr).as_mut_ptr(), buf);
    }
    sx = (*markdata).cx + 1 as libc::c_int;
    ex = (*flayer).l_width - 1 as libc::c_int;
    y = (*markdata).cy;
    while y < (*p).w_histheight + (*flayer).l_height {
        x = matchword(((*markdata).isstr).as_mut_ptr(), y, sx, ex);
        if x >= 0 as libc::c_int {
            break;
        }
        y += 1;
        y;
        sx = 0 as libc::c_int;
    }
    if y >= (*p).w_histheight + (*flayer).l_height {
        LGotoPos(flayer, (*markdata).cx, (*markdata).cy - (*markdata).hist_offset);
        LMsg(
            0 as libc::c_int,
            b"Pattern not found\0" as *const u8 as *const libc::c_char,
        );
    } else {
        revto(x, y);
    };
}
unsafe extern "C" fn backsearchend(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut sx: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut x: libc::c_int = -(1 as libc::c_int);
    let mut y: libc::c_int = 0;
    let mut markdata: *mut markdata = 0 as *mut markdata;
    markdata = (*flayer).l_data as *mut markdata;
    (*markdata).isdir = -(1 as libc::c_int);
    if len != 0 {
        strcpy(((*markdata).isstr).as_mut_ptr(), buf);
    }
    ex = (*markdata).cx - 1 as libc::c_int;
    y = (*markdata).cy;
    while y >= 0 as libc::c_int {
        sx = 0 as libc::c_int;
        loop {
            sx = matchword(((*markdata).isstr).as_mut_ptr(), y, sx, ex);
            if !(sx >= 0 as libc::c_int) {
                break;
            }
            let fresh0 = sx;
            sx = sx + 1;
            x = fresh0;
        }
        if x >= 0 as libc::c_int {
            break;
        }
        y -= 1;
        y;
        ex = (*flayer).l_width - 1 as libc::c_int;
    }
    if y < 0 as libc::c_int {
        LGotoPos(flayer, (*markdata).cx, (*markdata).cy - (*markdata).hist_offset);
        LMsg(
            0 as libc::c_int,
            b"Pattern not found\0" as *const u8 as *const libc::c_char,
        );
    } else {
        revto(x, y);
    };
}
unsafe extern "C" fn matchword(
    mut pattern: *mut libc::c_char,
    mut y: libc::c_int,
    mut sx: libc::c_int,
    mut ex: libc::c_int,
) -> libc::c_int {
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cpe: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cy: libc::c_int = 0;
    fore = (*((*flayer).l_data as *mut markdata)).md_window;
    while sx <= ex {
        cy = y;
        cp = ((*(if cy < (*fore).w_histheight {
            &mut *((*fore).w_hlines)
                .offset((((*fore).w_histidx + cy) % (*fore).w_histheight) as isize)
                as *mut mline
        } else {
            &mut *((*fore).w_mlines).offset((cy - (*fore).w_histheight) as isize)
                as *mut mline
        }))
            .image)
            .offset(sx as isize);
        cpe = ((*(if cy < (*fore).w_histheight {
            &mut *((*fore).w_hlines)
                .offset((((*fore).w_histidx + cy) % (*fore).w_histheight) as isize)
                as *mut mline
        } else {
            &mut *((*fore).w_mlines).offset((cy - (*fore).w_histheight) as isize)
                as *mut mline
        }))
            .image)
            .offset((*flayer).l_width as isize);
        pp = pattern as *mut libc::c_uchar;
        loop {
            if *cp as libc::c_int != *pp as libc::c_int {
                if search_ic == 0
                    || (*cp as libc::c_int ^ *pp as libc::c_int) & 0xdf as libc::c_int
                        != 0 || (*cp as libc::c_int | 0x20 as libc::c_int) < 'a' as i32
                    || *cp as libc::c_int | 0x20 as libc::c_int > 'z' as i32
                {
                    break;
                }
            }
            cp = cp.offset(1);
            cp;
            pp = pp.offset(1);
            pp;
            if *pp as libc::c_int == 0 as libc::c_int {
                return sx;
            }
            if !(cp == cpe) {
                continue;
            }
            if *cp as libc::c_int == ' ' as i32
                || cy >= (*fore).w_histheight + (*flayer).l_height - 1 as libc::c_int
            {
                break;
            }
            cy += 1;
            cy;
            cp = (*if cy < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset((((*fore).w_histidx + cy) % (*fore).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*fore).w_mlines).offset((cy - (*fore).w_histheight) as isize)
                    as *mut mline
            })
                .image;
            cpe = ((*(if cy < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset((((*fore).w_histidx + cy) % (*fore).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*fore).w_mlines).offset((cy - (*fore).w_histheight) as isize)
                    as *mut mline
            }))
                .image)
                .offset((*flayer).l_width as isize);
        }
        sx += 1;
        sx;
    }
    return -(1 as libc::c_int);
}
static mut isprompts: [*mut libc::c_char; 4] = [
    b"I-search backward: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"failing I-search backward: \0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"I-search: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"failing I-search: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn is_bm(
    mut str: *mut libc::c_char,
    mut l: libc::c_int,
    mut p: libc::c_int,
    mut end: libc::c_int,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut tab: [libc::c_int; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut w: libc::c_int = (*flayer).l_width;
    fore = (*((*(*flayer).l_next).l_data as *mut markdata)).md_window;
    if p < 0 as libc::c_int || p + l > end {
        return -(1 as libc::c_int);
    }
    if l == 0 as libc::c_int {
        return p;
    }
    if dir < 0 as libc::c_int {
        str = str.offset((l - 1 as libc::c_int) as isize);
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        tab[i as usize] = l * dir;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < l - 1 as libc::c_int {
        q = *(str as *mut libc::c_uchar) as libc::c_int;
        tab[q as usize] = (l - 1 as libc::c_int - i) * dir;
        if search_ic != 0 && q | 0x20 as libc::c_int >= 'a' as i32
            && q | 0x20 as libc::c_int <= 'z' as i32
        {
            tab[(q ^ 0x20 as libc::c_int) as usize] = (l - 1 as libc::c_int - i) * dir;
        }
        i += 1;
        i;
        str = str.offset(dir as isize);
    }
    if dir > 0 as libc::c_int {
        p += l - 1 as libc::c_int;
    }
    while p >= 0 as libc::c_int && p < end {
        q = p;
        s = str as *mut libc::c_uchar;
        i = 0 as libc::c_int;
        loop {
            c = *((*if q / w < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset(
                        (((*fore).w_histidx + q / w) % (*fore).w_histheight) as isize,
                    ) as *mut mline
            } else {
                &mut *((*fore).w_mlines).offset((q / w - (*fore).w_histheight) as isize)
                    as *mut mline
            })
                .image)
                .offset((q % w) as isize);
            if i == 0 as libc::c_int {
                p += tab[c as libc::c_int as usize];
            }
            if c as libc::c_int != *s as libc::c_int {
                if search_ic == 0
                    || (c as libc::c_int ^ *s as libc::c_int) & 0xdf as libc::c_int != 0
                    || (c as libc::c_int | 0x20 as libc::c_int) < 'a' as i32
                    || c as libc::c_int | 0x20 as libc::c_int > 'z' as i32
                {
                    break;
                }
            }
            q -= dir;
            s = s.offset(-(dir as isize));
            i += 1;
            if i == l {
                return q + (if dir > 0 as libc::c_int { 1 as libc::c_int } else { -l });
            }
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn is_process(
    mut p: *mut libc::c_char,
    mut n: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut pos: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut markdata: *mut markdata = 0 as *mut markdata;
    if n == 0 as libc::c_int {
        return;
    }
    markdata = (*(*flayer).l_next).l_data as *mut markdata;
    pos = (*markdata).cx + (*markdata).cy * (*flayer).l_width;
    LGotoPos(flayer, (*markdata).cx, (*markdata).cy - (*markdata).hist_offset);
    let mut current_block_29: u64;
    match *p as libc::c_int {
        7 => {
            pos = (*markdata).isstartpos;
            current_block_29 = 5327833733962873709;
        }
        27 => {
            current_block_29 = 5327833733962873709;
        }
        11 | 23 => {
            (*markdata).isistrl = 1 as libc::c_int;
            current_block_29 = 10662815440448604992;
        }
        8 | 127 => {
            current_block_29 = 10662815440448604992;
        }
        19 | 18 => {
            if (*markdata).isistrl
                >= ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong
                    as libc::c_int
            {
                return;
            }
            dir = if *p as libc::c_int == '\u{13}' as i32 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
            pos += dir;
            if (*markdata).isdir == dir && (*markdata).isistrl == 0 as libc::c_int {
                strcpy(
                    ((*markdata).isistr).as_mut_ptr(),
                    ((*markdata).isstr).as_mut_ptr(),
                );
                (*markdata)
                    .isstrl = strlen(((*markdata).isstr).as_mut_ptr()) as libc::c_int;
                (*markdata).isistrl = (*markdata).isstrl;
            } else {
                (*markdata).isdir = dir;
                let fresh1 = (*markdata).isistrl;
                (*markdata).isistrl = (*markdata).isistrl + 1;
                (*markdata).isistr[fresh1 as usize] = *p;
            }
            current_block_29 = 3437258052017859086;
        }
        _ => {
            if (*p as libc::c_int) < ' ' as i32
                || (*markdata).isistrl
                    >= ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong
                        as libc::c_int
                || (*markdata).isstrl
                    >= ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                        as libc::c_int - 1 as libc::c_int
            {
                return;
            }
            let fresh2 = (*markdata).isstrl;
            (*markdata).isstrl = (*markdata).isstrl + 1;
            (*markdata).isstr[fresh2 as usize] = *p;
            let fresh3 = (*markdata).isistrl;
            (*markdata).isistrl = (*markdata).isistrl + 1;
            (*markdata).isistr[fresh3 as usize] = *p;
            (*markdata)
                .isstr[(*markdata).isstrl as usize] = 0 as libc::c_int as libc::c_char;
            current_block_29 = 3437258052017859086;
        }
    }
    match current_block_29 {
        10662815440448604992 => {
            if (*markdata).isistrl == 0 as libc::c_int {
                return;
            }
            (*markdata).isistrl -= 1;
            (*markdata).isistrl;
            pos = is_redo(markdata);
            *p = '\u{8}' as i32 as libc::c_char;
        }
        5327833733962873709 => {
            *p = 0 as libc::c_int as libc::c_char;
        }
        _ => {}
    }
    if *p as libc::c_int != 0 && *p as libc::c_int != '\u{8}' as i32 {
        pos = is_bm(
            ((*markdata).isstr).as_mut_ptr(),
            (*markdata).isstrl,
            pos,
            (*flayer).l_width
                * ((*(*markdata).md_window).w_histheight + (*flayer).l_height),
            (*markdata).isdir,
        );
    }
    if pos >= 0 as libc::c_int {
        x = pos % (*flayer).l_width;
        y = pos / (*flayer).l_width;
        let mut oldlay: *mut layer = flayer;
        let mut oldcvlist: *mut canvas = 0 as *mut canvas;
        let mut cv: *mut canvas = 0 as *mut canvas;
        flayer = (*flayer).l_next;
        oldcvlist = (*flayer).l_cvlist;
        (*flayer).l_cvlist = (*oldlay).l_cvlist;
        cv = (*flayer).l_cvlist;
        while !cv.is_null() {
            (*cv).c_layer = flayer;
            cv = (*cv).c_lnext;
        }
        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
            .unwrap()(
            (*flayer).l_height - 1 as libc::c_int,
            0 as libc::c_int,
            (*flayer).l_width - 1 as libc::c_int,
            0 as libc::c_int,
        );
        revto(x, y);
        if (*markdata).cy - (*markdata).hist_offset
            == (*flayer).l_height - 1 as libc::c_int
        {
            revto_line(
                (*markdata).cx,
                (*markdata).cy,
                if (*flayer).l_height - 1 as libc::c_int > 0 as libc::c_int {
                    (*flayer).l_height - 1 as libc::c_int - 1 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            );
        }
        flayer = oldlay;
        cv = (*flayer).l_cvlist;
        while !cv.is_null() {
            (*cv).c_layer = flayer;
            cv = (*cv).c_lnext;
        }
        (*(*flayer).l_next).l_cvlist = oldcvlist;
    }
    if *p != 0 {
        inp_setprompt(
            isprompts[((*markdata).isdir + (pos < 0 as libc::c_int) as libc::c_int
                + 1 as libc::c_int) as usize],
            (if (*markdata).isstrl != 0 {
                ((*markdata).isstr).as_mut_ptr() as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        );
    }
    (*flayer).l_x = (*markdata).cx;
    (*flayer).l_y = (*markdata).cy - (*markdata).hist_offset;
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
    if *p == 0 {
        (*(*flayer).l_next).l_x = (*markdata).cx;
        (*(*flayer).l_next).l_y = (*markdata).cy - (*markdata).hist_offset;
    }
}
unsafe extern "C" fn is_redo(mut markdata: *mut markdata) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut npos: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    pos = (*markdata).isstartpos;
    npos = pos;
    dir = (*markdata).isstartdir;
    (*markdata).isstrl = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*markdata).isistrl {
        c = (*markdata).isistr[i as usize];
        if c as libc::c_int == '\u{12}' as i32 {
            dir = -(1 as libc::c_int);
            pos += dir;
        } else if c as libc::c_int == '\u{13}' as i32 {
            dir = 1 as libc::c_int;
            pos += dir;
        } else {
            let fresh4 = (*markdata).isstrl;
            (*markdata).isstrl = (*markdata).isstrl + 1;
            (*markdata).isstr[fresh4 as usize] = c;
        }
        if pos >= 0 as libc::c_int {
            npos = is_bm(
                ((*markdata).isstr).as_mut_ptr(),
                (*markdata).isstrl,
                pos,
                (*flayer).l_width
                    * ((*(*markdata).md_window).w_histheight + (*flayer).l_height),
                dir,
            );
            if npos >= 0 as libc::c_int {
                pos = npos;
            }
        }
        i += 1;
        i;
    }
    (*markdata).isstr[(*markdata).isstrl as usize] = 0 as libc::c_int as libc::c_char;
    (*markdata).isdir = dir;
    return npos;
}
pub unsafe extern "C" fn ISearch(mut dir: libc::c_int) {
    let mut markdata: *mut markdata = 0 as *mut markdata;
    markdata = (*flayer).l_data as *mut markdata;
    (*markdata).isstartdir = dir;
    (*markdata).isdir = (*markdata).isstartdir;
    (*markdata).isstartpos = (*markdata).cx + (*markdata).cy * (*flayer).l_width;
    (*markdata).isstrl = 0 as libc::c_int;
    (*markdata).isistrl = (*markdata).isstrl;
    if (*markdata).cy - (*markdata).hist_offset == (*flayer).l_height - 1 as libc::c_int
    {
        revto_line(
            (*markdata).cx,
            (*markdata).cy,
            if (*flayer).l_height - 1 as libc::c_int > 0 as libc::c_int {
                (*flayer).l_height - 1 as libc::c_int - 1 as libc::c_int
            } else {
                1 as libc::c_int
            },
        );
    }
    Input(
        isprompts[(dir + 1 as libc::c_int) as usize],
        (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        2 as libc::c_int,
        Some(
            is_process
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut libc::c_char,
                ) -> (),
        ),
        0 as *mut libc::c_char,
        0 as libc::c_int,
    );
    LGotoPos(flayer, (*markdata).cx, (*markdata).cy - (*markdata).hist_offset);
    (*flayer).l_x = (*markdata).cx;
    (*flayer).l_y = (*markdata).cy - (*markdata).hist_offset;
}
