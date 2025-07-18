use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn xfree(_: pointer);
    static mut fLog_sighup: boolean;
    fn fconn_set(
        qconn: *mut sconnection,
        tparity: tparitysetting,
        tstrip: tstripsetting,
        txonxoff: txonxoffsetting,
    ) -> boolean;
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
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
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
pub static mut protg_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: protg.c,v 1.71 2002/03/05 19:10:41 ian Rel $\0")
};
static mut iGsendseq: libc::c_int = 0;
static mut iGremote_ack: libc::c_int = 0;
static mut iGretransmit_seq: libc::c_int = 0;
static mut iGrecseq: libc::c_int = 0;
static mut iGlocal_ack: libc::c_int = 0;
static mut iGrequest_winsize: libc::c_int = 7 as libc::c_int;
static mut iGrequest_packsize: libc::c_int = 64 as libc::c_int;
static mut iGremote_winsize: libc::c_int = 0;
static mut iGforced_remote_winsize: libc::c_int = 0 as libc::c_int;
static mut iGremote_segsize: libc::c_int = 0;
static mut iGremote_packsize: size_t = 0;
static mut iGforced_remote_packsize: libc::c_int = 0 as libc::c_int;
static mut iGpacket_control: libc::c_int = 0;
static mut cGstartup_retries: libc::c_int = 8 as libc::c_int;
static mut cGexchange_init_retries: libc::c_int = 4 as libc::c_int;
static mut cGexchange_init_timeout: libc::c_int = 10 as libc::c_int;
static mut cGtimeout: libc::c_int = 10 as libc::c_int;
static mut cGretries: libc::c_int = 6 as libc::c_int;
static mut cGgarbage_data: libc::c_int = 10000 as libc::c_int;
static mut cGmax_errors: libc::c_int = 100 as libc::c_int;
static mut cGerror_decay: libc::c_int = 10 as libc::c_int;
static mut fGshort_packets: boolean = 1 as libc::c_int;
pub static mut asGproto_params: [uuconf_cmdtab; 14] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"window\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iGrequest_winsize as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"packet-size\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iGrequest_packsize as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"startup-retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGstartup_retries as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"init-timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGexchange_init_timeout as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"init-retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGexchange_init_retries as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGtimeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGretries as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"garbage\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGgarbage_data as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"errors\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGmax_errors as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"error-decay\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cGerror_decay as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"remote-window\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iGforced_remote_winsize as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"remote-packet-size\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iGforced_remote_packsize as *const libc::c_int
                    as *mut libc::c_int as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"short-packets\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x12 as libc::c_int,
                uuconf_pvar: &fGshort_packets as *const boolean as *mut boolean
                    as pointer,
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
static mut cGsent_packets: libc::c_long = 0;
static mut cGresent_packets: libc::c_long = 0;
static mut cGdelayed_packets: libc::c_long = 0;
static mut cGrec_packets: libc::c_long = 0;
static mut cGbad_hdr: libc::c_long = 0;
static mut cGbad_checksum: libc::c_long = 0;
static mut cGbad_order: libc::c_long = 0;
static mut cGremote_rejects: libc::c_long = 0;
static mut cGremote_duprrs: libc::c_long = 0;
static mut cGerror_level: libc::c_long = 0;
static mut cGexpect_bad_order: libc::c_int = 0;
static mut azGcontrol: [*const libc::c_char; 8] = [
    b"?0?\0" as *const u8 as *const libc::c_char,
    b"CLOSE\0" as *const u8 as *const libc::c_char,
    b"RJ\0" as *const u8 as *const libc::c_char,
    b"SRJ\0" as *const u8 as *const libc::c_char,
    b"RR\0" as *const u8 as *const libc::c_char,
    b"INITC\0" as *const u8 as *const libc::c_char,
    b"INITB\0" as *const u8 as *const libc::c_char,
    b"INITA\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn fgstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    let mut iseg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut fgota: boolean = 0;
    let mut fgotb: boolean = 0;
    *pzlog = 0 as *mut libc::c_char;
    if fconn_set(
        (*qdaemon).qconn,
        PARITYSETTING_NONE,
        STRIPSETTING_EIGHTBITS,
        XONXOFF_OFF,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    iGsendseq = 1 as libc::c_int;
    iGremote_ack = 0 as libc::c_int;
    iGretransmit_seq = -(1 as libc::c_int);
    iGrecseq = 0 as libc::c_int;
    iGlocal_ack = 0 as libc::c_int;
    cGsent_packets = 0 as libc::c_int as libc::c_long;
    cGresent_packets = 0 as libc::c_int as libc::c_long;
    cGdelayed_packets = 0 as libc::c_int as libc::c_long;
    cGrec_packets = 0 as libc::c_int as libc::c_long;
    cGbad_hdr = 0 as libc::c_int as libc::c_long;
    cGbad_checksum = 0 as libc::c_int as libc::c_long;
    cGbad_order = 0 as libc::c_int as libc::c_long;
    cGremote_rejects = 0 as libc::c_int as libc::c_long;
    cGremote_duprrs = 0 as libc::c_int as libc::c_long;
    cGerror_level = 0 as libc::c_int as libc::c_long;
    cGexpect_bad_order = 0 as libc::c_int;
    i = iGrequest_packsize;
    iseg = -(1 as libc::c_int);
    while i > 0 as libc::c_int {
        iseg += 1;
        iseg;
        i >>= 1 as libc::c_int;
    }
    iseg -= 5 as libc::c_int;
    if iseg < 0 as libc::c_int || iseg > 7 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"Illegal packet size %d for '%c' protocol\0" as *const u8
                as *const libc::c_char,
            iGrequest_packsize,
            (*(*qdaemon).qproto).bname as libc::c_int,
        );
        iseg = 1 as libc::c_int;
    }
    if iGrequest_winsize <= 0 as libc::c_int || iGrequest_winsize > 7 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"Illegal window size %d for '%c' protocol\0" as *const u8
                as *const libc::c_char,
            iGrequest_winsize,
            (*(*qdaemon).qproto).bname as libc::c_int,
        );
        iGrequest_winsize = 7 as libc::c_int;
    }
    fgota = 0 as libc::c_int;
    fgotb = 0 as libc::c_int;
    let mut current_block_62: u64;
    i = 0 as libc::c_int;
    while i < cGstartup_retries {
        if fgota != 0 {
            if fgsend_control(qdaemon, 7 as libc::c_int, iGrequest_winsize) == 0 {
                return 0 as libc::c_int;
            }
            current_block_62 = 17281240262373992796;
        } else if fgexchange_init(
            qdaemon,
            7 as libc::c_int,
            iGrequest_winsize,
            &mut iGremote_winsize,
        ) == 0
        {
            current_block_62 = 7172762164747879670;
        } else {
            current_block_62 = 17281240262373992796;
        }
        match current_block_62 {
            17281240262373992796 => {
                fgota = 1 as libc::c_int;
                if fgotb != 0 {
                    if fgsend_control(qdaemon, 6 as libc::c_int, iseg) == 0 {
                        return 0 as libc::c_int;
                    }
                    current_block_62 = 11048769245176032998;
                } else if fgexchange_init(
                    qdaemon,
                    6 as libc::c_int,
                    iseg,
                    &mut iGremote_segsize,
                ) == 0
                {
                    current_block_62 = 7172762164747879670;
                } else {
                    current_block_62 = 11048769245176032998;
                }
                match current_block_62 {
                    7172762164747879670 => {}
                    _ => {
                        fgotb = 1 as libc::c_int;
                        if !(fgexchange_init(
                            qdaemon,
                            5 as libc::c_int,
                            iGrequest_winsize,
                            &mut iGremote_winsize,
                        ) == 0)
                        {
                            iGremote_packsize = ((1 as libc::c_int)
                                << iGremote_segsize + 5 as libc::c_int) as size_t;
                            if iGforced_remote_winsize > 0 as libc::c_int
                                && iGforced_remote_winsize <= 7 as libc::c_int
                            {
                                iGremote_winsize = iGforced_remote_winsize;
                            }
                            if iGforced_remote_packsize >= 32 as libc::c_int
                                && iGforced_remote_packsize <= 4096 as libc::c_int
                            {
                                i = iGforced_remote_packsize;
                                iseg = -(1 as libc::c_int);
                                while i > 0 as libc::c_int {
                                    iseg += 1;
                                    iseg;
                                    i >>= 1 as libc::c_int;
                                }
                                iGremote_packsize = ((1 as libc::c_int) << iseg) as size_t;
                                iGremote_segsize = iseg - 5 as libc::c_int;
                            }
                            if fginit_sendbuffers(1 as libc::c_int) == 0 {
                                return 0 as libc::c_int;
                            }
                            *pzlog = zbufalc(
                                (::std::mem::size_of::<[libc::c_char; 48]>()
                                    as libc::c_ulong)
                                    .wrapping_add(64 as libc::c_int as libc::c_ulong),
                            );
                            sprintf(
                                *pzlog,
                                b"protocol '%c' sending packet/window %d/%d receiving %d/%d\0"
                                    as *const u8 as *const libc::c_char,
                                (*(*qdaemon).qproto).bname as libc::c_int,
                                iGremote_packsize as libc::c_int,
                                iGremote_winsize,
                                iGrequest_packsize,
                                iGrequest_winsize,
                            );
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fgstart: Protocol startup failed\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fbiggstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    fGshort_packets = 0 as libc::c_int;
    return fgstart(qdaemon, pzlog);
}
pub unsafe extern "C" fn fvstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    if iGrequest_packsize == 64 as libc::c_int {
        iGrequest_packsize = 1024 as libc::c_int;
    }
    return fgstart(qdaemon, pzlog);
}
unsafe extern "C" fn fgexchange_init(
    mut qdaemon: *mut sdaemon,
    mut ictl: libc::c_int,
    mut ival: libc::c_int,
    mut piset: *mut libc::c_int,
) -> boolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < cGexchange_init_retries {
        let mut itime: libc::c_long = 0;
        let mut ctimeout: libc::c_int = 0;
        if (*qdaemon).fcaller != 0 || i > 0 as libc::c_int {
            if fgsend_control(qdaemon, ictl, ival) == 0 {
                return 0 as libc::c_int;
            }
        }
        itime = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
        ctimeout = cGexchange_init_timeout;
        loop {
            let mut inewtime: libc::c_long = 0;
            if fgwait_for_packet(qdaemon, 1 as libc::c_int, ctimeout, 0 as libc::c_int)
                == 0
            {
                break;
            }
            if iGpacket_control >> 6 as libc::c_int & 0o3 as libc::c_int
                == 0 as libc::c_int
            {
                if iGpacket_control >> 3 as libc::c_int & 0o7 as libc::c_int == ictl {
                    *piset = iGpacket_control & 0o7 as libc::c_int;
                    if (*qdaemon).fcaller == 0 && i == 0 as libc::c_int {
                        if fgsend_control(qdaemon, ictl, ival) == 0 {
                            return 0 as libc::c_int;
                        }
                    }
                    return 1 as libc::c_int;
                }
                if (iGpacket_control >> 3 as libc::c_int & 0o7 as libc::c_int) < ictl
                    && ictl != 7 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if iGpacket_control >> 3 as libc::c_int & 0o7 as libc::c_int
                    == 7 as libc::c_int && ictl == 5 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if iGpacket_control >> 3 as libc::c_int & 0o7 as libc::c_int
                    == 6 as libc::c_int && ictl == 5 as libc::c_int
                {
                    iGremote_segsize = iGpacket_control & 0o7 as libc::c_int;
                }
            }
            inewtime = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
            ctimeout = (ctimeout as libc::c_long - (inewtime - itime)) as libc::c_int;
            if !(ctimeout > 0 as libc::c_int) {
                break;
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fgshutdown(mut qdaemon: *mut sdaemon) -> boolean {
    fgsend_control(qdaemon, 1 as libc::c_int, 0 as libc::c_int);
    fgsend_control(qdaemon, 1 as libc::c_int, 0 as libc::c_int);
    fginit_sendbuffers(0 as libc::c_int);
    ulog(
        LOG_NORMAL,
        b"Protocol '%c' packets: sent %ld, resent %ld, received %ld\0" as *const u8
            as *const libc::c_char,
        (*(*qdaemon).qproto).bname as libc::c_int,
        cGsent_packets,
        cGresent_packets - cGdelayed_packets,
        cGrec_packets,
    );
    if cGbad_hdr != 0 as libc::c_int as libc::c_long
        || cGbad_checksum != 0 as libc::c_int as libc::c_long
        || cGbad_order != 0 as libc::c_int as libc::c_long
        || cGremote_rejects != 0 as libc::c_int as libc::c_long
        || cGremote_duprrs != 0 as libc::c_int as libc::c_long
    {
        ulog(
            LOG_NORMAL,
            b"Errors: header %ld, checksum %ld, order %ld, remote rejects %ld\0"
                as *const u8 as *const libc::c_char,
            cGbad_hdr,
            cGbad_checksum,
            cGbad_order,
            cGremote_duprrs + cGremote_rejects,
        );
    }
    iGrequest_winsize = 7 as libc::c_int;
    iGrequest_packsize = 64 as libc::c_int;
    cGstartup_retries = 8 as libc::c_int;
    cGexchange_init_timeout = 10 as libc::c_int;
    cGexchange_init_retries = 4 as libc::c_int;
    cGtimeout = 10 as libc::c_int;
    cGretries = 6 as libc::c_int;
    cGgarbage_data = 10000 as libc::c_int;
    cGmax_errors = 100 as libc::c_int;
    cGerror_decay = 10 as libc::c_int;
    iGforced_remote_winsize = 0 as libc::c_int;
    iGforced_remote_packsize = 0 as libc::c_int;
    fGshort_packets = 1 as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fgsendcmd(
    mut qdaemon: *mut sdaemon,
    mut z: *const libc::c_char,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
) -> boolean {
    let mut clen: size_t = 0;
    let mut fagain: boolean = 0;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fgsendcmd: Sending command \"%s\"\0" as *const u8 as *const libc::c_char,
            z,
        );
    }
    clen = strlen(z);
    loop {
        let mut zpacket: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cdummy: size_t = 0;
        zpacket = zggetspace(qdaemon, &mut cdummy);
        if clen < iGremote_packsize {
            let mut csize: size_t = 0;
            if iGremote_packsize <= 64 as libc::c_int as libc::c_ulong
                || fGshort_packets == 0
            {
                csize = iGremote_packsize;
            } else {
                csize = 32 as libc::c_int as size_t;
                while csize <= clen {
                    csize <<= 1 as libc::c_int;
                }
            }
            memcpy(zpacket as *mut libc::c_void, z as *const libc::c_void, clen);
            if csize > clen {
                bzero(
                    zpacket.offset(clen as isize) as *mut libc::c_void,
                    csize.wrapping_sub(clen),
                );
            }
            fagain = 0 as libc::c_int;
            if fgsenddata(
                qdaemon,
                zpacket,
                csize,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int as libc::c_long,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else {
            memcpy(
                zpacket as *mut libc::c_void,
                z as *const libc::c_void,
                iGremote_packsize,
            );
            z = z.offset(iGremote_packsize as isize);
            clen = (clen as libc::c_ulong).wrapping_sub(iGremote_packsize) as size_t
                as size_t;
            fagain = 1 as libc::c_int;
            if fgsenddata(
                qdaemon,
                zpacket,
                iGremote_packsize,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int as libc::c_long,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        if !(fagain != 0) {
            break;
        }
    }
    return 1 as libc::c_int;
}
static mut azGsendbuffers: [*mut libc::c_char; 8] = [0 as *const libc::c_char
    as *mut libc::c_char; 8];
unsafe extern "C" fn fginit_sendbuffers(mut fallocate: boolean) -> boolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int + 1 as libc::c_int {
        xfree(azGsendbuffers[i as usize] as pointer);
        if fallocate != 0 {
            azGsendbuffers[i
                as usize] = malloc(
                ((6 as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(iGremote_packsize),
            ) as *mut libc::c_char;
            if (azGsendbuffers[i as usize]).is_null() {
                return 0 as libc::c_int;
            }
            bzero(
                azGsendbuffers[i as usize] as *mut libc::c_void,
                ((6 as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(iGremote_packsize),
            );
        } else {
            azGsendbuffers[i as usize] = 0 as *mut libc::c_char;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zggetspace(
    mut qdaemon: *mut sdaemon,
    mut pclen: *mut size_t,
) -> *mut libc::c_char {
    *pclen = iGremote_packsize;
    return (azGsendbuffers[iGsendseq as usize])
        .offset(6 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize);
}
pub unsafe extern "C" fn fgsenddata(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
    mut ipos: libc::c_long,
) -> boolean {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itt: libc::c_int = 0;
    let mut iseg: libc::c_int = 0;
    let mut csize: size_t = 0;
    let mut iclr1: libc::c_int = 0;
    let mut iclr2: libc::c_int = 0;
    let mut icheck: libc::c_ushort = 0;
    itt = 2 as libc::c_int;
    csize = iGremote_packsize;
    iseg = iGremote_segsize + 1 as libc::c_int;
    if cdata > csize {
        ulog(
            LOG_FATAL,
            b"fgsend_packet: Packet size too large\0" as *const u8 as *const libc::c_char,
        );
    }
    iclr1 = -(1 as libc::c_int);
    iclr2 = -(2 as libc::c_int);
    if cdata < csize {
        if iGremote_packsize > 64 as libc::c_int as libc::c_ulong && fGshort_packets != 0
        {
            iseg = 1 as libc::c_int;
            csize = 32 as libc::c_int as size_t;
            while csize < cdata {
                csize <<= 1 as libc::c_int;
                iseg += 1;
                iseg;
            }
        }
        if csize != cdata {
            let mut cshort: size_t = 0;
            iclr2 = 0 as libc::c_int;
            itt = 3 as libc::c_int;
            cshort = csize.wrapping_sub(cdata);
            if cshort <= 127 as libc::c_int as libc::c_ulong {
                zdata = zdata.offset(-1);
                zdata;
                *zdata.offset(0 as libc::c_int as isize) = cshort as libc::c_char;
                *zdata
                    .offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
                if cshort > 1 as libc::c_int as libc::c_ulong {
                    bzero(
                        zdata.offset(cdata as isize).offset(1 as libc::c_int as isize)
                            as *mut libc::c_void,
                        cshort.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                }
            } else {
                zdata = zdata.offset(-(2 as libc::c_int as isize));
                *zdata
                    .offset(
                        0 as libc::c_int as isize,
                    ) = (0x80 as libc::c_int as libc::c_ulong
                    | cshort & 0x7f as libc::c_int as libc::c_ulong) as libc::c_char;
                *zdata
                    .offset(
                        1 as libc::c_int as isize,
                    ) = (cshort >> 7 as libc::c_int) as libc::c_char;
                bzero(
                    zdata.offset(cdata as isize).offset(2 as libc::c_int as isize)
                        as *mut libc::c_void,
                    cshort.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                );
                iclr1 = 0 as libc::c_int;
            }
        }
    }
    z = zdata.offset(-(6 as libc::c_int as isize));
    *z.offset(iclr1 as isize) = '\0' as i32 as libc::c_char;
    *z.offset(iclr2 as isize) = '\0' as i32 as libc::c_char;
    *z.offset(0 as libc::c_int as isize) = '\u{10}' as i32 as libc::c_char;
    *z.offset(1 as libc::c_int as isize) = iseg as libc::c_char;
    icheck = igchecksum(zdata, csize) as libc::c_ushort;
    while iGsendseq == iGremote_ack
        || iGsendseq + 8 as libc::c_int - iGremote_ack & 0o7 as libc::c_int
            > iGremote_winsize
    {
        if fgwait_for_packet(qdaemon, 1 as libc::c_int, cGtimeout, cGretries) == 0 {
            return 0 as libc::c_int;
        }
    }
    while iGrecseq + 8 as libc::c_int - iGlocal_ack & 0o7 as libc::c_int
        > 1 as libc::c_int
    {
        iGlocal_ack = iGlocal_ack + 1 as libc::c_int & 0o7 as libc::c_int;
        if fgsend_control(qdaemon, 4 as libc::c_int, iGlocal_ack) == 0 {
            return 0 as libc::c_int;
        }
    }
    iGlocal_ack = iGrecseq;
    *z
        .offset(
            4 as libc::c_int as isize,
        ) = (itt << 6 as libc::c_int | iGsendseq << 3 as libc::c_int | iGrecseq)
        as libc::c_char;
    iGsendseq = iGsendseq + 1 as libc::c_int & 0o7 as libc::c_int;
    icheck = (0xaaaa as libc::c_int
        - (icheck as libc::c_int
            ^ *z.offset(4 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        & 0xffff as libc::c_int) as libc::c_ushort;
    *z
        .offset(
            2 as libc::c_int as isize,
        ) = (icheck as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *z
        .offset(
            3 as libc::c_int as isize,
        ) = (icheck as libc::c_int >> 8 as libc::c_int) as libc::c_char;
    *z
        .offset(
            5 as libc::c_int as isize,
        ) = (*z.offset(1 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(2 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(3 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(4 as libc::c_int as isize) as libc::c_int) as libc::c_char;
    cGsent_packets += 1;
    cGsent_packets;
    if iGretransmit_seq != -(1 as libc::c_int) {
        cGdelayed_packets += 1;
        cGdelayed_packets;
        return 1 as libc::c_int;
    }
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fgsenddata: Sending packet %d (%d bytes)\0" as *const u8
                as *const libc::c_char,
            *z.offset(4 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
                & 0o7 as libc::c_int,
            cdata,
        );
    }
    return fsend_data(
        (*qdaemon).qconn,
        z,
        (6 as libc::c_int as libc::c_ulong).wrapping_add(csize),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn zgadjust_ack(mut iseq: libc::c_int) -> *mut libc::c_char {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut icheck: libc::c_ushort = 0;
    z = azGsendbuffers[iseq as usize];
    if *z as libc::c_int == '\0' as i32 {
        z = z.offset(1);
        z;
    }
    if *z as libc::c_int == '\0' as i32 {
        z = z.offset(1);
        z;
    }
    if *z.offset(4 as libc::c_int as isize) as libc::c_int & 0o7 as libc::c_int
        == iGrecseq
    {
        return z;
    }
    icheck = ((*z.offset(3 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | *z.offset(2 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        as libc::c_ushort;
    icheck = ((0xaaaa as libc::c_int - icheck as libc::c_int
        ^ *z.offset(4 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        & 0xffff as libc::c_int) as libc::c_ushort;
    *z
        .offset(
            4 as libc::c_int as isize,
        ) = (*z.offset(4 as libc::c_int as isize) as libc::c_int & !(0o7 as libc::c_int)
        | iGrecseq) as libc::c_char;
    icheck = (0xaaaa as libc::c_int
        - (icheck as libc::c_int
            ^ *z.offset(4 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        & 0xffff as libc::c_int) as libc::c_ushort;
    *z
        .offset(
            2 as libc::c_int as isize,
        ) = (icheck as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *z
        .offset(
            3 as libc::c_int as isize,
        ) = (icheck as libc::c_int >> 8 as libc::c_int) as libc::c_char;
    *z
        .offset(
            5 as libc::c_int as isize,
        ) = (*z.offset(1 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(2 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(3 as libc::c_int as isize) as libc::c_int
        ^ *z.offset(4 as libc::c_int as isize) as libc::c_int) as libc::c_char;
    return z;
}
unsafe extern "C" fn fgsend_control(
    mut qdaemon: *mut sdaemon,
    mut ixxx: libc::c_int,
    mut iyyy: libc::c_int,
) -> boolean {
    let mut ab: [libc::c_char; 6] = [0; 6];
    let mut ictl: libc::c_int = 0;
    let mut icheck: libc::c_ushort = 0;
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int
        || iDebug & 0o1 as libc::c_int != 0 as libc::c_int && ixxx != 4 as libc::c_int
    {
        ulog(
            LOG_DEBUG,
            b"fgsend_control: Sending control %s %d\0" as *const u8
                as *const libc::c_char,
            azGcontrol[ixxx as usize],
            iyyy,
        );
    }
    ab[0 as libc::c_int as usize] = '\u{10}' as i32 as libc::c_char;
    ab[1 as libc::c_int as usize] = 9 as libc::c_int as libc::c_char;
    ictl = (0 as libc::c_int) << 6 as libc::c_int | ixxx << 3 as libc::c_int | iyyy;
    icheck = (0xaaaa as libc::c_int - ictl) as libc::c_ushort;
    ab[2 as libc::c_int
        as usize] = (icheck as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    ab[3 as libc::c_int
        as usize] = (icheck as libc::c_int >> 8 as libc::c_int) as libc::c_char;
    ab[4 as libc::c_int as usize] = ictl as libc::c_char;
    ab[5 as libc::c_int
        as usize] = (ab[1 as libc::c_int as usize] as libc::c_int
        ^ ab[2 as libc::c_int as usize] as libc::c_int
        ^ ab[3 as libc::c_int as usize] as libc::c_int
        ^ ab[4 as libc::c_int as usize] as libc::c_int) as libc::c_char;
    return fsend_data(
        (*qdaemon).qconn,
        ab.as_mut_ptr(),
        6 as libc::c_int as size_t,
        1 as libc::c_int,
    );
}
pub unsafe extern "C" fn fgwait(mut qdaemon: *mut sdaemon) -> boolean {
    return fgwait_for_packet(qdaemon, 0 as libc::c_int, cGtimeout, cGretries);
}
unsafe extern "C" fn fgwait_for_packet(
    mut qdaemon: *mut sdaemon,
    mut freturncontrol: boolean,
    mut ctimeout: libc::c_int,
    mut cretries: libc::c_int,
) -> boolean {
    let mut ctimeouts: libc::c_int = 0;
    let mut cgarbage: libc::c_int = 0;
    let mut cshort: libc::c_int = 0;
    ctimeouts = 0 as libc::c_int;
    cgarbage = 0 as libc::c_int;
    cshort = 0 as libc::c_int;
    loop {
        let mut fexit: boolean = 0;
        let mut cneed: size_t = 0;
        let mut ffound: boolean = 0;
        let mut crec: size_t = 0;
        if fgprocess_data(
            qdaemon,
            1 as libc::c_int,
            freturncontrol,
            &mut fexit,
            &mut cneed,
            &mut ffound,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if fexit != 0 {
            return 1 as libc::c_int;
        }
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fgwait_for_packet: Need %lu bytes\0" as *const u8
                    as *const libc::c_char,
                cneed,
            );
        }
        if ffound != 0 {
            ctimeouts = 0 as libc::c_int;
            cgarbage = 0 as libc::c_int;
        } else if cgarbage > cGgarbage_data {
            ulog(
                LOG_ERROR,
                b"Too much unrecognized data\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        if freceive_data((*qdaemon).qconn, cneed, &mut crec, ctimeout, 1 as libc::c_int)
            == 0
        {
            return 0 as libc::c_int;
        }
        cgarbage = (cgarbage as libc::c_ulong).wrapping_add(crec) as libc::c_int
            as libc::c_int;
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
                return 0 as libc::c_int;
            }
            if iGremote_ack + 1 as libc::c_int & 0o7 as libc::c_int != iGsendseq {
                let mut inext: libc::c_int = 0;
                let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
                inext = iGremote_ack + 1 as libc::c_int & 0o7 as libc::c_int;
                if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                    != 0 as libc::c_int
                {
                    ulog(
                        LOG_DEBUG,
                        b"fgwait_for_packet: Resending packet %d\0" as *const u8
                            as *const libc::c_char,
                        inext,
                    );
                }
                cGresent_packets += 1;
                cGresent_packets;
                zsend = zgadjust_ack(inext);
                if fsend_data(
                    (*qdaemon).qconn,
                    zsend,
                    (6 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            ((1 as libc::c_int)
                                << *zsend.offset(1 as libc::c_int as isize) as libc::c_int
                                    + 4 as libc::c_int) as size_t,
                        ),
                    1 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                iGretransmit_seq = inext;
            } else {
                if iGlocal_ack != iGrecseq {
                    if fgsend_acks(qdaemon) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                if fgsend_control(qdaemon, 2 as libc::c_int, iGrecseq) == 0 {
                    return 0 as libc::c_int;
                }
            }
        }
    };
}
unsafe extern "C" fn fgsend_acks(mut qdaemon: *mut sdaemon) -> boolean {
    while iGlocal_ack != iGrecseq {
        iGlocal_ack = iGlocal_ack + 1 as libc::c_int & 0o7 as libc::c_int;
        if fgsend_control(qdaemon, 4 as libc::c_int, iGlocal_ack) == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fggot_ack(
    mut qdaemon: *mut sdaemon,
    mut iack: libc::c_int,
) -> boolean {
    let mut inext: libc::c_int = 0;
    let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
    if cGerror_level > 0 as libc::c_int as libc::c_long
        && iGretransmit_seq == -(1 as libc::c_int)
        && cGsent_packets % cGerror_decay as libc::c_long
            == 0 as libc::c_int as libc::c_long
    {
        cGerror_level -= 1;
        cGerror_level;
    }
    cGexpect_bad_order = 0 as libc::c_int;
    if iack < iGremote_ack {
        uwindow_acked(qdaemon, 0 as libc::c_int);
    }
    iGremote_ack = iack;
    if iGretransmit_seq == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    inext = iGretransmit_seq + 1 as libc::c_int & 0o7 as libc::c_int;
    if inext == iGsendseq {
        iGretransmit_seq = -(1 as libc::c_int);
    } else {
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fggot_ack: Sending packet %d\0" as *const u8 as *const libc::c_char,
                inext,
            );
        }
        cGresent_packets += 1;
        cGresent_packets;
        zsend = zgadjust_ack(inext);
        if fsend_data(
            (*qdaemon).qconn,
            zsend,
            (6 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    ((1 as libc::c_int)
                        << *zsend.offset(1 as libc::c_int as isize) as libc::c_int
                            + 4 as libc::c_int) as size_t,
                ),
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        inext = inext + 1 as libc::c_int & 0o7 as libc::c_int;
        if inext == iGsendseq {
            iGretransmit_seq = -(1 as libc::c_int);
        } else {
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fggot_ack: Sending packet %d\0" as *const u8
                        as *const libc::c_char,
                    inext,
                );
            }
            cGresent_packets += 1;
            cGresent_packets;
            zsend = zgadjust_ack(inext);
            if fsend_data(
                (*qdaemon).qconn,
                zsend,
                (6 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        ((1 as libc::c_int)
                            << *zsend.offset(1 as libc::c_int as isize) as libc::c_int
                                + 4 as libc::c_int) as size_t,
                    ),
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            iGretransmit_seq = inext;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fgcheck_errors(mut qdaemon: *mut sdaemon) -> boolean {
    if cGerror_level > cGmax_errors as libc::c_long && cGmax_errors >= 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"Too many '%c' protocol errors\0" as *const u8 as *const libc::c_char,
            (*(*qdaemon).qproto).bname as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fgprocess_data(
    mut qdaemon: *mut sdaemon,
    mut fdoacks: boolean,
    mut freturncontrol: boolean,
    mut pfexit: *mut boolean,
    mut pcneed: *mut size_t,
    mut pffound: *mut boolean,
) -> boolean {
    *pfexit = 0 as libc::c_int;
    if !pffound.is_null() {
        *pffound = 0 as libc::c_int;
    }
    while iPrecstart != iPrecend {
        let mut ab: [libc::c_char; 6] = [0; 6];
        let mut i: libc::c_int = 0;
        let mut iget: libc::c_int = 0;
        let mut cwant: libc::c_int = 0;
        let mut ihdrcheck: libc::c_ushort = 0;
        let mut idatcheck: libc::c_ushort = 0;
        let mut zfirst: *const libc::c_char = 0 as *const libc::c_char;
        let mut zsecond: *const libc::c_char = 0 as *const libc::c_char;
        let mut cfirst: libc::c_int = 0;
        let mut csecond: libc::c_int = 0;
        let mut fduprr: boolean = 0;
        if abPrecbuf[iPrecstart as usize] as libc::c_int != '\u{10}' as i32 {
            let mut zdle: *mut libc::c_char = 0 as *mut libc::c_char;
            cfirst = iPrecend - iPrecstart;
            if cfirst < 0 as libc::c_int {
                cfirst = 16384 as libc::c_int - iPrecstart;
            }
            zdle = memchr(
                abPrecbuf.as_mut_ptr().offset(iPrecstart as isize)
                    as *const libc::c_void,
                '\u{10}' as i32,
                cfirst as size_t,
            ) as *mut libc::c_char;
            if zdle.is_null() {
                iPrecstart = (iPrecstart + cfirst) % 16384 as libc::c_int;
                continue;
            } else {
                iPrecstart = (iPrecstart as libc::c_long
                    + zdle
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
        if ab[0 as libc::c_int as usize] as libc::c_int != '\u{10}' as i32
            || (ab[1 as libc::c_int as usize] as libc::c_int) < 1 as libc::c_int
            || ab[1 as libc::c_int as usize] as libc::c_int > 9 as libc::c_int
            || ab[5 as libc::c_int as usize] as libc::c_int
                != ab[1 as libc::c_int as usize] as libc::c_int
                    ^ ab[2 as libc::c_int as usize] as libc::c_int
                    ^ ab[3 as libc::c_int as usize] as libc::c_int
                    ^ ab[4 as libc::c_int as usize] as libc::c_int
            || ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                & 0o3 as libc::c_int == 1 as libc::c_int
        {
            cGbad_hdr += 1;
            cGbad_hdr;
            cGerror_level += 1;
            cGerror_level;
            if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fgprocess_data: Bad header: K %d TT %d XOR byte %d calc %d\0"
                        as *const u8 as *const libc::c_char,
                    ab[1 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int,
                    ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                        & 0o3 as libc::c_int,
                    ab[5 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int,
                    (ab[1 as libc::c_int as usize] as libc::c_int
                        ^ ab[2 as libc::c_int as usize] as libc::c_int
                        ^ ab[3 as libc::c_int as usize] as libc::c_int
                        ^ ab[4 as libc::c_int as usize] as libc::c_int)
                        & 0xff as libc::c_int,
                );
            }
            if fgcheck_errors(qdaemon) == 0 {
                return 0 as libc::c_int;
            }
            iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            zfirst = abPrecbuf
                .as_mut_ptr()
                .offset(iPrecstart as isize)
                .offset(6 as libc::c_int as isize);
            cfirst = 0 as libc::c_int;
            zsecond = 0 as *const libc::c_char;
            csecond = 0 as libc::c_int;
            if ab[1 as libc::c_int as usize] as libc::c_int == 9 as libc::c_int {
                if ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                    & 0o3 as libc::c_int != 0 as libc::c_int
                {
                    cGbad_hdr += 1;
                    cGbad_hdr;
                    cGerror_level += 1;
                    cGerror_level;
                    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fgprocess_data: Bad header: control packet with data\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if fgcheck_errors(qdaemon) == 0 {
                        return 0 as libc::c_int;
                    }
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                    continue;
                } else {
                    idatcheck = (0xaaaa as libc::c_int
                        - ab[4 as libc::c_int as usize] as libc::c_int)
                        as libc::c_ushort;
                    cwant = 0 as libc::c_int;
                }
            } else {
                let mut cinbuf: libc::c_int = 0;
                let mut icheck: libc::c_ushort = 0;
                if ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                    & 0o3 as libc::c_int == 0 as libc::c_int
                {
                    cGbad_hdr += 1;
                    cGbad_hdr;
                    cGerror_level += 1;
                    cGerror_level;
                    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fgprocess_data: Bad header: data packet is type CONTROL\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if fgcheck_errors(qdaemon) == 0 {
                        return 0 as libc::c_int;
                    }
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                    continue;
                } else {
                    cinbuf = iPrecend - iPrecstart;
                    if cinbuf < 0 as libc::c_int {
                        cinbuf += 16384 as libc::c_int;
                    }
                    cinbuf -= 6 as libc::c_int;
                    cwant = ((1 as libc::c_int)
                        << ab[1 as libc::c_int as usize] as libc::c_int
                            + 4 as libc::c_int) as size_t as libc::c_int;
                    if cinbuf < cwant {
                        if !pcneed.is_null() {
                            *pcneed = (cwant - cinbuf) as size_t;
                        }
                        return 1 as libc::c_int;
                    }
                    if iPrecend >= iPrecstart {
                        cfirst = cwant;
                    } else {
                        cfirst = 16384 as libc::c_int - (iPrecstart + 6 as libc::c_int);
                        if cfirst >= cwant {
                            cfirst = cwant;
                        } else if cfirst > 0 as libc::c_int {
                            zsecond = abPrecbuf.as_mut_ptr();
                            csecond = cwant - cfirst;
                        } else {
                            zfirst = abPrecbuf.as_mut_ptr().offset(-(cfirst as isize));
                            cfirst = cwant;
                        }
                    }
                    if csecond == 0 as libc::c_int {
                        icheck = igchecksum(zfirst, cfirst as size_t) as libc::c_ushort;
                    } else {
                        icheck = igchecksum2(
                            zfirst,
                            cfirst as size_t,
                            zsecond,
                            csecond as size_t,
                        ) as libc::c_ushort;
                    }
                    idatcheck = (0xaaaa as libc::c_int
                        - (icheck as libc::c_int
                            ^ ab[4 as libc::c_int as usize] as libc::c_int
                                & 0xff as libc::c_int) & 0xffff as libc::c_int)
                        as libc::c_ushort;
                }
            }
            ihdrcheck = ((ab[3 as libc::c_int as usize] as libc::c_int
                & 0xff as libc::c_int) << 8 as libc::c_int
                | ab[2 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int)
                as libc::c_ushort;
            if ihdrcheck as libc::c_int != idatcheck as libc::c_int {
                if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                    != 0 as libc::c_int
                {
                    ulog(
                        LOG_DEBUG,
                        b"fgprocess_data: Bad checksum: header 0x%x, data 0x%x\0"
                            as *const u8 as *const libc::c_char,
                        ihdrcheck as libc::c_int,
                        idatcheck as libc::c_int,
                    );
                }
                cGbad_checksum += 1;
                cGbad_checksum;
                cGerror_level += 1;
                cGerror_level;
                if fgcheck_errors(qdaemon) == 0 {
                    return 0 as libc::c_int;
                }
                if ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                    & 0o3 as libc::c_int != 0 as libc::c_int
                {
                    if iGrecseq != iGlocal_ack {
                        if fgsend_acks(qdaemon) == 0 {
                            return 0 as libc::c_int;
                        }
                    }
                    if ab[4 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                        & 0o7 as libc::c_int
                        == iGrecseq + 1 as libc::c_int & 0o7 as libc::c_int
                    {
                        if fgsend_control(qdaemon, 2 as libc::c_int, iGrecseq) == 0 {
                            return 0 as libc::c_int;
                        }
                        cGexpect_bad_order += iGrequest_winsize - 1 as libc::c_int;
                    }
                }
                iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
            } else {
                iPrecstart = (iPrecstart + cwant + 6 as libc::c_int)
                    % 16384 as libc::c_int;
                iGpacket_control = ab[4 as libc::c_int as usize] as libc::c_int
                    & 0xff as libc::c_int;
                fduprr = 0 as libc::c_int;
                if cGremote_rejects == 0 as libc::c_int as libc::c_long
                    && ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                        & 0o3 as libc::c_int == 0 as libc::c_int
                    && ab[4 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                        & 0o7 as libc::c_int == 4 as libc::c_int
                    && iGremote_ack
                        == ab[4 as libc::c_int as usize] as libc::c_int
                            & 0o7 as libc::c_int
                    && iGremote_ack + 1 as libc::c_int & 0o7 as libc::c_int != iGsendseq
                    && iGretransmit_seq == -(1 as libc::c_int)
                {
                    if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fgprocess_data: Treating duplicate RR as RJ\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    fduprr = 1 as libc::c_int;
                }
                if ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                    & 0o3 as libc::c_int != 0 as libc::c_int
                    && ab[4 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                        & 0o7 as libc::c_int
                        == iGrecseq + 1 as libc::c_int & 0o7 as libc::c_int
                    || ab[4 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                        & 0o7 as libc::c_int == 4 as libc::c_int && fduprr == 0
                {
                    if fggot_ack(
                        qdaemon,
                        ab[4 as libc::c_int as usize] as libc::c_int & 0o7 as libc::c_int,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
                if ab[4 as libc::c_int as usize] as libc::c_int >> 6 as libc::c_int
                    & 0o3 as libc::c_int != 0 as libc::c_int
                {
                    if ab[4 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int
                        & 0o7 as libc::c_int
                        != iGrecseq + 1 as libc::c_int & 0o7 as libc::c_int
                    {
                        if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                            != 0 as libc::c_int
                        {
                            ulog(
                                LOG_DEBUG,
                                b"fgprocess_data: Got packet %d; expected %d\0" as *const u8
                                    as *const libc::c_char,
                                ab[4 as libc::c_int as usize] as libc::c_int
                                    >> 3 as libc::c_int & 0o7 as libc::c_int,
                                iGrecseq + 1 as libc::c_int & 0o7 as libc::c_int,
                            );
                        }
                        if cGexpect_bad_order > 0 as libc::c_int {
                            cGexpect_bad_order -= 1;
                            cGexpect_bad_order;
                        } else {
                            cGbad_order += 1;
                            cGbad_order;
                            cGerror_level += 1;
                            cGerror_level;
                            if fgcheck_errors(qdaemon) == 0 {
                                return 0 as libc::c_int;
                            }
                        }
                    } else {
                        cGrec_packets += 1;
                        cGrec_packets;
                        if cGerror_level > 0 as libc::c_int as libc::c_long
                            && cGrec_packets % cGerror_decay as libc::c_long
                                == 0 as libc::c_int as libc::c_long
                        {
                            cGerror_level -= 1;
                            cGerror_level;
                        }
                        cGexpect_bad_order = 0 as libc::c_int;
                        iGrecseq = iGrecseq + 1 as libc::c_int & 0o7 as libc::c_int;
                        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                            ulog(
                                LOG_DEBUG,
                                b"fgprocess_data: Got packet %d\0" as *const u8
                                    as *const libc::c_char,
                                iGrecseq,
                            );
                        }
                        if !pffound.is_null() {
                            *pffound = 1 as libc::c_int;
                        }
                        if fdoacks != 0 {
                            if fgsend_acks(qdaemon) == 0 {
                                return 0 as libc::c_int;
                            }
                        }
                        if ab[4 as libc::c_int as usize] as libc::c_int
                            >> 6 as libc::c_int & 0o3 as libc::c_int == 3 as libc::c_int
                        {
                            let mut cshort: libc::c_int = 0;
                            let mut cmove: libc::c_int = 0;
                            if *zfirst.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0x80 as libc::c_int == 0 as libc::c_int
                            {
                                cshort = *zfirst.offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0xff as libc::c_int;
                                cmove = 1 as libc::c_int;
                            } else {
                                let mut cbyte2: libc::c_int = 0;
                                if cfirst > 1 as libc::c_int {
                                    cbyte2 = *zfirst.offset(1 as libc::c_int as isize)
                                        as libc::c_int & 0xff as libc::c_int;
                                } else {
                                    cbyte2 = *zsecond.offset(0 as libc::c_int as isize)
                                        as libc::c_int & 0xff as libc::c_int;
                                }
                                cshort = (*zfirst.offset(0 as libc::c_int as isize)
                                    as libc::c_int & 0x7f as libc::c_int)
                                    + (cbyte2 << 7 as libc::c_int);
                                cmove = 2 as libc::c_int;
                            }
                            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                ulog(
                                    LOG_DEBUG,
                                    b"fgprocess_data: Packet short by %d\0" as *const u8
                                        as *const libc::c_char,
                                    cshort,
                                );
                            }
                            if cfirst > cmove {
                                zfirst = zfirst.offset(cmove as isize);
                                cfirst -= cmove;
                            } else {
                                zfirst = zsecond.offset((cmove - cfirst) as isize);
                                cfirst = csecond - (cmove - cfirst);
                                csecond = 0 as libc::c_int;
                            }
                            cshort -= cmove;
                            if csecond >= cshort {
                                csecond -= cshort;
                            } else {
                                cfirst -= cshort - csecond;
                                csecond = 0 as libc::c_int;
                            }
                            if cfirst < 0 as libc::c_int {
                                cfirst = 0 as libc::c_int;
                            }
                        }
                        if fgot_data(
                            qdaemon,
                            zfirst,
                            cfirst as size_t,
                            zsecond,
                            csecond as size_t,
                            -(1 as libc::c_int),
                            -(1 as libc::c_int),
                            -(1 as libc::c_int) as libc::c_long,
                            (iGremote_ack + 1 as libc::c_int & 0o7 as libc::c_int
                                == iGsendseq) as libc::c_int,
                            pfexit,
                        ) == 0
                        {
                            return 0 as libc::c_int;
                        }
                        if *pfexit != 0 {
                            return 1 as libc::c_int;
                        }
                        if freturncontrol != 0 {
                            *pfexit = 1 as libc::c_int;
                            return 1 as libc::c_int;
                        }
                    }
                } else {
                    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int
                        || iDebug & 0o1 as libc::c_int != 0 as libc::c_int
                            && ab[4 as libc::c_int as usize] as libc::c_int
                                >> 3 as libc::c_int & 0o7 as libc::c_int != 4 as libc::c_int
                    {
                        ulog(
                            LOG_DEBUG,
                            b"fgprocess_data: Got control %s %d\0" as *const u8
                                as *const libc::c_char,
                            azGcontrol[(ab[4 as libc::c_int as usize] as libc::c_int
                                >> 3 as libc::c_int & 0o7 as libc::c_int) as usize],
                            ab[4 as libc::c_int as usize] as libc::c_int
                                & 0o7 as libc::c_int,
                        );
                    }
                    let mut current_block_214: u64;
                    match ab[4 as libc::c_int as usize] as libc::c_int
                        >> 3 as libc::c_int & 0o7 as libc::c_int
                    {
                        1 => {
                            if fLog_sighup != 0 {
                                ulog(
                                    LOG_ERROR,
                                    b"Received unexpected CLOSE packet\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fgsend_control(qdaemon, 1 as libc::c_int, 0 as libc::c_int);
                            }
                            return 0 as libc::c_int;
                        }
                        4 => {
                            if fduprr == 0 {
                                current_block_214 = 11844752514624976770;
                            } else {
                                current_block_214 = 2350172673136162805;
                            }
                        }
                        2 => {
                            current_block_214 = 2350172673136162805;
                        }
                        3 => {
                            if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                                != 0 as libc::c_int
                            {
                                ulog(
                                    LOG_DEBUG,
                                    b"fgprocess_data: Selective reject of %d\0" as *const u8
                                        as *const libc::c_char,
                                    ab[4 as libc::c_int as usize] as libc::c_int
                                        & 0o7 as libc::c_int,
                                );
                            }
                            let mut zpack_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            cGresent_packets += 1;
                            cGresent_packets;
                            cGremote_rejects += 1;
                            cGremote_rejects;
                            cGerror_level += 1;
                            cGerror_level;
                            zpack_0 = zgadjust_ack(
                                ab[4 as libc::c_int as usize] as libc::c_int
                                    & 0o7 as libc::c_int,
                            );
                            if fsend_data(
                                (*qdaemon).qconn,
                                zpack_0,
                                (6 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        ((1 as libc::c_int)
                                            << *zpack_0.offset(1 as libc::c_int as isize) as libc::c_int
                                                + 4 as libc::c_int) as size_t,
                                    ),
                                1 as libc::c_int,
                            ) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            current_block_214 = 11844752514624976770;
                        }
                        5 | 6 | 7 | _ => {
                            current_block_214 = 11844752514624976770;
                        }
                    }
                    match current_block_214 {
                        2350172673136162805 => {
                            iGremote_ack = ab[4 as libc::c_int as usize] as libc::c_int
                                & 0o7 as libc::c_int;
                            iGretransmit_seq = iGremote_ack + 1 as libc::c_int
                                & 0o7 as libc::c_int;
                            if iGretransmit_seq == iGsendseq {
                                iGretransmit_seq = -(1 as libc::c_int);
                            } else {
                                let mut zpack: *mut libc::c_char = 0 as *mut libc::c_char;
                                if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                                    != 0 as libc::c_int
                                {
                                    ulog(
                                        LOG_DEBUG,
                                        b"fgprocess_data: Remote reject: next %d resending %d\0"
                                            as *const u8 as *const libc::c_char,
                                        iGsendseq,
                                        iGretransmit_seq,
                                    );
                                }
                                cGresent_packets += 1;
                                cGresent_packets;
                                if fduprr != 0 {
                                    cGremote_duprrs += 1;
                                    cGremote_duprrs;
                                } else {
                                    cGremote_rejects += 1;
                                    cGremote_rejects;
                                }
                                cGerror_level += 1;
                                cGerror_level;
                                if fgcheck_errors(qdaemon) == 0 {
                                    return 0 as libc::c_int;
                                }
                                zpack = zgadjust_ack(iGretransmit_seq);
                                if fsend_data(
                                    (*qdaemon).qconn,
                                    zpack,
                                    (6 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(
                                            ((1 as libc::c_int)
                                                << *zpack.offset(1 as libc::c_int as isize) as libc::c_int
                                                    + 4 as libc::c_int) as size_t,
                                        ),
                                    1 as libc::c_int,
                                ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                        _ => {}
                    }
                    if freturncontrol != 0 {
                        *pfexit = 1 as libc::c_int;
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    if !pcneed.is_null() {
        *pcneed = 6 as libc::c_int as size_t;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn igchecksum(
    mut z: *const libc::c_char,
    mut c: size_t,
) -> libc::c_int {
    let mut ichk1: libc::c_ulong = 0;
    let mut ichk2: libc::c_ulong = 0;
    ichk1 = 0xffff as libc::c_int as libc::c_ulong;
    ichk2 = 0 as libc::c_int as libc::c_ulong;
    loop {
        let mut b: libc::c_uint = 0;
        ichk1 = ichk1
            .wrapping_add(
                ichk1
                    .wrapping_add(
                        (ichk1 & 0x8000 as libc::c_int as libc::c_ulong)
                            >> 15 as libc::c_int,
                    ),
            );
        let fresh0 = z;
        z = z.offset(1);
        b = *fresh0 as libc::c_uchar as libc::c_uint;
        if b != 0 as libc::c_int as libc::c_uint {
            ichk1 &= 0xffff as libc::c_int as libc::c_ulong;
            ichk1 = ichk1.wrapping_add(b as libc::c_ulong);
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            if ichk1 >> 16 as libc::c_int != 0 as libc::c_int as libc::c_ulong {
                ichk1 ^= ichk2;
            }
        } else {
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            ichk1 ^= ichk2;
        }
        c = c.wrapping_sub(1);
        c;
        ichk1 = ichk1
            .wrapping_add(
                ichk1
                    .wrapping_add(
                        (ichk1 & 0x8000 as libc::c_int as libc::c_ulong)
                            >> 15 as libc::c_int,
                    ),
            );
        let fresh1 = z;
        z = z.offset(1);
        b = *fresh1 as libc::c_uchar as libc::c_uint;
        if b != 0 as libc::c_int as libc::c_uint {
            ichk1 &= 0xffff as libc::c_int as libc::c_ulong;
            ichk1 = ichk1.wrapping_add(b as libc::c_ulong);
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            if ichk1 >> 16 as libc::c_int != 0 as libc::c_int as libc::c_ulong {
                ichk1 ^= ichk2;
            }
        } else {
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            ichk1 ^= ichk2;
        }
        c = c.wrapping_sub(1);
        c;
        ichk1 = ichk1
            .wrapping_add(
                ichk1
                    .wrapping_add(
                        (ichk1 & 0x8000 as libc::c_int as libc::c_ulong)
                            >> 15 as libc::c_int,
                    ),
            );
        let fresh2 = z;
        z = z.offset(1);
        b = *fresh2 as libc::c_uchar as libc::c_uint;
        if b != 0 as libc::c_int as libc::c_uint {
            ichk1 &= 0xffff as libc::c_int as libc::c_ulong;
            ichk1 = ichk1.wrapping_add(b as libc::c_ulong);
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            if ichk1 >> 16 as libc::c_int != 0 as libc::c_int as libc::c_ulong {
                ichk1 ^= ichk2;
            }
        } else {
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            ichk1 ^= ichk2;
        }
        c = c.wrapping_sub(1);
        c;
        ichk1 = ichk1
            .wrapping_add(
                ichk1
                    .wrapping_add(
                        (ichk1 & 0x8000 as libc::c_int as libc::c_ulong)
                            >> 15 as libc::c_int,
                    ),
            );
        let fresh3 = z;
        z = z.offset(1);
        b = *fresh3 as libc::c_uchar as libc::c_uint;
        if b != 0 as libc::c_int as libc::c_uint {
            ichk1 &= 0xffff as libc::c_int as libc::c_ulong;
            ichk1 = ichk1.wrapping_add(b as libc::c_ulong);
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            if ichk1 >> 16 as libc::c_int != 0 as libc::c_int as libc::c_ulong {
                ichk1 ^= ichk2;
            }
        } else {
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            ichk1 ^= ichk2;
        }
        c = c.wrapping_sub(1);
        c;
        if !(c > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    return (ichk1 & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn igchecksum2(
    mut zfirst: *const libc::c_char,
    mut cfirst: size_t,
    mut zsecond: *const libc::c_char,
    mut csecond: size_t,
) -> libc::c_int {
    let mut ichk1: libc::c_ulong = 0;
    let mut ichk2: libc::c_ulong = 0;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: size_t = 0;
    z = zfirst;
    c = cfirst.wrapping_add(csecond);
    ichk1 = 0xffff as libc::c_int as libc::c_ulong;
    ichk2 = 0 as libc::c_int as libc::c_ulong;
    loop {
        let mut b: libc::c_uint = 0;
        ichk1 = ichk1
            .wrapping_add(
                ichk1
                    .wrapping_add(
                        (ichk1 & 0x8000 as libc::c_int as libc::c_ulong)
                            >> 15 as libc::c_int,
                    ),
            );
        let fresh4 = z;
        z = z.offset(1);
        b = *fresh4 as libc::c_uchar as libc::c_uint;
        if b != 0 as libc::c_int as libc::c_uint {
            ichk1 &= 0xffff as libc::c_int as libc::c_ulong;
            ichk1 = ichk1.wrapping_add(b as libc::c_ulong);
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            if ichk1 >> 16 as libc::c_int != 0 as libc::c_int as libc::c_ulong {
                ichk1 ^= ichk2;
            }
        } else {
            ichk2 = ichk2.wrapping_add(ichk1 ^ c);
            ichk1 ^= ichk2;
        }
        c = c.wrapping_sub(1);
        c;
        cfirst = cfirst.wrapping_sub(1);
        cfirst;
        if cfirst == 0 as libc::c_int as libc::c_ulong {
            z = zsecond;
        }
        if !(c > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    return (ichk1 & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int;
}
