use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    static mut fLog_sighup: boolean;
    fn iconn_baud(qconn: *mut sconnection) -> libc::c_long;
    fn fgot_data(
        qdaemon: *mut sdaemon,
        zfirst: *const libc::c_char,
        cfirst: size_t,
        zsecond: *const libc::c_char,
        csecond: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
        fallacked: boolean,
        pfexit: *mut boolean,
    ) -> boolean;
    fn uwindow_acked(qdaemon: *mut sdaemon, fallacked: boolean);
    fn fsend_data(
        qconn: *mut sconnection,
        zsend: *const libc::c_char,
        csend: size_t,
        fdoread: boolean,
    ) -> boolean;
    fn freceive_data(
        qconn: *mut sconnection,
        cneed: size_t,
        pcrec: *mut size_t,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> boolean;
    fn icrc(z: *const libc::c_char, c: size_t, ick: libc::c_ulong) -> libc::c_ulong;
    static mut abPrecbuf: [libc::c_char; 16384];
    static mut iPrecstart: libc::c_int;
    static mut iPrecend: libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type openfile_t = *mut FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_system {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_pzalias: *mut *mut libc::c_char,
    pub uuconf_qalternate: *mut uuconf_system,
    pub uuconf_zalternate: *mut libc::c_char,
    pub uuconf_fcall: libc::c_int,
    pub uuconf_fcalled: libc::c_int,
    pub uuconf_qtimegrade: *mut uuconf_timespan,
    pub uuconf_qcalltimegrade: *mut uuconf_timespan,
    pub uuconf_qcalledtimegrade: *mut uuconf_timespan,
    pub uuconf_cmax_retries: libc::c_int,
    pub uuconf_csuccess_wait: libc::c_int,
    pub uuconf_qcall_local_size: *mut uuconf_timespan,
    pub uuconf_qcall_remote_size: *mut uuconf_timespan,
    pub uuconf_qcalled_local_size: *mut uuconf_timespan,
    pub uuconf_qcalled_remote_size: *mut uuconf_timespan,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_ihighbaud: libc::c_long,
    pub uuconf_zport: *mut libc::c_char,
    pub uuconf_qport: *mut uuconf_port,
    pub uuconf_zphone: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zcall_login: *mut libc::c_char,
    pub uuconf_zcall_password: *mut libc::c_char,
    pub uuconf_zcalled_login: *mut libc::c_char,
    pub uuconf_fcallback: libc::c_int,
    pub uuconf_fsequence: libc::c_int,
    pub uuconf_zprotocols: *mut libc::c_char,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_scalled_chat: uuconf_chat,
    pub uuconf_zdebug: *mut libc::c_char,
    pub uuconf_zmax_remote_debug: *mut libc::c_char,
    pub uuconf_fsend_request: libc::c_int,
    pub uuconf_frec_request: libc::c_int,
    pub uuconf_fcall_transfer: libc::c_int,
    pub uuconf_fcalled_transfer: libc::c_int,
    pub uuconf_pzlocal_send: *mut *mut libc::c_char,
    pub uuconf_pzremote_send: *mut *mut libc::c_char,
    pub uuconf_pzlocal_receive: *mut *mut libc::c_char,
    pub uuconf_pzremote_receive: *mut *mut libc::c_char,
    pub uuconf_pzpath: *mut *mut libc::c_char,
    pub uuconf_pzcmds: *mut *mut libc::c_char,
    pub uuconf_cfree_space: libc::c_long,
    pub uuconf_pzforward_from: *mut *mut libc::c_char,
    pub uuconf_pzforward_to: *mut *mut libc::c_char,
    pub uuconf_zpubdir: *const libc::c_char,
    pub uuconf_zlocalname: *mut libc::c_char,
    pub uuconf_cmax_file_time: libc::c_long,
    pub uuconf_palloc: UUCONF_POINTER,
}
pub type UUCONF_POINTER = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_proto_param {
    pub uuconf_bproto: libc::c_int,
    pub uuconf_qentries: *mut uuconf_proto_param_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_proto_param_entry {
    pub uuconf_cargs: libc::c_int,
    pub uuconf_pzargs: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_port {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_ttype: uuconf_porttype,
    pub uuconf_zprotocols: *mut libc::c_char,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_zlockname: *mut libc::c_char,
    pub uuconf_palloc: UUCONF_POINTER,
    pub uuconf_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uuconf_sstdin: uuconf_stdin_port,
    pub uuconf_smodem: uuconf_modem_port,
    pub uuconf_sdirect: uuconf_direct_port,
    pub uuconf_stcp: uuconf_tcp_port,
    pub uuconf_stli: uuconf_tli_port,
    pub uuconf_spipe: uuconf_pipe_port,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_pipe_port {
    pub uuconf_pzcmd: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_tli_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_fstream: libc::c_int,
    pub uuconf_pzpush: *mut *mut libc::c_char,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
    pub uuconf_zservaddr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_tcp_port {
    pub uuconf_zport: *mut libc::c_char,
    pub uuconf_iversion: libc::c_int,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_direct_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_fhardflow: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_modem_port {
    pub uuconf_zdevice: *mut libc::c_char,
    pub uuconf_zdial_device: *mut libc::c_char,
    pub uuconf_ibaud: libc::c_long,
    pub uuconf_ilowbaud: libc::c_long,
    pub uuconf_ihighbaud: libc::c_long,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_fhardflow: libc::c_int,
    pub uuconf_pzdialer: *mut *mut libc::c_char,
    pub uuconf_qdialer: *mut uuconf_dialer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_dialer {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zdialtone: *mut libc::c_char,
    pub uuconf_zpause: *mut libc::c_char,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_ccarrier_wait: libc::c_int,
    pub uuconf_fdtr_toggle: libc::c_int,
    pub uuconf_fdtr_toggle_wait: libc::c_int,
    pub uuconf_scomplete: uuconf_chat,
    pub uuconf_sabort: uuconf_chat,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_palloc: UUCONF_POINTER,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_stdin_port {
    pub uuconf_idummy: libc::c_int,
}
pub type uuconf_porttype = libc::c_uint;
pub const UUCONF_PORTTYPE_PIPE: uuconf_porttype = 6;
pub const UUCONF_PORTTYPE_TLI: uuconf_porttype = 5;
pub const UUCONF_PORTTYPE_TCP: uuconf_porttype = 4;
pub const UUCONF_PORTTYPE_DIRECT: uuconf_porttype = 3;
pub const UUCONF_PORTTYPE_MODEM: uuconf_porttype = 2;
pub const UUCONF_PORTTYPE_STDIN: uuconf_porttype = 1;
pub const UUCONF_PORTTYPE_UNKNOWN: uuconf_porttype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
}
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scmd {
    pub bcmd: libc::c_char,
    pub bgrade: libc::c_char,
    pub pseq: pointer,
    pub zfrom: *const libc::c_char,
    pub zto: *const libc::c_char,
    pub zuser: *const libc::c_char,
    pub zoptions: *const libc::c_char,
    pub ztemp: *const libc::c_char,
    pub imode: libc::c_uint,
    pub znotify: *const libc::c_char,
    pub cbytes: libc::c_long,
    pub zcmd: *const libc::c_char,
    pub ipos: libc::c_long,
}
pub type uuconf_cmdtabfn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::c_int,
        *mut *mut libc::c_char,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_cmdtab {
    pub uuconf_zcmd: *const libc::c_char,
    pub uuconf_itype: libc::c_int,
    pub uuconf_pvar: UUCONF_POINTER,
    pub uuconf_pifn: uuconf_cmdtabfn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sconnection {
    pub qcmds: *const sconncmds,
    pub psysdep: pointer,
    pub qport: *mut uuconf_port,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sconncmds {
    pub pufree: Option::<unsafe extern "C" fn(*mut sconnection) -> ()>,
    pub pflock: Option::<
        unsafe extern "C" fn(*mut sconnection, boolean, boolean) -> boolean,
    >,
    pub pfunlock: Option::<unsafe extern "C" fn(*mut sconnection) -> boolean>,
    pub pfopen: Option::<
        unsafe extern "C" fn(*mut sconnection, libc::c_long, boolean, boolean) -> boolean,
    >,
    pub pfclose: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            pointer,
            *mut uuconf_dialer,
            boolean,
        ) -> boolean,
    >,
    pub pfdial: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            pointer,
            *const uuconf_system,
            *const libc::c_char,
            *mut uuconf_dialer,
            *mut tdialerfound,
        ) -> boolean,
    >,
    pub pfread: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            *mut libc::c_char,
            *mut size_t,
            size_t,
            libc::c_int,
            boolean,
        ) -> boolean,
    >,
    pub pfwrite: Option::<
        unsafe extern "C" fn(*mut sconnection, *const libc::c_char, size_t) -> boolean,
    >,
    pub pfio: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            *const libc::c_char,
            *mut size_t,
            *mut libc::c_char,
            *mut size_t,
        ) -> boolean,
    >,
    pub pfbreak: Option::<unsafe extern "C" fn(*mut sconnection) -> boolean>,
    pub pfset: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            tparitysetting,
            tstripsetting,
            txonxoffsetting,
        ) -> boolean,
    >,
    pub pfcarrier: Option::<unsafe extern "C" fn(*mut sconnection, boolean) -> boolean>,
    pub pfchat: Option::<
        unsafe extern "C" fn(*mut sconnection, *mut *mut libc::c_char) -> boolean,
    >,
    pub pibaud: Option::<unsafe extern "C" fn(*mut sconnection) -> libc::c_long>,
}
pub type txonxoffsetting = libc::c_uint;
pub const XONXOFF_ON: txonxoffsetting = 2;
pub const XONXOFF_OFF: txonxoffsetting = 1;
pub const XONXOFF_DEFAULT: txonxoffsetting = 0;
pub type tstripsetting = libc::c_uint;
pub const STRIPSETTING_SEVENBITS: tstripsetting = 2;
pub const STRIPSETTING_EIGHTBITS: tstripsetting = 1;
pub const STRIPSETTING_DEFAULT: tstripsetting = 0;
pub type tparitysetting = libc::c_uint;
pub const PARITYSETTING_SPACE: tparitysetting = 5;
pub const PARITYSETTING_MARK: tparitysetting = 4;
pub const PARITYSETTING_ODD: tparitysetting = 3;
pub const PARITYSETTING_EVEN: tparitysetting = 2;
pub const PARITYSETTING_NONE: tparitysetting = 1;
pub const PARITYSETTING_DEFAULT: tparitysetting = 0;
pub type tdialerfound = libc::c_uint;
pub const DIALERFOUND_FREE: tdialerfound = 2;
pub const DIALERFOUND_TRUE: tdialerfound = 1;
pub const DIALERFOUND_FALSE: tdialerfound = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdaemon {
    pub puuconf: pointer,
    pub zconfig: *const libc::c_char,
    pub irunuuxqt: libc::c_int,
    pub qsys: *const uuconf_system,
    pub zlocalname: *const libc::c_char,
    pub qconn: *mut sconnection,
    pub qproto: *const sprotocol,
    pub cchans: libc::c_int,
    pub clocal_size: libc::c_long,
    pub cremote_size: libc::c_long,
    pub cmax_ever: libc::c_long,
    pub cmax_receive: libc::c_long,
    pub csent: libc::c_long,
    pub creceived: libc::c_long,
    pub cxfiles_received: libc::c_long,
    pub ifeatures: libc::c_int,
    pub frequest_hangup: boolean,
    pub fhangup_requested: boolean,
    pub fhangup: boolean,
    pub fmaster: boolean,
    pub fcaller: boolean,
    pub ireliable: libc::c_int,
    pub bgrade: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sprotocol {
    pub bname: libc::c_char,
    pub ireliable: libc::c_int,
    pub cchans: libc::c_int,
    pub frestart: boolean,
    pub qcmds: *mut uuconf_cmdtab,
    pub pfstart: Option::<
        unsafe extern "C" fn(*mut sdaemon, *mut *mut libc::c_char) -> boolean,
    >,
    pub pfshutdown: Option::<unsafe extern "C" fn(*mut sdaemon) -> boolean>,
    pub pfsendcmd: Option::<
        unsafe extern "C" fn(
            *mut sdaemon,
            *const libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> boolean,
    >,
    pub pzgetspace: Option::<
        unsafe extern "C" fn(*mut sdaemon, *mut size_t) -> *mut libc::c_char,
    >,
    pub pfsenddata: Option::<
        unsafe extern "C" fn(
            *mut sdaemon,
            *mut libc::c_char,
            size_t,
            libc::c_int,
            libc::c_int,
            libc::c_long,
        ) -> boolean,
    >,
    pub pfwait: Option::<unsafe extern "C" fn(*mut sdaemon) -> boolean>,
    pub pffile: Option::<
        unsafe extern "C" fn(
            *mut sdaemon,
            *mut stransfer,
            boolean,
            boolean,
            libc::c_long,
            *mut boolean,
        ) -> boolean,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stransfer {
    pub qnext: *mut stransfer,
    pub qprev: *mut stransfer,
    pub pqqueue: *mut *mut stransfer,
    pub psendfn: Option::<unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean>,
    pub precfn: Option::<
        unsafe extern "C" fn(
            *mut stransfer,
            *mut sdaemon,
            *const libc::c_char,
            size_t,
        ) -> boolean,
    >,
    pub pinfo: pointer,
    pub fsendfile: boolean,
    pub frecfile: boolean,
    pub e: openfile_t,
    pub ipos: libc::c_long,
    pub fcmd: boolean,
    pub zcmd: *mut libc::c_char,
    pub ccmd: size_t,
    pub ilocal: libc::c_int,
    pub iremote: libc::c_int,
    pub s: scmd,
    pub zlog: *mut libc::c_char,
    pub isecs: libc::c_long,
    pub imicros: libc::c_long,
    pub cbytes: libc::c_long,
}
pub static mut proti_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: proti.c,v 1.36 2002/03/05 19:10:41 ian Rel $\0")
};
static mut iIrequest_packsize: libc::c_int = 1024 as libc::c_int;
static mut iIrequest_winsize: libc::c_int = 16 as libc::c_int;
static mut iIremote_packsize: libc::c_int = 0;
static mut iIalc_packsize: libc::c_int = 0;
static mut iIforced_remote_packsize: libc::c_int = 0 as libc::c_int;
static mut iIremote_winsize: libc::c_int = 0;
pub static mut cIsync_timeout: libc::c_int = 10 as libc::c_int;
static mut cIsync_retries: libc::c_int = 6 as libc::c_int;
static mut cItimeout: libc::c_int = 10 as libc::c_int;
static mut cIwindow_timeout: libc::c_int = 10 as libc::c_int;
static mut cIretries: libc::c_int = 6 as libc::c_int;
static mut cIerrors: libc::c_int = 100 as libc::c_int;
static mut cIerror_decay: libc::c_int = 10 as libc::c_int;
static mut cIack_frequency: libc::c_int = 0 as libc::c_int;
pub static mut zJavoid_parameter: *const libc::c_char = b"\\021\\023\0" as *const u8
    as *const libc::c_char;
static mut pfIsend: Option::<
    unsafe extern "C" fn(
        *mut sconnection,
        *const libc::c_char,
        size_t,
        boolean,
    ) -> boolean,
> = None;
static mut pfIreceive: Option::<
    unsafe extern "C" fn(
        *mut sconnection,
        size_t,
        *mut size_t,
        libc::c_int,
        boolean,
    ) -> boolean,
> = None;
static mut iIsendseq: libc::c_int = 0;
static mut iIrecseq: libc::c_int = 0;
static mut iIlocal_ack: libc::c_int = 0;
static mut iIremote_ack: libc::c_int = 0;
static mut iIsendpos: libc::c_long = 0;
static mut iIrecpos: libc::c_long = 0;
static mut fIclosing: boolean = 0;
static mut azIsendbuffers: [*mut libc::c_char; 32] = [0 as *const libc::c_char
    as *mut libc::c_char; 32];
static mut azIrecbuffers: [*mut libc::c_char; 32] = [0 as *const libc::c_char
    as *mut libc::c_char; 32];
static mut afInaked: [boolean; 32] = [0; 32];
static mut cIsyncs: libc::c_int = 0;
static mut cIsent_packets: libc::c_long = 0;
static mut cIreceived_packets: libc::c_long = 0;
static mut cIresent_packets: libc::c_long = 0;
static mut cIbad_hdr: libc::c_long = 0;
static mut cIbad_order: libc::c_long = 0;
static mut cIbad_cksum: libc::c_long = 0;
static mut cIremote_rejects: libc::c_long = 0;
pub static mut asIproto_params: [uuconf_cmdtab; 12] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"packet-size\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iIrequest_packsize as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"window\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iIrequest_winsize as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"remote-packet-size\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iIforced_remote_packsize as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"sync-timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cIsync_timeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"sync-retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cIsync_retries as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cItimeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cIretries as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"errors\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cIerrors as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"error-decay\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cIerror_decay as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"ack-frequency\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cIack_frequency as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"avoid\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zJavoid_parameter as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: 0 as *const libc::c_char,
                uuconf_itype: 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn fistart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    return fijstart(
        qdaemon,
        pzlog,
        ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int,
        Some(
            fsend_data
                as unsafe extern "C" fn(
                    *mut sconnection,
                    *const libc::c_char,
                    size_t,
                    boolean,
                ) -> boolean,
        ),
        Some(
            freceive_data
                as unsafe extern "C" fn(
                    *mut sconnection,
                    size_t,
                    *mut size_t,
                    libc::c_int,
                    boolean,
                ) -> boolean,
        ),
    );
}
pub unsafe extern "C" fn fijstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
    mut imaxpacksize: libc::c_int,
    mut pfsend: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            *const libc::c_char,
            size_t,
            boolean,
        ) -> boolean,
    >,
    mut pfreceive: Option::<
        unsafe extern "C" fn(
            *mut sconnection,
            size_t,
            *mut size_t,
            libc::c_int,
            boolean,
        ) -> boolean,
    >,
) -> boolean {
    let mut ab: [libc::c_char; 14] = [0; 14];
    let mut icksum: libc::c_ulong = 0;
    let mut ctries: libc::c_int = 0;
    let mut csyncs: libc::c_int = 0;
    let mut ibaud: libc::c_long = 0;
    *pzlog = 0 as *mut libc::c_char;
    pfIsend = pfsend;
    pfIreceive = pfreceive;
    if iIforced_remote_packsize <= 0 as libc::c_int
        || iIforced_remote_packsize > imaxpacksize
    {
        iIforced_remote_packsize = 0 as libc::c_int;
    } else {
        iIremote_packsize = iIforced_remote_packsize;
    }
    iIalc_packsize = 0 as libc::c_int;
    iIsendseq = 1 as libc::c_int;
    iIrecseq = 0 as libc::c_int;
    iIlocal_ack = 0 as libc::c_int;
    iIremote_ack = 0 as libc::c_int;
    iIsendpos = 0 as libc::c_int as libc::c_long;
    iIrecpos = 0 as libc::c_int as libc::c_long;
    fIclosing = 0 as libc::c_int;
    cIsent_packets = 0 as libc::c_int as libc::c_long;
    cIreceived_packets = 0 as libc::c_int as libc::c_long;
    cIresent_packets = 0 as libc::c_int as libc::c_long;
    cIbad_hdr = 0 as libc::c_int as libc::c_long;
    cIbad_order = 0 as libc::c_int as libc::c_long;
    cIbad_cksum = 0 as libc::c_int as libc::c_long;
    cIremote_rejects = 0 as libc::c_int as libc::c_long;
    if iIrequest_packsize < 0 as libc::c_int || iIrequest_packsize > imaxpacksize {
        ulog(
            LOG_ERROR,
            b"Illegal protocol '%c' packet size; using %d\0" as *const u8
                as *const libc::c_char,
            (*(*qdaemon).qproto).bname as libc::c_int,
            imaxpacksize,
        );
        iIrequest_packsize = imaxpacksize;
    }
    if iIrequest_winsize < 0 as libc::c_int
        || iIrequest_winsize > 32 as libc::c_int / 2 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"Illegal protocol '%c' window size; using %d\0" as *const u8
                as *const libc::c_char,
            (*(*qdaemon).qproto).bname as libc::c_int,
            16 as libc::c_int,
        );
        iIrequest_winsize = 16 as libc::c_int;
    }
    if cIack_frequency <= 0 as libc::c_int || cIack_frequency >= iIrequest_winsize {
        cIack_frequency = iIrequest_winsize / 2 as libc::c_int;
    }
    ab[0 as libc::c_int as usize] = '\u{7}' as i32 as libc::c_char;
    ab[2 as libc::c_int
        as usize] = ((0 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int)
        as libc::c_char;
    ab[1 as libc::c_int as usize] = ab[2 as libc::c_int as usize];
    ab[3 as libc::c_int
        as usize] = ((1 as libc::c_int) << 5 as libc::c_int
        | (if (*qdaemon).fcaller != 0 {
            (1 as libc::c_int) << 4 as libc::c_int
        } else {
            0 as libc::c_int
        }) | 4 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int) as libc::c_char;
    ab[4 as libc::c_int
        as usize] = (4 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    ab[5 as libc::c_int
        as usize] = ((ab[1 as libc::c_int as usize] as libc::c_int
        ^ ab[2 as libc::c_int as usize] as libc::c_int
        ^ ab[3 as libc::c_int as usize] as libc::c_int
        ^ ab[4 as libc::c_int as usize] as libc::c_int) & 0xff as libc::c_int)
        as libc::c_char;
    ab[(6 as libc::c_int + 0 as libc::c_int)
        as usize] = (iIrequest_packsize >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_char;
    ab[(6 as libc::c_int + 1 as libc::c_int)
        as usize] = (iIrequest_packsize & 0xff as libc::c_int) as libc::c_char;
    ab[(6 as libc::c_int + 2 as libc::c_int)
        as usize] = iIrequest_winsize as libc::c_char;
    ab[(6 as libc::c_int + 3 as libc::c_int)
        as usize] = (*qdaemon).cchans as libc::c_char;
    icksum = icrc(
        ab.as_mut_ptr().offset(6 as libc::c_int as isize),
        4 as libc::c_int as size_t,
        0xffffffff as libc::c_ulong,
    );
    *ab
        .as_mut_ptr()
        .offset(6 as libc::c_int as isize)
        .offset(4 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (icksum >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    *ab
        .as_mut_ptr()
        .offset(6 as libc::c_int as isize)
        .offset(4 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (icksum >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    *ab
        .as_mut_ptr()
        .offset(6 as libc::c_int as isize)
        .offset(4 as libc::c_int as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (icksum >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    *ab
        .as_mut_ptr()
        .offset(6 as libc::c_int as isize)
        .offset(4 as libc::c_int as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (icksum & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    csyncs = cIsyncs;
    ctries = 0 as libc::c_int;
    loop {
        let mut ftimedout: boolean = 0;
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fistart: Sending SYNC packsize %d winsize %d channels %d\0"
                    as *const u8 as *const libc::c_char,
                iIrequest_packsize,
                iIrequest_winsize,
                (*qdaemon).cchans,
            );
        }
        if (Some(pfIsend.unwrap()))
            .unwrap()(
            (*qdaemon).qconn,
            ab.as_mut_ptr(),
            (6 as libc::c_int + 4 as libc::c_int + 4 as libc::c_int) as size_t,
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if fiwait_for_packet(
            qdaemon,
            cIsync_timeout,
            0 as libc::c_int,
            0 as libc::c_int,
            &mut ftimedout,
        ) != 0
        {
            if csyncs != cIsyncs {
                break;
            }
        } else {
            if ftimedout == 0 {
                return 0 as libc::c_int;
            }
            ctries += 1;
            ctries;
            if ctries > cIsync_retries {
                ulog(
                    LOG_ERROR,
                    b"Protocol startup failed\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
        }
    }
    ibaud = iconn_baud((*qdaemon).qconn);
    if ibaud == 0 as libc::c_int as libc::c_long {
        cIwindow_timeout = cItimeout;
    } else {
        cIwindow_timeout = ((5 as libc::c_int * iIremote_packsize * iIremote_winsize)
            as libc::c_long / ibaud + cItimeout as libc::c_long) as libc::c_int;
    }
    if (*qdaemon).fcaller == 0 {
        cItimeout += 1;
        cItimeout;
        cIwindow_timeout += 1;
        cIwindow_timeout;
    }
    if iIremote_packsize > imaxpacksize {
        iIremote_packsize = imaxpacksize;
    }
    loop {
        let mut iseq: libc::c_int = 0;
        iseq = 0 as libc::c_int;
        while iseq < 32 as libc::c_int {
            azIrecbuffers[iseq as usize] = 0 as *mut libc::c_char;
            afInaked[iseq as usize] = 0 as libc::c_int;
            azIsendbuffers[iseq
                as usize] = malloc(
                (iIremote_packsize as libc::c_ulong)
                    .wrapping_add(
                        (6 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_sub(
                                        (6 as libc::c_int as libc::c_ulong)
                                            .wrapping_rem(
                                                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                            ),
                                    ),
                            ),
                    )
                    .wrapping_add(4 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if (azIsendbuffers[iseq as usize]).is_null() {
                let mut ifree: libc::c_int = 0;
                ifree = 0 as libc::c_int;
                while ifree < iseq {
                    free(azIsendbuffers[ifree as usize] as pointer);
                    ifree += 1;
                    ifree;
                }
                break;
            } else {
                iseq += 1;
                iseq;
            }
        }
        if iseq >= 32 as libc::c_int {
            *pzlog = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong)
                    .wrapping_add(64 as libc::c_int as libc::c_ulong),
            );
            sprintf(
                *pzlog,
                b"protocol '%c' sending packet/window %d/%d receiving %d/%d\0"
                    as *const u8 as *const libc::c_char,
                (*(*qdaemon).qproto).bname as libc::c_int,
                iIremote_packsize,
                iIremote_winsize,
                iIrequest_packsize,
                iIrequest_winsize,
            );
            iIalc_packsize = iIremote_packsize;
            return 1 as libc::c_int;
        }
        iIremote_packsize >>= 1 as libc::c_int;
        if !(iIremote_packsize > 200 as libc::c_int) {
            break;
        }
    }
    ulog(
        LOG_ERROR,
        b"'%c' protocol startup failed; insufficient memory for packets\0" as *const u8
            as *const libc::c_char,
        (*(*qdaemon).qproto).bname as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fishutdown(mut qdaemon: *mut sdaemon) -> boolean {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clen: size_t = 0;
    fIclosing = 1 as libc::c_int;
    z = (zigetspace(qdaemon, &mut clen)).offset(-(6 as libc::c_int as isize));
    *z.offset(0 as libc::c_int as isize) = '\u{7}' as i32 as libc::c_char;
    *z
        .offset(
            1 as libc::c_int as isize,
        ) = (iIsendseq << 3 as libc::c_int | 0 as libc::c_int) as libc::c_char;
    *z
        .offset(
            2 as libc::c_int as isize,
        ) = (iIrecseq << 3 as libc::c_int | 0 as libc::c_int) as libc::c_char;
    iIlocal_ack = iIrecseq;
    *z
        .offset(
            3 as libc::c_int as isize,
        ) = ((5 as libc::c_int) << 5 as libc::c_int
        | (if (*qdaemon).fcaller != 0 {
            (1 as libc::c_int) << 4 as libc::c_int
        } else {
            0 as libc::c_int
        }) | 0 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int) as libc::c_char;
    *z
        .offset(
            4 as libc::c_int as isize,
        ) = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *z
        .offset(
            5 as libc::c_int as isize,
        ) = ((*z.offset(1 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(2 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(3 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(4 as libc::c_int as isize) as libc::c_int) & 0xff as libc::c_int)
        as libc::c_char;
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fishutdown: Sending CLOSE\0" as *const u8 as *const libc::c_char,
        );
    }
    if (Some(pfIsend.unwrap()))
        .unwrap()((*qdaemon).qconn, z, 6 as libc::c_int as size_t, 0 as libc::c_int) == 0
    {
        return 0 as libc::c_int;
    }
    ulog(
        LOG_NORMAL,
        b"Protocol '%c' packets: sent %ld, resent %ld, received %ld\0" as *const u8
            as *const libc::c_char,
        (*(*qdaemon).qproto).bname as libc::c_int,
        cIsent_packets,
        cIresent_packets,
        cIreceived_packets,
    );
    if cIbad_hdr != 0 as libc::c_int as libc::c_long
        || cIbad_cksum != 0 as libc::c_int as libc::c_long
        || cIbad_order != 0 as libc::c_int as libc::c_long
        || cIremote_rejects != 0 as libc::c_int as libc::c_long
    {
        ulog(
            LOG_NORMAL,
            b"Errors: header %ld, checksum %ld, order %ld, remote rejects %ld\0"
                as *const u8 as *const libc::c_char,
            cIbad_hdr,
            cIbad_cksum,
            cIbad_order,
            cIremote_rejects,
        );
    }
    iIrequest_packsize = 1024 as libc::c_int;
    iIrequest_winsize = 16 as libc::c_int;
    iIforced_remote_packsize = 0 as libc::c_int;
    cIsync_timeout = 10 as libc::c_int;
    cIsync_retries = 6 as libc::c_int;
    cItimeout = 10 as libc::c_int;
    cIwindow_timeout = 10 as libc::c_int;
    cIretries = 6 as libc::c_int;
    cIerrors = 100 as libc::c_int;
    cIerror_decay = 10 as libc::c_int;
    cIack_frequency = 0 as libc::c_int;
    zJavoid_parameter = b"\\021\\023\0" as *const u8 as *const libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fisendcmd(
    mut qdaemon: *mut sdaemon,
    mut z: *const libc::c_char,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
) -> boolean {
    let mut clen: size_t = 0;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fisendcmd: Sending command \"%s\"\0" as *const u8 as *const libc::c_char,
            z,
        );
    }
    clen = strlen(z);
    loop {
        let mut zpacket: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut csize: size_t = 0;
        zpacket = zigetspace(qdaemon, &mut csize);
        if clen < csize {
            memcpy(
                zpacket as *mut libc::c_void,
                z as *const libc::c_void,
                clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            return fisenddata(
                qdaemon,
                zpacket,
                clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ilocal,
                iremote,
                -(1 as libc::c_int) as libc::c_long,
            );
        }
        memcpy(zpacket as *mut libc::c_void, z as *const libc::c_void, csize);
        z = z.offset(csize as isize);
        clen = (clen as libc::c_ulong).wrapping_sub(csize) as size_t as size_t;
        if fisenddata(
            qdaemon,
            zpacket,
            csize,
            ilocal,
            iremote,
            -(1 as libc::c_int) as libc::c_long,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn finak(mut qdaemon: *mut sdaemon, mut iseq: libc::c_int) -> boolean {
    let mut abnak: [libc::c_char; 6] = [0; 6];
    abnak[0 as libc::c_int as usize] = '\u{7}' as i32 as libc::c_char;
    abnak[1 as libc::c_int
        as usize] = (iseq << 3 as libc::c_int | 0 as libc::c_int) as libc::c_char;
    abnak[2 as libc::c_int
        as usize] = (iIrecseq << 3 as libc::c_int | 0 as libc::c_int) as libc::c_char;
    iIlocal_ack = iIrecseq;
    abnak[3 as libc::c_int
        as usize] = ((3 as libc::c_int) << 5 as libc::c_int
        | (if (*qdaemon).fcaller != 0 {
            (1 as libc::c_int) << 4 as libc::c_int
        } else {
            0 as libc::c_int
        }) | 0 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int) as libc::c_char;
    abnak[4 as libc::c_int
        as usize] = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    abnak[5 as libc::c_int
        as usize] = ((abnak[1 as libc::c_int as usize] as libc::c_int
        ^ abnak[2 as libc::c_int as usize] as libc::c_int
        ^ abnak[3 as libc::c_int as usize] as libc::c_int
        ^ abnak[4 as libc::c_int as usize] as libc::c_int) & 0xff as libc::c_int)
        as libc::c_char;
    afInaked[iseq as usize] = 1 as libc::c_int;
    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"finak: Sending NAK %d\0" as *const u8 as *const libc::c_char,
            iseq,
        );
    }
    return (Some(pfIsend.unwrap()))
        .unwrap()(
        (*qdaemon).qconn,
        abnak.as_mut_ptr(),
        6 as libc::c_int as size_t,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn firesend(mut qdaemon: *mut sdaemon) -> boolean {
    let mut iseq: libc::c_int = 0;
    let mut zhdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clen: size_t = 0;
    iseq = iIremote_ack + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int;
    if iseq == iIsendseq {
        return 1 as libc::c_int;
    }
    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"firesend: Resending packet %d\0" as *const u8 as *const libc::c_char,
            iseq,
        );
    }
    zhdr = (azIsendbuffers[iseq as usize])
        .offset(
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_sub(
                            (6 as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                ),
                        ),
                )
                .wrapping_sub(6 as libc::c_int as libc::c_ulong) as isize,
        );
    if *zhdr.offset(2 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
        & 0x1f as libc::c_int != iIrecseq
    {
        let mut iremote: libc::c_int = 0;
        iremote = *zhdr.offset(2 as libc::c_int as isize) as libc::c_int
            & 0x7 as libc::c_int;
        *zhdr
            .offset(
                2 as libc::c_int as isize,
            ) = (iIrecseq << 3 as libc::c_int | iremote) as libc::c_char;
        *zhdr
            .offset(
                5 as libc::c_int as isize,
            ) = ((*zhdr.offset(1 as libc::c_int as isize) as libc::c_int
            ^ *zhdr.offset(2 as libc::c_int as isize) as libc::c_int
            ^ *zhdr.offset(3 as libc::c_int as isize) as libc::c_int
            ^ *zhdr.offset(4 as libc::c_int as isize) as libc::c_int)
            & 0xff as libc::c_int) as libc::c_char;
        iIlocal_ack = iIrecseq;
    }
    cIresent_packets += 1;
    cIresent_packets;
    clen = ((*zhdr.offset(3 as libc::c_int as isize) as libc::c_int & 0xf as libc::c_int)
        << 8 as libc::c_int
        | *zhdr.offset(4 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        as size_t;
    return (Some(pfIsend.unwrap()))
        .unwrap()(
        (*qdaemon).qconn,
        zhdr,
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_add(clen)
            .wrapping_add(
                (if clen > 0 as libc::c_int as libc::c_ulong {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong,
            ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn fiwindow_wait(mut qdaemon: *mut sdaemon) -> boolean {
    while iIsendseq + 32 as libc::c_int - iIremote_ack
        & 32 as libc::c_int - 1 as libc::c_int > iIremote_winsize
    {
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fiwindow_wait: Waiting for ACK\0" as *const u8 as *const libc::c_char,
            );
        }
        if fiwait_for_packet(
            qdaemon,
            cIwindow_timeout,
            cIretries,
            1 as libc::c_int,
            0 as *mut libc::c_void as *mut boolean,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zigetspace(
    mut qdaemon: *mut sdaemon,
    mut pclen: *mut size_t,
) -> *mut libc::c_char {
    *pclen = iIremote_packsize as size_t;
    return (azIsendbuffers[iIsendseq as usize])
        .offset(
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_sub(
                            (6 as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                ),
                        ),
                ) as isize,
        );
}
pub unsafe extern "C" fn fisenddata(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
    mut ipos: libc::c_long,
) -> boolean {
    let mut zhdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut icksum: libc::c_ulong = 0;
    let mut fret: boolean = 0;
    if ilocal < 0 as libc::c_int || ilocal >= 8 as libc::c_int
        || iremote < 0 as libc::c_int || iremote >= 8 as libc::c_int
    {
        ulog(
            LOG_FATAL,
            b"fisenddata: ilocal %d iremote %d\0" as *const u8 as *const libc::c_char,
            ilocal,
            iremote,
        );
    }
    if ipos != iIsendpos && ipos != -(1 as libc::c_int) as libc::c_long {
        let mut inext: libc::c_int = 0;
        let mut zspos: *mut libc::c_char = 0 as *mut libc::c_char;
        inext = iIsendseq + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int;
        zspos = azIsendbuffers[inext as usize];
        azIsendbuffers[inext
            as usize] = zdata
            .offset(
                -((6 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_sub(
                                (6 as libc::c_int as libc::c_ulong)
                                    .wrapping_rem(
                                        ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                    ),
                            ),
                    ) as isize),
            );
        azIsendbuffers[iIsendseq as usize] = zspos;
        zspos = zspos
            .offset(
                (6 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_sub(
                                (6 as libc::c_int as libc::c_ulong)
                                    .wrapping_rem(
                                        ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                    ),
                            ),
                    )
                    .wrapping_sub(6 as libc::c_int as libc::c_ulong) as isize,
            );
        *zspos.offset(0 as libc::c_int as isize) = '\u{7}' as i32 as libc::c_char;
        *zspos
            .offset(
                1 as libc::c_int as isize,
            ) = (iIsendseq << 3 as libc::c_int | 0 as libc::c_int) as libc::c_char;
        *zspos
            .offset(
                2 as libc::c_int as isize,
            ) = (iIrecseq << 3 as libc::c_int | 0 as libc::c_int) as libc::c_char;
        iIlocal_ack = iIrecseq;
        *zspos
            .offset(
                3 as libc::c_int as isize,
            ) = ((4 as libc::c_int) << 5 as libc::c_int
            | (if (*qdaemon).fcaller != 0 {
                (1 as libc::c_int) << 4 as libc::c_int
            } else {
                0 as libc::c_int
            }) | 4 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int)
            as libc::c_char;
        *zspos
            .offset(
                4 as libc::c_int as isize,
            ) = (4 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
        *zspos
            .offset(
                5 as libc::c_int as isize,
            ) = ((*zspos.offset(1 as libc::c_int as isize) as libc::c_int
            ^ *zspos.offset(2 as libc::c_int as isize) as libc::c_int
            ^ *zspos.offset(3 as libc::c_int as isize) as libc::c_int
            ^ *zspos.offset(4 as libc::c_int as isize) as libc::c_int)
            & 0xff as libc::c_int) as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (ipos as libc::c_ulong >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (ipos as libc::c_ulong >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (ipos as libc::c_ulong >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (ipos as libc::c_ulong & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        icksum = icrc(
            zspos.offset(6 as libc::c_int as isize),
            4 as libc::c_int as size_t,
            0xffffffff as libc::c_ulong,
        );
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(4 as libc::c_int as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (icksum >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(4 as libc::c_int as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (icksum >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(4 as libc::c_int as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (icksum >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *zspos
            .offset(6 as libc::c_int as isize)
            .offset(4 as libc::c_int as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (icksum & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        if iIremote_winsize > 0 as libc::c_int
            && iIsendseq + 32 as libc::c_int - iIremote_ack
                & 32 as libc::c_int - 1 as libc::c_int > iIremote_winsize
        {
            if fiwindow_wait(qdaemon) == 0 {
                return 0 as libc::c_int;
            }
        }
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fisenddata: Sending SPOS %ld\0" as *const u8 as *const libc::c_char,
                ipos,
            );
        }
        if (Some(pfIsend.unwrap()))
            .unwrap()(
            (*qdaemon).qconn,
            zspos,
            (6 as libc::c_int + 4 as libc::c_int + 4 as libc::c_int) as size_t,
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        iIsendseq = iIsendseq + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int;
        iIsendpos = ipos;
    }
    zhdr = zdata.offset(-(6 as libc::c_int as isize));
    *zhdr.offset(0 as libc::c_int as isize) = '\u{7}' as i32 as libc::c_char;
    *zhdr
        .offset(
            1 as libc::c_int as isize,
        ) = (iIsendseq << 3 as libc::c_int | ilocal) as libc::c_char;
    *zhdr
        .offset(
            3 as libc::c_int as isize,
        ) = (((0 as libc::c_int) << 5 as libc::c_int
        | (if (*qdaemon).fcaller != 0 {
            (1 as libc::c_int) << 4 as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_ulong
        | cdata >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_ulong)
        as libc::c_char;
    *zhdr
        .offset(
            4 as libc::c_int as isize,
        ) = (cdata & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    if cdata > 0 as libc::c_int as libc::c_ulong {
        icksum = icrc(zdata, cdata, 0xffffffff as libc::c_ulong);
        *zdata
            .offset(cdata as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (icksum >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *zdata
            .offset(cdata as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (icksum >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *zdata
            .offset(cdata as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (icksum >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_char;
        *zdata
            .offset(cdata as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (icksum & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    }
    if iIremote_winsize > 0 as libc::c_int
        && iIsendseq + 32 as libc::c_int - iIremote_ack
            & 32 as libc::c_int - 1 as libc::c_int > iIremote_winsize
    {
        if fiwindow_wait(qdaemon) == 0 {
            return 0 as libc::c_int;
        }
    }
    *zhdr
        .offset(
            2 as libc::c_int as isize,
        ) = (iIrecseq << 3 as libc::c_int | iremote) as libc::c_char;
    iIlocal_ack = iIrecseq;
    *zhdr
        .offset(
            5 as libc::c_int as isize,
        ) = ((*zhdr.offset(1 as libc::c_int as isize) as libc::c_int
        ^ *zhdr.offset(2 as libc::c_int as isize) as libc::c_int
        ^ *zhdr.offset(3 as libc::c_int as isize) as libc::c_int
        ^ *zhdr.offset(4 as libc::c_int as isize) as libc::c_int) & 0xff as libc::c_int)
        as libc::c_char;
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fisenddata: Sending packet %d size %d local %d remote %d\0" as *const u8
                as *const libc::c_char,
            iIsendseq,
            cdata as libc::c_int,
            ilocal,
            iremote,
        );
    }
    iIsendseq = iIsendseq + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int;
    cIsent_packets += 1;
    cIsent_packets;
    fret = (Some(pfIsend.unwrap()))
        .unwrap()(
        (*qdaemon).qconn,
        zhdr,
        cdata
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if cdata > 0 as libc::c_int as libc::c_ulong {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong,
            ),
        1 as libc::c_int,
    );
    iIsendpos = (iIsendpos as libc::c_ulong).wrapping_add(cdata) as libc::c_long
        as libc::c_long;
    if fret != 0 && iPrecstart != iPrecend {
        let mut fexit: boolean = 0;
        fret = fiprocess_data(
            qdaemon,
            &mut fexit,
            0 as *mut libc::c_void as *mut boolean,
            0 as *mut libc::c_void as *mut size_t,
        );
    }
    return fret;
}
pub unsafe extern "C" fn fiwait(mut qdaemon: *mut sdaemon) -> boolean {
    return fiwait_for_packet(
        qdaemon,
        cItimeout,
        cIretries,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut boolean,
    );
}
unsafe extern "C" fn fiwait_for_packet(
    mut qdaemon: *mut sdaemon,
    mut ctimeout: libc::c_int,
    mut cretries: libc::c_int,
    mut fone: boolean,
    mut pftimedout: *mut boolean,
) -> boolean {
    let mut cshort: libc::c_int = 0;
    let mut ctimeouts: libc::c_int = 0;
    if !pftimedout.is_null() {
        *pftimedout = 0 as libc::c_int;
    }
    cshort = 0 as libc::c_int;
    ctimeouts = 0 as libc::c_int;
    loop {
        let mut fexit: boolean = 0;
        let mut ffound: boolean = 0;
        let mut cneed: size_t = 0;
        let mut crec: size_t = 0;
        if fiprocess_data(qdaemon, &mut fexit, &mut ffound, &mut cneed) == 0 {
            return 0 as libc::c_int;
        }
        if fexit != 0 || fone != 0 && ffound != 0 {
            return 1 as libc::c_int;
        }
        if cneed == 0 as libc::c_int as libc::c_ulong {
            continue;
        }
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fiwait_for_packet: Need %d bytes\0" as *const u8
                    as *const libc::c_char,
                cneed as libc::c_int,
            );
        }
        if (Some(pfIreceive.unwrap()))
            .unwrap()((*qdaemon).qconn, cneed, &mut crec, ctimeout, 1 as libc::c_int)
            == 0
        {
            return 0 as libc::c_int;
        }
        if crec != 0 as libc::c_int as libc::c_ulong {
            if crec >= cneed {
                cshort = 0 as libc::c_int;
            } else {
                cshort += 1;
                cshort;
                if cshort > 1 as libc::c_int {
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                    cshort = 0 as libc::c_int;
                }
            }
        } else {
            let mut i: libc::c_int = 0;
            ctimeouts += 1;
            ctimeouts;
            if ctimeouts > cretries {
                if cretries > 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"Timed out waiting for packet\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !pftimedout.is_null() {
                    *pftimedout = 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                afInaked[i as usize] = 0 as libc::c_int;
                i += 1;
                i;
            }
            if finak(
                qdaemon,
                iIrecseq + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int,
            ) == 0 || firesend(qdaemon) == 0
            {
                return 0 as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn ficheck_errors(mut qdaemon: *mut sdaemon) -> boolean {
    if cIerrors < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if cIbad_order + cIbad_hdr + cIbad_cksum + cIremote_rejects
        - cIreceived_packets / cIerror_decay as libc::c_long > cIerrors as libc::c_long
    {
        if iIrequest_packsize > 400 as libc::c_int {
            let mut absync: [libc::c_char; 13] = [0; 13];
            let mut icksum: libc::c_ulong = 0;
            iIrequest_packsize /= 2 as libc::c_int;
            absync[0 as libc::c_int as usize] = '\u{7}' as i32 as libc::c_char;
            absync[1 as libc::c_int
                as usize] = ((0 as libc::c_int) << 3 as libc::c_int | 0 as libc::c_int)
                as libc::c_char;
            absync[2 as libc::c_int
                as usize] = (iIrecseq << 3 as libc::c_int | 0 as libc::c_int)
                as libc::c_char;
            iIlocal_ack = iIrecseq;
            absync[3 as libc::c_int
                as usize] = ((1 as libc::c_int) << 5 as libc::c_int
                | (if (*qdaemon).fcaller != 0 {
                    (1 as libc::c_int) << 4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | 3 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int)
                as libc::c_char;
            absync[4 as libc::c_int
                as usize] = (3 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
            absync[5 as libc::c_int
                as usize] = ((absync[1 as libc::c_int as usize] as libc::c_int
                ^ absync[2 as libc::c_int as usize] as libc::c_int
                ^ absync[3 as libc::c_int as usize] as libc::c_int
                ^ absync[4 as libc::c_int as usize] as libc::c_int)
                & 0xff as libc::c_int) as libc::c_char;
            absync[(6 as libc::c_int + 0 as libc::c_int)
                as usize] = (iIrequest_packsize >> 8 as libc::c_int
                & 0xff as libc::c_int) as libc::c_char;
            absync[(6 as libc::c_int + 1 as libc::c_int)
                as usize] = (iIrequest_packsize & 0xff as libc::c_int) as libc::c_char;
            absync[(6 as libc::c_int + 2 as libc::c_int)
                as usize] = iIrequest_winsize as libc::c_char;
            icksum = icrc(
                absync.as_mut_ptr().offset(6 as libc::c_int as isize),
                3 as libc::c_int as size_t,
                0xffffffff as libc::c_ulong,
            );
            *absync
                .as_mut_ptr()
                .offset(6 as libc::c_int as isize)
                .offset(3 as libc::c_int as isize)
                .offset(
                    0 as libc::c_int as isize,
                ) = (icksum >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_char;
            *absync
                .as_mut_ptr()
                .offset(6 as libc::c_int as isize)
                .offset(3 as libc::c_int as isize)
                .offset(
                    1 as libc::c_int as isize,
                ) = (icksum >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_char;
            *absync
                .as_mut_ptr()
                .offset(6 as libc::c_int as isize)
                .offset(3 as libc::c_int as isize)
                .offset(
                    2 as libc::c_int as isize,
                ) = (icksum >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_char;
            *absync
                .as_mut_ptr()
                .offset(6 as libc::c_int as isize)
                .offset(3 as libc::c_int as isize)
                .offset(
                    3 as libc::c_int as isize,
                ) = (icksum & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
            cIerrors *= 2 as libc::c_int;
            if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"ficheck_errors: Sending SYNC packsize %d winsize %d\0" as *const u8
                        as *const libc::c_char,
                    iIrequest_packsize,
                    iIrequest_winsize,
                );
            }
            return (Some(pfIsend.unwrap()))
                .unwrap()(
                (*qdaemon).qconn,
                absync.as_mut_ptr(),
                (6 as libc::c_int + 3 as libc::c_int + 4 as libc::c_int) as size_t,
                1 as libc::c_int,
            );
        }
        ulog(
            LOG_ERROR,
            b"Too many '%c' protocol errors\0" as *const u8 as *const libc::c_char,
            (*(*qdaemon).qproto).bname as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fiprocess_data(
    mut qdaemon: *mut sdaemon,
    mut pfexit: *mut boolean,
    mut pffound: *mut boolean,
    mut pcneed: *mut size_t,
) -> boolean {
    let mut fbadhdr: boolean = 0;
    if !pfexit.is_null() {
        *pfexit = 0 as libc::c_int;
    }
    if !pffound.is_null() {
        *pffound = 0 as libc::c_int;
    }
    fbadhdr = 0 as libc::c_int;
    while iPrecstart != iPrecend {
        let mut ab: [libc::c_char; 6] = [0; 6];
        let mut cfirst: libc::c_int = 0;
        let mut csecond: libc::c_int = 0;
        let mut zfirst: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zsecond: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        let mut iget: libc::c_int = 0;
        let mut ttype: libc::c_int = 0;
        let mut iseq: libc::c_int = 0;
        let mut csize: libc::c_int = 0;
        let mut iack: libc::c_int = 0;
        if fIclosing != 0 {
            if !pfexit.is_null() {
                *pfexit = 1 as libc::c_int;
            }
            if !pcneed.is_null() {
                *pcneed = 0 as libc::c_int as size_t;
            }
            return 1 as libc::c_int;
        }
        if abPrecbuf[iPrecstart as usize] as libc::c_int != '\u{7}' as i32 {
            let mut zintro: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cintro: libc::c_int = 0;
            cintro = iPrecend - iPrecstart;
            if cintro < 0 as libc::c_int {
                cintro = 16384 as libc::c_int - iPrecstart;
            }
            zintro = memchr(
                abPrecbuf.as_mut_ptr().offset(iPrecstart as isize)
                    as *const libc::c_void,
                '\u{7}' as i32,
                cintro as size_t,
            ) as *mut libc::c_char;
            if zintro.is_null() {
                iPrecstart = (iPrecstart + cintro) % 16384 as libc::c_int;
                continue;
            } else {
                iPrecstart = (iPrecstart as libc::c_long
                    + zintro
                        .offset_from(abPrecbuf.as_mut_ptr().offset(iPrecstart as isize))
                        as libc::c_long) as libc::c_int;
            }
        }
        i = 0 as libc::c_int;
        iget = iPrecstart;
        while i < 6 as libc::c_int && iget != iPrecend {
            ab[i as usize] = abPrecbuf[iget as usize];
            i += 1;
            i;
            iget = (iget + 1 as libc::c_int) % 16384 as libc::c_int;
        }
        if i < 6 as libc::c_int {
            if !pcneed.is_null() {
                *pcneed = (6 as libc::c_int - i) as size_t;
            }
            return 1 as libc::c_int;
        }
        if ab[5 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int
            != (ab[1 as libc::c_int as usize] as libc::c_int
                ^ ab[2 as libc::c_int as usize] as libc::c_int
                ^ ab[3 as libc::c_int as usize] as libc::c_int
                ^ ab[4 as libc::c_int as usize] as libc::c_int) & 0xff as libc::c_int
            || (if ab[3 as libc::c_int as usize] as libc::c_int
                & (1 as libc::c_int) << 4 as libc::c_int != 0 as libc::c_int
            {
                (*qdaemon).fcaller
            } else {
                ((*qdaemon).fcaller == 0) as libc::c_int
            }) != 0
        {
            if fbadhdr == 0 {
                if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                    != 0 as libc::c_int
                {
                    ulog(
                        LOG_DEBUG,
                        b"fiprocess_data: Bad header\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                cIbad_hdr += 1;
                cIbad_hdr;
                if ficheck_errors(qdaemon) == 0 {
                    return 0 as libc::c_int;
                }
                fbadhdr = 1 as libc::c_int;
            }
            iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            zsecond = 0 as *mut libc::c_char;
            zfirst = zsecond;
            csecond = 0 as libc::c_int;
            cfirst = csecond;
            ttype = ab[3 as libc::c_int as usize] as libc::c_int >> 5 as libc::c_int
                & 0x7 as libc::c_int;
            if ttype == 0 as libc::c_int || ttype == 4 as libc::c_int
                || ttype == 5 as libc::c_int
            {
                iseq = ab[1 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                    & 0x1f as libc::c_int;
            } else {
                iseq = -(1 as libc::c_int);
            }
            csize = (ab[3 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int)
                << 8 as libc::c_int
                | ab[4 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int;
            if iseq != -(1 as libc::c_int) {
                if iIrequest_winsize > 0 as libc::c_int
                    && iseq + 32 as libc::c_int - iIlocal_ack
                        & 32 as libc::c_int - 1 as libc::c_int > iIrequest_winsize
                {
                    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fiprocess_data: Out of order packet %d (ack %d)\0"
                                as *const u8 as *const libc::c_char,
                            iseq,
                            iIlocal_ack,
                        );
                    }
                    cIbad_order += 1;
                    cIbad_order;
                    if ficheck_errors(qdaemon) == 0 {
                        return 0 as libc::c_int;
                    }
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                    continue;
                }
            }
            if csize > 0 as libc::c_int {
                let mut cinbuf: libc::c_int = 0;
                let mut abcksum: [libc::c_char; 4] = [0; 4];
                let mut ickdata: libc::c_ulong = 0;
                cinbuf = iPrecend - iPrecstart;
                if cinbuf < 0 as libc::c_int {
                    cinbuf += 16384 as libc::c_int;
                }
                if cinbuf < 6 as libc::c_int + csize + 4 as libc::c_int {
                    if !pcneed.is_null() {
                        *pcneed = (6 as libc::c_int + csize + 4 as libc::c_int - cinbuf)
                            as size_t;
                    }
                    return 1 as libc::c_int;
                }
                if iPrecend > iPrecstart {
                    cfirst = csize;
                    zfirst = abPrecbuf
                        .as_mut_ptr()
                        .offset(iPrecstart as isize)
                        .offset(6 as libc::c_int as isize);
                } else {
                    cfirst = 16384 as libc::c_int - (iPrecstart + 6 as libc::c_int);
                    if cfirst <= 0 as libc::c_int {
                        zfirst = abPrecbuf.as_mut_ptr().offset(-(cfirst as isize));
                        cfirst = csize;
                    } else {
                        if cfirst >= csize {
                            cfirst = csize;
                        } else {
                            zsecond = abPrecbuf.as_mut_ptr();
                            csecond = csize - cfirst;
                        }
                        zfirst = abPrecbuf
                            .as_mut_ptr()
                            .offset(iPrecstart as isize)
                            .offset(6 as libc::c_int as isize);
                    }
                }
                i = 0 as libc::c_int;
                iget = (iPrecstart + 6 as libc::c_int + csize) % 16384 as libc::c_int;
                while i < 4 as libc::c_int {
                    abcksum[i as usize] = abPrecbuf[iget as usize];
                    i += 1;
                    i;
                    iget = (iget + 1 as libc::c_int) % 16384 as libc::c_int;
                }
                ickdata = icrc(zfirst, cfirst as size_t, 0xffffffff as libc::c_ulong);
                if csecond > 0 as libc::c_int {
                    ickdata = icrc(zsecond, csecond as size_t, ickdata);
                }
                if ((((abcksum[0 as libc::c_int as usize] as libc::c_int
                    & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                    | (abcksum[1 as libc::c_int as usize] as libc::c_int
                        & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                    | (abcksum[2 as libc::c_int as usize] as libc::c_int
                        & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                    | (abcksum[3 as libc::c_int as usize] as libc::c_int
                        & 0xff as libc::c_int) as libc::c_ulong != ickdata
                {
                    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fiprocess_data: Bad checksum; data %lu, frame %lu\0"
                                as *const u8 as *const libc::c_char,
                            ickdata,
                            ((((abcksum[0 as libc::c_int as usize] as libc::c_int
                                & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                                | (abcksum[1 as libc::c_int as usize] as libc::c_int
                                    & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                                | (abcksum[2 as libc::c_int as usize] as libc::c_int
                                    & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                                | (abcksum[3 as libc::c_int as usize] as libc::c_int
                                    & 0xff as libc::c_int) as libc::c_ulong,
                        );
                    }
                    cIbad_cksum += 1;
                    cIbad_cksum;
                    if ficheck_errors(qdaemon) == 0 {
                        return 0 as libc::c_int;
                    }
                    if iseq != -(1 as libc::c_int) && iseq != iIrecseq
                        && (iIrequest_winsize <= 0 as libc::c_int
                            || iseq + 32 as libc::c_int - iIrecseq
                                & 32 as libc::c_int - 1 as libc::c_int <= iIrequest_winsize)
                        && (azIrecbuffers[iseq as usize]).is_null()
                    {
                        if finak(qdaemon, iseq) == 0 {
                            return 0 as libc::c_int;
                        }
                    }
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                    continue;
                }
            }
            if csize == 0 as libc::c_int {
                iPrecstart = (iPrecstart + 6 as libc::c_int) % 16384 as libc::c_int;
            } else {
                iPrecstart = (iPrecstart + 6 as libc::c_int + csize + 4 as libc::c_int)
                    % 16384 as libc::c_int;
                cIreceived_packets += 1;
                cIreceived_packets;
            }
            iack = ab[2 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                & 0x1f as libc::c_int;
            if iIremote_winsize > 0 as libc::c_int && iack != iIsendseq
                && iack + 32 as libc::c_int - iIremote_ack
                    & 32 as libc::c_int - 1 as libc::c_int <= iIremote_winsize
                && iIsendseq + 32 as libc::c_int - iack
                    & 32 as libc::c_int - 1 as libc::c_int <= iIremote_winsize
            {
                if iack < iIremote_ack {
                    uwindow_acked(qdaemon, 0 as libc::c_int);
                }
                iIremote_ack = iack;
            }
            if iseq != -(1 as libc::c_int) {
                if afInaked[iseq as usize] != 0
                    && (azIrecbuffers[(iseq + 32 as libc::c_int - 1 as libc::c_int
                        & 32 as libc::c_int - 1 as libc::c_int) as usize])
                        .is_null()
                {
                    i = iIrecseq + 1 as libc::c_int
                        & 32 as libc::c_int - 1 as libc::c_int;
                    while i != iseq {
                        afInaked[i as usize] = 0 as libc::c_int;
                        i = i + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int;
                    }
                    afInaked[iseq as usize] = 0 as libc::c_int;
                }
                if iseq
                    != iIrecseq + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int
                {
                    if iseq == iIrecseq
                        || iIrequest_winsize > 0 as libc::c_int
                            && iseq + 32 as libc::c_int - iIrecseq
                                & 32 as libc::c_int - 1 as libc::c_int > iIrequest_winsize
                    {
                        if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                            != 0 as libc::c_int
                        {
                            ulog(
                                LOG_DEBUG,
                                b"fiprocess_data: Ignoring out of order packet %d (recseq %d)\0"
                                    as *const u8 as *const libc::c_char,
                                iseq,
                                iIrecseq,
                            );
                        }
                        continue;
                    } else {
                        if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                            != 0 as libc::c_int
                        {
                            ulog(
                                LOG_DEBUG,
                                b"fiprocess_data: Saving unexpected packet %d (recseq %d)\0"
                                    as *const u8 as *const libc::c_char,
                                iseq,
                                iIrecseq,
                            );
                        }
                        if (azIrecbuffers[iseq as usize]).is_null() {
                            azIrecbuffers[iseq
                                as usize] = zbufalc((6 as libc::c_int + csize) as size_t);
                            memcpy(
                                azIrecbuffers[iseq as usize] as *mut libc::c_void,
                                ab.as_mut_ptr() as *const libc::c_void,
                                6 as libc::c_int as libc::c_ulong,
                            );
                            if csize > 0 as libc::c_int {
                                memcpy(
                                    (azIrecbuffers[iseq as usize])
                                        .offset(6 as libc::c_int as isize) as *mut libc::c_void,
                                    zfirst as *const libc::c_void,
                                    cfirst as size_t,
                                );
                                if csecond > 0 as libc::c_int {
                                    memcpy(
                                        (azIrecbuffers[iseq as usize])
                                            .offset(6 as libc::c_int as isize)
                                            .offset(cfirst as isize) as *mut libc::c_void,
                                        zsecond as *const libc::c_void,
                                        csecond as size_t,
                                    );
                                }
                            }
                        }
                        i = iIrecseq + 1 as libc::c_int
                            & 32 as libc::c_int - 1 as libc::c_int;
                        while i != iseq {
                            if afInaked[i as usize] == 0
                                && (azIrecbuffers[i as usize]).is_null()
                            {
                                if finak(qdaemon, i) == 0 {
                                    return 0 as libc::c_int;
                                }
                            }
                            i = i + 1 as libc::c_int
                                & 32 as libc::c_int - 1 as libc::c_int;
                        }
                        continue;
                    }
                } else {
                    iIrecseq = iseq;
                }
            }
            if !pffound.is_null() {
                *pffound = 1 as libc::c_int;
            }
            if fiprocess_packet(
                qdaemon,
                ab.as_mut_ptr(),
                zfirst,
                cfirst,
                zsecond,
                csecond,
                pfexit,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if iseq != -(1 as libc::c_int) {
                let mut inext: libc::c_int = 0;
                inext = iIrecseq + 1 as libc::c_int
                    & 32 as libc::c_int - 1 as libc::c_int;
                while !(azIrecbuffers[inext as usize]).is_null() {
                    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut c: libc::c_int = 0;
                    z = azIrecbuffers[inext as usize];
                    c = (*z.offset(3 as libc::c_int as isize) as libc::c_int
                        & 0xf as libc::c_int) << 8 as libc::c_int
                        | *z.offset(4 as libc::c_int as isize) as libc::c_int
                            & 0xff as libc::c_int;
                    iIrecseq = inext;
                    if fiprocess_packet(
                        qdaemon,
                        z,
                        z.offset(6 as libc::c_int as isize),
                        c,
                        0 as *mut libc::c_void as *mut libc::c_char,
                        0 as libc::c_int,
                        pfexit,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    ubuffree(azIrecbuffers[inext as usize]);
                    azIrecbuffers[inext as usize] = 0 as *mut libc::c_char;
                    inext = inext + 1 as libc::c_int
                        & 32 as libc::c_int - 1 as libc::c_int;
                }
            }
            if iIrequest_winsize > 0 as libc::c_int
                && iIrecseq + 32 as libc::c_int - iIlocal_ack
                    & 32 as libc::c_int - 1 as libc::c_int >= cIack_frequency
            {
                let mut aback: [libc::c_char; 6] = [0; 6];
                aback[0 as libc::c_int as usize] = '\u{7}' as i32 as libc::c_char;
                aback[1 as libc::c_int
                    as usize] = ((0 as libc::c_int) << 3 as libc::c_int
                    | 0 as libc::c_int) as libc::c_char;
                aback[2 as libc::c_int
                    as usize] = (iIrecseq << 3 as libc::c_int | 0 as libc::c_int)
                    as libc::c_char;
                iIlocal_ack = iIrecseq;
                aback[3 as libc::c_int
                    as usize] = ((2 as libc::c_int) << 5 as libc::c_int
                    | (if (*qdaemon).fcaller != 0 {
                        (1 as libc::c_int) << 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) | 0 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int)
                    as libc::c_char;
                aback[4 as libc::c_int
                    as usize] = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
                aback[5 as libc::c_int
                    as usize] = ((aback[1 as libc::c_int as usize] as libc::c_int
                    ^ aback[2 as libc::c_int as usize] as libc::c_int
                    ^ aback[3 as libc::c_int as usize] as libc::c_int
                    ^ aback[4 as libc::c_int as usize] as libc::c_int)
                    & 0xff as libc::c_int) as libc::c_char;
                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fiprocess_data: Sending ACK %d\0" as *const u8
                            as *const libc::c_char,
                        iIrecseq,
                    );
                }
                if (Some(pfIsend.unwrap()))
                    .unwrap()(
                    (*qdaemon).qconn,
                    aback.as_mut_ptr(),
                    6 as libc::c_int as size_t,
                    1 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    if !pcneed.is_null() {
        *pcneed = 6 as libc::c_int as size_t;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fiprocess_packet(
    mut qdaemon: *mut sdaemon,
    mut zhdr: *const libc::c_char,
    mut zfirst: *const libc::c_char,
    mut cfirst: libc::c_int,
    mut zsecond: *const libc::c_char,
    mut csecond: libc::c_int,
    mut pfexit: *mut boolean,
) -> boolean {
    let mut ttype: libc::c_int = 0;
    ttype = *zhdr.offset(3 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int
        & 0x7 as libc::c_int;
    match ttype {
        0 => {
            let mut iseq: libc::c_int = 0;
            let mut fret: boolean = 0;
            iseq = *zhdr.offset(1 as libc::c_int as isize) as libc::c_int
                >> 3 as libc::c_int & 0x1f as libc::c_int;
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fiprocess_packet: Got DATA packet %d size %d local %d remote %d\0"
                        as *const u8 as *const libc::c_char,
                    iseq,
                    cfirst + csecond,
                    *zhdr.offset(2 as libc::c_int as isize) as libc::c_int
                        & 0x7 as libc::c_int,
                    *zhdr.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0x7 as libc::c_int,
                );
            }
            fret = fgot_data(
                qdaemon,
                zfirst,
                cfirst as size_t,
                zsecond,
                csecond as size_t,
                *zhdr.offset(2 as libc::c_int as isize) as libc::c_int
                    & 0x7 as libc::c_int,
                *zhdr.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0x7 as libc::c_int,
                iIrecpos,
                (iIremote_ack + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int
                    == iIsendseq) as libc::c_int,
                pfexit,
            );
            iIrecpos += (cfirst + csecond) as libc::c_long;
            return fret;
        }
        1 => {
            let mut ipack: libc::c_int = 0;
            let mut iwin: libc::c_int = 0;
            let mut cchans: libc::c_int = 0;
            if cfirst + csecond < 3 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"Bad SYNC packet\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            ipack = (*zfirst.offset(0 as libc::c_int as isize) as libc::c_int
                & 0xff as libc::c_int) << 8 as libc::c_int;
            if cfirst > 1 as libc::c_int {
                ipack
                    |= *zfirst.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0xff as libc::c_int;
            } else {
                ipack |= *zsecond.offset(0 as libc::c_int as isize) as libc::c_int;
            }
            if cfirst > 2 as libc::c_int {
                iwin = *zfirst.offset(2 as libc::c_int as isize) as libc::c_int;
            } else {
                iwin = *zsecond.offset((2 as libc::c_int - cfirst) as isize)
                    as libc::c_int;
            }
            if cfirst + csecond <= 3 as libc::c_int {
                cchans = 0 as libc::c_int;
            } else {
                if cfirst > 3 as libc::c_int {
                    cchans = *zfirst.offset(3 as libc::c_int as isize) as libc::c_int;
                } else {
                    cchans = *zsecond.offset((3 as libc::c_int - cfirst) as isize)
                        as libc::c_int;
                }
                if cchans > 0 as libc::c_int && cchans < 8 as libc::c_int {
                    (*qdaemon).cchans = cchans;
                }
            }
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fiprocess_packet: Got SYNC packsize %d winsize %d channels %d\0"
                        as *const u8 as *const libc::c_char,
                    ipack,
                    iwin,
                    cchans,
                );
            }
            if iIforced_remote_packsize == 0 as libc::c_int
                && (iIalc_packsize == 0 as libc::c_int || ipack <= iIalc_packsize)
            {
                iIremote_packsize = ipack;
            }
            iIremote_winsize = iwin;
            cIsyncs += 1;
            cIsyncs;
            *pfexit = 1 as libc::c_int;
            return 1 as libc::c_int;
        }
        2 => {
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fiprocess_packet: Got ACK %d\0" as *const u8
                        as *const libc::c_char,
                    *zhdr.offset(2 as libc::c_int as isize) as libc::c_int
                        >> 3 as libc::c_int & 0x1f as libc::c_int,
                );
            }
            return 1 as libc::c_int;
        }
        3 => {
            let mut iseq_0: libc::c_int = 0;
            let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut clen: size_t = 0;
            cIremote_rejects += 1;
            cIremote_rejects;
            if ficheck_errors(qdaemon) == 0 {
                return 0 as libc::c_int;
            }
            iseq_0 = *zhdr.offset(1 as libc::c_int as isize) as libc::c_int
                >> 3 as libc::c_int & 0x1f as libc::c_int;
            if iseq_0 == iIsendseq
                && iIremote_ack + 1 as libc::c_int & 32 as libc::c_int - 1 as libc::c_int
                    == iIsendseq
            {
                let mut aback: [libc::c_char; 6] = [0; 6];
                aback[0 as libc::c_int as usize] = '\u{7}' as i32 as libc::c_char;
                aback[1 as libc::c_int
                    as usize] = ((0 as libc::c_int) << 3 as libc::c_int
                    | 0 as libc::c_int) as libc::c_char;
                aback[2 as libc::c_int
                    as usize] = (iIrecseq << 3 as libc::c_int | 0 as libc::c_int)
                    as libc::c_char;
                iIlocal_ack = iIrecseq;
                aback[3 as libc::c_int
                    as usize] = ((2 as libc::c_int) << 5 as libc::c_int
                    | (if (*qdaemon).fcaller != 0 {
                        (1 as libc::c_int) << 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) | 0 as libc::c_int >> 8 as libc::c_int & 0xf as libc::c_int)
                    as libc::c_char;
                aback[4 as libc::c_int
                    as usize] = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
                aback[5 as libc::c_int
                    as usize] = ((aback[1 as libc::c_int as usize] as libc::c_int
                    ^ aback[2 as libc::c_int as usize] as libc::c_int
                    ^ aback[3 as libc::c_int as usize] as libc::c_int
                    ^ aback[4 as libc::c_int as usize] as libc::c_int)
                    & 0xff as libc::c_int) as libc::c_char;
                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fiprocess_packet: Sending ACK %d\0" as *const u8
                            as *const libc::c_char,
                        iIrecseq,
                    );
                }
                return (Some(pfIsend.unwrap()))
                    .unwrap()(
                    (*qdaemon).qconn,
                    aback.as_mut_ptr(),
                    6 as libc::c_int as size_t,
                    1 as libc::c_int,
                );
            } else {
                if iseq_0 == iIsendseq
                    || iIremote_winsize > 0 as libc::c_int
                        && (iseq_0 + 32 as libc::c_int - iIremote_ack
                            & 32 as libc::c_int - 1 as libc::c_int > iIremote_winsize
                            || iIsendseq + 32 as libc::c_int - iseq_0
                                & 32 as libc::c_int - 1 as libc::c_int > iIremote_winsize)
                {
                    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fiprocess_packet: Ignoring out of order NAK %d (sendseq %d)\0"
                                as *const u8 as *const libc::c_char,
                            iseq_0,
                            iIsendseq,
                        );
                    }
                    return 1 as libc::c_int;
                }
                if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                    != 0 as libc::c_int
                {
                    ulog(
                        LOG_DEBUG,
                        b"fiprocess_packet: Got NAK %d; resending packet\0" as *const u8
                            as *const libc::c_char,
                        iseq_0,
                    );
                }
                zsend = (azIsendbuffers[iseq_0 as usize])
                    .offset(
                        (6 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_sub(
                                        (6 as libc::c_int as libc::c_ulong)
                                            .wrapping_rem(
                                                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                                            ),
                                    ),
                            )
                            .wrapping_sub(6 as libc::c_int as libc::c_ulong) as isize,
                    );
                if *zsend.offset(2 as libc::c_int as isize) as libc::c_int
                    >> 3 as libc::c_int & 0x1f as libc::c_int != iIrecseq
                {
                    let mut iremote: libc::c_int = 0;
                    iremote = *zsend.offset(2 as libc::c_int as isize) as libc::c_int
                        & 0x7 as libc::c_int;
                    *zsend
                        .offset(
                            2 as libc::c_int as isize,
                        ) = (iIrecseq << 3 as libc::c_int | iremote) as libc::c_char;
                    *zsend
                        .offset(
                            5 as libc::c_int as isize,
                        ) = ((*zsend.offset(1 as libc::c_int as isize) as libc::c_int
                        ^ *zsend.offset(2 as libc::c_int as isize) as libc::c_int
                        ^ *zsend.offset(3 as libc::c_int as isize) as libc::c_int
                        ^ *zsend.offset(4 as libc::c_int as isize) as libc::c_int)
                        & 0xff as libc::c_int) as libc::c_char;
                    iIlocal_ack = iIrecseq;
                }
                cIresent_packets += 1;
                cIresent_packets;
                clen = ((*zsend.offset(3 as libc::c_int as isize) as libc::c_int
                    & 0xf as libc::c_int) << 8 as libc::c_int
                    | *zsend.offset(4 as libc::c_int as isize) as libc::c_int
                        & 0xff as libc::c_int) as size_t;
                return (Some(pfIsend.unwrap()))
                    .unwrap()(
                    (*qdaemon).qconn,
                    zsend,
                    (6 as libc::c_int as libc::c_ulong)
                        .wrapping_add(clen)
                        .wrapping_add(
                            (if clen > 0 as libc::c_int as libc::c_ulong {
                                4 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as libc::c_ulong,
                        ),
                    1 as libc::c_int,
                );
            }
        }
        4 => {
            let mut abpos: [libc::c_char; 4] = [0; 4];
            let mut zpos: *const libc::c_char = 0 as *const libc::c_char;
            if cfirst >= 4 as libc::c_int {
                zpos = zfirst;
            } else {
                memcpy(
                    abpos.as_mut_ptr() as *mut libc::c_void,
                    zfirst as *const libc::c_void,
                    cfirst as size_t,
                );
                memcpy(
                    abpos.as_mut_ptr().offset(cfirst as isize) as *mut libc::c_void,
                    zsecond as *const libc::c_void,
                    (4 as libc::c_int - cfirst) as size_t,
                );
                zpos = abpos.as_mut_ptr();
            }
            iIrecpos = (((((*zpos.offset(0 as libc::c_int as isize) as libc::c_int
                & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                | (*zpos.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                | (*zpos.offset(2 as libc::c_int as isize) as libc::c_int
                    & 0xff as libc::c_int) as libc::c_ulong) << 8 as libc::c_int
                | (*zpos.offset(3 as libc::c_int as isize) as libc::c_int
                    & 0xff as libc::c_int) as libc::c_ulong) as libc::c_long;
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fiprocess_packet: Got SPOS %ld\0" as *const u8
                        as *const libc::c_char,
                    iIrecpos,
                );
            }
            return 1 as libc::c_int;
        }
        5 => {
            let mut fexpected: boolean = 0;
            fexpected = (fLog_sighup == 0 || fIclosing != 0) as libc::c_int;
            if fexpected == 0 {
                ulog(
                    LOG_ERROR,
                    b"Received unexpected CLOSE packet\0" as *const u8
                        as *const libc::c_char,
                );
            } else if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fiprocess_packet: Got CLOSE packet\0" as *const u8
                        as *const libc::c_char,
                );
            }
            fIclosing = 1 as libc::c_int;
            *pfexit = 1 as libc::c_int;
            return fexpected;
        }
        _ => {
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fiprocess_packet: Got packet type %d\0" as *const u8
                        as *const libc::c_char,
                    ttype,
                );
            }
            return 1 as libc::c_int;
        }
    };
}
