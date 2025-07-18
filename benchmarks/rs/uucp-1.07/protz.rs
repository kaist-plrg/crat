use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    fn fconn_set(
        qconn: *mut sconnection,
        tparity: tparitysetting,
        tstrip: tstripsetting,
        txonxoff: txonxoffsetting,
    ) -> boolean;
    fn fqueue_send(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
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
    fn fsend_data(
        qconn: *mut sconnection,
        zsend: *const libc::c_char,
        csend: size_t,
        fdoread: boolean,
    ) -> boolean;
    fn breceive_char(
        qconn: *mut sconnection,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> libc::c_int;
    static mut abPrecbuf: [libc::c_char; 16384];
    static mut iPrecstart: libc::c_int;
    static mut iPrecend: libc::c_int;
    static mut qTsend: *mut stransfer;
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
pub type achdrval_t = [libc::c_uchar; 4];
pub type hdrval_t = libc::c_ulong;
pub type winpos_t = libc::c_ulong;
pub static mut protz_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: protz.c,v 1.11 2002/02/08 10:35:52 ian Rel $\0")
};
static mut cZtimeout: libc::c_int = 10 as libc::c_int;
static mut cZretries: libc::c_int = 10 as libc::c_int;
static mut cZstartup_retries: libc::c_int = 4 as libc::c_int;
static mut cZmax_garbage: libc::c_int = 2400 as libc::c_int;
static mut cZtx_window: libc::c_int = 16384 as libc::c_int;
static mut cZrx_buf_len: libc::c_int = 0 as libc::c_int;
static mut fZesc_ctl: boolean = 0 as libc::c_int;
pub static mut asZproto_params: [uuconf_cmdtab; 7] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cZtimeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cZretries as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"startup-retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cZstartup_retries as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"garbage\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cZmax_garbage as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"send-window\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cZtx_window as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"escape-control\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x12 as libc::c_int,
                uuconf_pvar: &fZesc_ctl as *const boolean as *mut boolean as pointer,
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
static mut cZheaders_sent: libc::c_ulong = 0;
static mut cZheaders_received: libc::c_ulong = 0;
static mut cZbytes_resent: libc::c_ulong = 0;
static mut cZtimeouts: libc::c_ulong = 0;
static mut cZerrors: libc::c_ulong = 0;
static mut zZtx_buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zZtx_packet_buf: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut zZrx_packet_buf: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut cZblklen: libc::c_uint = 0;
static mut cZtxwspac: libc::c_uint = 0;
static mut cZtxwcnt: libc::c_uint = 0;
static mut cZrxwcnt: libc::c_uint = 0;
static mut wpZtxstart: winpos_t = 0;
static mut wpZtxpos: winpos_t = 0;
static mut wpZlastsync: winpos_t = 0;
static mut wpZlrxpos: winpos_t = 0;
static mut wpZrxpos: winpos_t = 0;
static mut iZlast_tx_data_packet: libc::c_int = 0;
static mut iZjunk_count: libc::c_int = 0;
static mut iZtleft: libc::c_int = 0;
static mut iZbeenhereb4: libc::c_int = 0;
static mut wpZrxbytes: winpos_t = 0;
static mut iZlast_rx_data_packet: libc::c_int = 0;
static mut xon: libc::c_char = 0o21 as libc::c_int as libc::c_char;
static mut iZpkt_rcvd_kludge: libc::c_int = 0;
static mut hvZpkt_hdrval_kludge: hdrval_t = 0;
static mut azZframe_types: [*const libc::c_char; 12] = [
    b"Carrier Lost\0" as *const u8 as *const libc::c_char,
    b"Timeout\0" as *const u8 as *const libc::c_char,
    b"Error\0" as *const u8 as *const libc::c_char,
    b"ZINIT\0" as *const u8 as *const libc::c_char,
    b"ZDATA\0" as *const u8 as *const libc::c_char,
    b"ZRPOS\0" as *const u8 as *const libc::c_char,
    b"ZACK\0" as *const u8 as *const libc::c_char,
    b"ZNAK\0" as *const u8 as *const libc::c_char,
    b"Zreserved\0" as *const u8 as *const libc::c_char,
    b"ZINITEND\0" as *const u8 as *const libc::c_char,
    b"ZFIN\0" as *const u8 as *const libc::c_char,
    b"UNKNOWN!!!\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn fzstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    *pzlog = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong)
            .wrapping_add(100 as libc::c_int as libc::c_ulong),
    );
    sprintf(
        *pzlog,
        b"protocol 'a' starting: %d, %d, %d, %d, %d, %d\0" as *const u8
            as *const libc::c_char,
        cZtimeout,
        cZretries,
        cZstartup_retries,
        cZmax_garbage,
        cZtx_window,
        fZesc_ctl,
    );
    if fconn_set(
        (*qdaemon).qconn,
        PARITYSETTING_NONE,
        STRIPSETTING_EIGHTBITS,
        XONXOFF_OFF,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if cZtx_window % 1024 as libc::c_int != 0 as libc::c_int
        || cZtx_window < 4096 as libc::c_int || cZtx_window > 65536 as libc::c_int
        || 65536 as libc::c_int % cZtx_window != 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fzstart: cZtx_window not one of 4096, 8192, 16384, 32768, 65536\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    zZtx_buf = xmalloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
    zZtx_packet_buf = xmalloc(
        (12 as libc::c_int + 2 as libc::c_int * 1024 as libc::c_int + 10 as libc::c_int
            + 42 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    zZrx_packet_buf = xmalloc(
        (12 as libc::c_int + 2 as libc::c_int * 1024 as libc::c_int + 10 as libc::c_int
            + 42 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    iZlast_tx_data_packet = -(1 as libc::c_int);
    iZlast_rx_data_packet = -(1 as libc::c_int);
    wpZrxbytes = 0 as libc::c_int as winpos_t;
    wpZrxpos = wpZrxbytes;
    wpZlrxpos = wpZrxpos;
    wpZtxpos = wpZlrxpos;
    cZtxwspac = (cZtx_window / 4 as libc::c_int) as libc::c_uint;
    cZbytes_resent = 0 as libc::c_int as libc::c_ulong;
    cZheaders_received = cZbytes_resent;
    cZheaders_sent = cZheaders_received;
    cZerrors = 0 as libc::c_int as libc::c_ulong;
    cZtimeouts = cZerrors;
    iZpkt_rcvd_kludge = -(1 as libc::c_int);
    if fzstart_proto(qdaemon) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fzshutdown(mut qdaemon: *mut sdaemon) -> boolean {
    fzshutdown_proto(qdaemon);
    xfree(zZtx_buf as pointer);
    xfree(zZtx_packet_buf as pointer);
    xfree(zZrx_packet_buf as pointer);
    zZtx_buf = 0 as *mut libc::c_char;
    zZtx_packet_buf = 0 as *mut libc::c_char;
    zZrx_packet_buf = 0 as *mut libc::c_char;
    ulog(
        LOG_NORMAL,
        b"Protocol 'a' messages: sent %lu, received %lu\0" as *const u8
            as *const libc::c_char,
        cZheaders_sent,
        cZheaders_received,
    );
    ulog(
        LOG_NORMAL,
        b"Protocol 'a' packets: sent %lu, received %lu\0" as *const u8
            as *const libc::c_char,
        wpZtxpos.wrapping_div(1024 as libc::c_int as libc::c_ulong),
        wpZrxbytes.wrapping_div(1024 as libc::c_int as libc::c_ulong),
    );
    if cZbytes_resent != 0 as libc::c_int as libc::c_ulong
        || cZtimeouts != 0 as libc::c_int as libc::c_ulong
        || cZerrors != 0 as libc::c_int as libc::c_ulong
    {
        ulog(
            LOG_NORMAL,
            b"Protocol 'a' errors: bytes resent %lu, timeouts %lu, errors %lu\0"
                as *const u8 as *const libc::c_char,
            cZbytes_resent,
            cZtimeouts,
            cZerrors,
        );
    }
    cZtimeout = 10 as libc::c_int;
    cZretries = 10 as libc::c_int;
    cZstartup_retries = 4 as libc::c_int;
    cZmax_garbage = 2400 as libc::c_int;
    cZtx_window = 16384 as libc::c_int;
    fZesc_ctl = 0 as libc::c_int;
    cZbytes_resent = 0 as libc::c_int as libc::c_ulong;
    cZheaders_received = cZbytes_resent;
    cZheaders_sent = cZheaders_received;
    cZerrors = 0 as libc::c_int as libc::c_ulong;
    cZtimeouts = cZerrors;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fzsendcmd(
    mut qdaemon: *mut sdaemon,
    mut z: *const libc::c_char,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
) -> boolean {
    let mut n: size_t = 0;
    let mut clen: size_t = 0;
    let mut lredo: libc::c_long = 0;
    let mut zbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    clen = (strlen(z)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fzsendcmd: sending command %s\0" as *const u8 as *const libc::c_char,
            z,
        );
    }
    if fzstart_tx() == 0 {
        return 0 as libc::c_int;
    }
    zbuf = zzgetspace(qdaemon, &mut n);
    if zbuf.is_null() {
        return 0 as libc::c_int;
    }
    if clen > n {
        ulog(LOG_FATAL, b"fzsendcmd: clen > n\0" as *const u8 as *const libc::c_char);
    }
    strcpy(zbuf, z);
    loop {
        if fzsend_data(qdaemon, zbuf, clen, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        if fzfinish_tx(qdaemon, &mut lredo) == 0 {
            return 0 as libc::c_int;
        }
        if !(lredo >= 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return fzprocess(qdaemon);
}
pub unsafe extern "C" fn zzgetspace(
    mut qdaemon: *mut sdaemon,
    mut pclen: *mut size_t,
) -> *mut libc::c_char {
    *pclen = cZblklen as size_t;
    return zZtx_buf;
}
pub unsafe extern "C" fn fzsenddata(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
    mut ipos: libc::c_long,
) -> boolean {
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fzsenddata: %d bytes\0" as *const u8 as *const libc::c_char,
            cdata,
        );
    }
    if fzsend_data(
        qdaemon,
        zdata,
        cdata,
        (cdata == 0 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return fzprocess(qdaemon);
}
unsafe extern "C" fn fzsend_data(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut fendofmessage: boolean,
) -> boolean {
    let mut n: size_t = 0;
    if iZlast_tx_data_packet == -(1 as libc::c_int)
        || iZlast_tx_data_packet == 'k' as i32
    {
        cZrxwcnt = 0 as libc::c_int as libc::c_uint;
        cZtxwcnt = cZrxwcnt;
        iZjunk_count = 0 as libc::c_int;
        if fzsend_hdr(
            qdaemon,
            'A' as i32,
            1 as libc::c_int,
            hvzencode_data_hdr(wpZtxpos),
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    n = cdata;
    if fendofmessage != 0 {
        iZlast_tx_data_packet = 'l' as i32;
    } else if iZjunk_count > 3 as libc::c_int {
        iZlast_tx_data_packet = 'k' as i32;
    } else if wpZtxpos == wpZlastsync {
        iZlast_tx_data_packet = 'k' as i32;
    } else if cZrx_buf_len != 0
        && {
            cZrxwcnt = (cZrxwcnt as libc::c_ulong).wrapping_add(n) as libc::c_uint
                as libc::c_uint;
            cZrxwcnt as libc::c_ulong >= cZrx_buf_len as size_t
        }
    {
        iZlast_tx_data_packet = 'k' as i32;
    } else {
        cZtxwcnt = (cZtxwcnt as libc::c_ulong).wrapping_add(n) as libc::c_uint
            as libc::c_uint;
        if cZtxwcnt >= cZtxwspac {
            iZlast_tx_data_packet = 'j' as i32;
            cZtxwcnt = 0 as libc::c_int as libc::c_uint;
        } else {
            iZlast_tx_data_packet = 'i' as i32;
        }
    }
    iZtleft += 1;
    if iZtleft > 3 as libc::c_int {
        iZtleft = 0 as libc::c_int;
        if cZblklen < 1024 as libc::c_int as libc::c_uint {
            cZblklen = cZblklen.wrapping_mul(2 as libc::c_int as libc::c_uint);
        }
        if cZblklen > 1024 as libc::c_int as libc::c_uint {
            cZblklen = 1024 as libc::c_int as libc::c_uint;
        }
        if cZrx_buf_len != 0 && cZblklen as libc::c_ulong > cZrx_buf_len as size_t {
            cZblklen = cZrx_buf_len as libc::c_uint;
        }
    }
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
        match iZlast_tx_data_packet {
            107 => {
                type_0 = b"ZCRCW\0" as *const u8 as *const libc::c_char;
            }
            105 => {
                type_0 = b"ZCRCG\0" as *const u8 as *const libc::c_char;
            }
            106 => {
                type_0 = b"ZCRCQ\0" as *const u8 as *const libc::c_char;
            }
            104 => {
                type_0 = b"ZCRCE\0" as *const u8 as *const libc::c_char;
            }
            108 => {
                type_0 = b"ZCRCF\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                type_0 = b"UNKNOWN!!!\0" as *const u8 as *const libc::c_char;
            }
        }
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fzsend_data: %s, pos 0x%lx, %d bytes\0" as *const u8
                    as *const libc::c_char,
                type_0,
                wpZtxpos,
                n,
            );
        }
    }
    if fzsend_data_packet(qdaemon, zdata, n, iZlast_tx_data_packet, 1 as libc::c_int)
        == 0
    {
        return 0 as libc::c_int;
    }
    wpZtxpos = (wpZtxpos as libc::c_ulong).wrapping_add(n) as winpos_t as winpos_t;
    if iZlast_tx_data_packet == 'k' as i32 {
        match getinsync(qdaemon, 0 as libc::c_int) {
            3 => {}
            2 => {
                if qTsend.is_null() || ((*qTsend).e).is_null() {
                    ulog(
                        LOG_ERROR,
                        b"Can't reset non-file\0" as *const u8 as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
                iZlast_tx_data_packet = -(1 as libc::c_int);
                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fzsend_data: Seeking to %ld\0" as *const u8
                            as *const libc::c_char,
                        wpZrxpos.wrapping_sub(wpZtxstart) as libc::c_long,
                    );
                }
                if !(fseek(
                    (*qTsend).e,
                    wpZrxpos.wrapping_sub(wpZtxstart) as libc::c_long,
                    0 as libc::c_int,
                ) == 0 as libc::c_int)
                {
                    ulog(
                        LOG_ERROR,
                        b"seek: %s\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                    return 0 as libc::c_int;
                }
            }
            _ => return 0 as libc::c_int,
        }
        return 1 as libc::c_int;
    }
    while wpZtxpos.wrapping_sub(wpZlrxpos)
        >= (cZtx_window as size_t).wrapping_sub(2048 as libc::c_int as libc::c_ulong)
    {
        if iZlast_tx_data_packet != 'j' as i32 {
            iZlast_tx_data_packet = 'j' as i32;
            if fzsend_data_packet(
                qdaemon,
                zdata,
                0 as libc::c_int as size_t,
                iZlast_tx_data_packet,
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        match getinsync(qdaemon, 1 as libc::c_int) {
            3 => {}
            2 => {
                if qTsend.is_null() || ((*qTsend).e).is_null() {
                    ulog(
                        LOG_ERROR,
                        b"Can't reset non-file\0" as *const u8 as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
                iZlast_tx_data_packet = -(1 as libc::c_int);
                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fzsend_data: Seeking to %ld\0" as *const u8
                            as *const libc::c_char,
                        wpZrxpos.wrapping_sub(wpZtxstart) as libc::c_long,
                    );
                }
                if !(fseek(
                    (*qTsend).e,
                    wpZrxpos.wrapping_sub(wpZtxstart) as libc::c_long,
                    0 as libc::c_int,
                ) == 0 as libc::c_int)
                {
                    ulog(
                        LOG_ERROR,
                        b"seek: %s\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                    return 0 as libc::c_int;
                }
            }
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fzprocess(mut qdaemon: *mut sdaemon) -> boolean {
    let mut c: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    while fzreceive_ready() != 0 {
        if iPrecstart != iPrecend {
            ch = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
            iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            ch = realreadchar(qdaemon, 1 as libc::c_int);
        };
        match ch {
            42 => {
                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fzprocess: possible ZRPOS packet\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                iPrecstart = (iPrecstart + 16384 as libc::c_int - 1 as libc::c_int)
                    % 16384 as libc::c_int;
                c = getinsync(qdaemon, 1 as libc::c_int);
                if !(c == 3 as libc::c_int) {
                    if c == 2 as libc::c_int {
                        if qTsend.is_null() || ((*qTsend).e).is_null() {
                            ulog(
                                LOG_ERROR,
                                b"Attempt to back up non-file\0" as *const u8
                                    as *const libc::c_char,
                            );
                            return 0 as libc::c_int;
                        }
                        if !(fseek(
                            (*qTsend).e,
                            wpZrxpos.wrapping_sub(wpZtxstart) as libc::c_long,
                            0 as libc::c_int,
                        ) == 0 as libc::c_int)
                        {
                            ulog(
                                LOG_ERROR,
                                b"seek: %s\0" as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                            return 0 as libc::c_int;
                        }
                        iZlast_tx_data_packet = -(1 as libc::c_int);
                    } else {
                        return 0 as libc::c_int
                    }
                }
            }
            19 | 147 => {
                if iPrecstart != iPrecend {
                    ch = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                } else {
                    ch = realreadchar(qdaemon, 10 as libc::c_int);
                };
            }
            13 => {}
            _ => {
                iZjunk_count += 1;
                iZjunk_count;
            }
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fzwait(mut qdaemon: *mut sdaemon) -> boolean {
    let mut c: libc::c_int = 0;
    let mut cerr: libc::c_int = 0;
    let mut rxcount: libc::c_int = 0;
    let mut fexit: boolean = 0;
    let mut rx_hdr: achdrval_t = [0; 4];
    if fzstart_rx() == 0 {
        return 0 as libc::c_int;
    }
    cerr = cZretries;
    '_nxthdr: loop {
        c = izrecv_hdr(qdaemon, rx_hdr.as_mut_ptr());
        match c {
            -2 | 4 => {
                cerr -= 1;
                if cerr < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"fzwait: retries exhausted\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
            }
            -1 => {
                cerr -= 1;
                if cerr < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"fzwait: retries exhausted\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
            }
            -3 | 7 => return 0 as libc::c_int,
            2 | 3 => {
                continue;
            }
            1 => {
                let mut rx_bytes: winpos_t = 0;
                zdecode_data_hdr(rclhdr(rx_hdr.as_mut_ptr()), &mut rx_bytes);
                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"fzwait: bytes(us,them) 0x%lx,0x%lx\0" as *const u8
                            as *const libc::c_char,
                        wpZrxbytes,
                        rx_bytes,
                    );
                }
                if rx_bytes != wpZrxbytes {
                    cerr -= 1;
                    if cerr < 0 as libc::c_int {
                        ulog(
                            LOG_ERROR,
                            b"fzwait: retries exhausted\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 0 as libc::c_int;
                    }
                    zrdat32(qdaemon, zZrx_packet_buf, 1024 as libc::c_int, &mut rxcount);
                } else {
                    loop {
                        c = zrdat32(
                            qdaemon,
                            zZrx_packet_buf,
                            1024 as libc::c_int,
                            &mut rxcount,
                        );
                        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                            let mut msg: *const libc::c_char = 0 as *const libc::c_char;
                            if c < 0 as libc::c_int {
                                msg = azZframe_types[(if ((c + 3 as libc::c_int) as size_t)
                                    < (::std::mem::size_of::<[*const libc::c_char; 12]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                {
                                    (c + 3 as libc::c_int) as size_t
                                } else {
                                    (::std::mem::size_of::<[*const libc::c_char; 12]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                }) as usize];
                            } else {
                                match c {
                                    363 => {
                                        msg = b"ZCRCW\0" as *const u8 as *const libc::c_char;
                                    }
                                    361 => {
                                        msg = b"ZCRCG\0" as *const u8 as *const libc::c_char;
                                    }
                                    362 => {
                                        msg = b"ZCRCQ\0" as *const u8 as *const libc::c_char;
                                    }
                                    360 => {
                                        msg = b"ZCRCE\0" as *const u8 as *const libc::c_char;
                                    }
                                    364 => {
                                        msg = b"ZCRCF\0" as *const u8 as *const libc::c_char;
                                    }
                                    _ => {
                                        msg = 0 as *const libc::c_char;
                                    }
                                }
                            }
                            if !msg.is_null() {
                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                    ulog(
                                        LOG_DEBUG,
                                        b"fzwait: zrdat32: %s, %d bytes\0" as *const u8
                                            as *const libc::c_char,
                                        msg,
                                        rxcount,
                                    );
                                }
                            } else if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                ulog(
                                    LOG_DEBUG,
                                    b"fzwait: zrdat32: %d, %d bytes\0" as *const u8
                                        as *const libc::c_char,
                                    c,
                                    rxcount,
                                );
                            }
                        }
                        match c {
                            -1 => {
                                cZerrors = cZerrors.wrapping_add(1);
                                cZerrors;
                                cerr -= 1;
                                if cerr < 0 as libc::c_int {
                                    ulog(
                                        LOG_ERROR,
                                        b"fzwait: retries exhausted\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int;
                                }
                                break;
                            }
                            -2 => {
                                cZtimeouts = cZtimeouts.wrapping_add(1);
                                cZtimeouts;
                                cerr -= 1;
                                if cerr < 0 as libc::c_int {
                                    ulog(
                                        LOG_ERROR,
                                        b"fzwait: retries exhausted\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int;
                                }
                                break;
                            }
                            -3 => return 0 as libc::c_int,
                            363 => {
                                iZlast_rx_data_packet = 'k' as i32;
                                cerr = cZretries;
                                if rxcount != 0 as libc::c_int
                                    && fgot_data(
                                        qdaemon,
                                        zZrx_packet_buf,
                                        rxcount as size_t,
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        0 as libc::c_int as size_t,
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int) as libc::c_long,
                                        1 as libc::c_int,
                                        &mut fexit,
                                    ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                wpZrxbytes = (wpZrxbytes as libc::c_ulong)
                                    .wrapping_add(rxcount as libc::c_ulong) as winpos_t
                                    as winpos_t;
                                if fzsend_hdr(
                                    qdaemon,
                                    'B' as i32,
                                    3 as libc::c_int,
                                    hvzencode_data_hdr(wpZrxbytes),
                                    0 as libc::c_int,
                                ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                if fsend_data(
                                    (*qdaemon).qconn,
                                    &mut xon,
                                    1 as libc::c_int as size_t,
                                    0 as libc::c_int,
                                ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                continue '_nxthdr;
                            }
                            362 => {
                                iZlast_rx_data_packet = 'j' as i32;
                                cerr = cZretries;
                                if rxcount != 0 as libc::c_int
                                    && fgot_data(
                                        qdaemon,
                                        zZrx_packet_buf,
                                        rxcount as size_t,
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        0 as libc::c_int as size_t,
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int) as libc::c_long,
                                        1 as libc::c_int,
                                        &mut fexit,
                                    ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                wpZrxbytes = (wpZrxbytes as libc::c_ulong)
                                    .wrapping_add(rxcount as libc::c_ulong) as winpos_t
                                    as winpos_t;
                                if fzsend_hdr(
                                    qdaemon,
                                    'B' as i32,
                                    3 as libc::c_int,
                                    hvzencode_data_hdr(wpZrxbytes),
                                    0 as libc::c_int,
                                ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                            }
                            361 => {
                                iZlast_rx_data_packet = 'i' as i32;
                                cerr = cZretries;
                                if rxcount != 0 as libc::c_int
                                    && fgot_data(
                                        qdaemon,
                                        zZrx_packet_buf,
                                        rxcount as size_t,
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        0 as libc::c_int as size_t,
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int) as libc::c_long,
                                        1 as libc::c_int,
                                        &mut fexit,
                                    ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                wpZrxbytes = (wpZrxbytes as libc::c_ulong)
                                    .wrapping_add(rxcount as libc::c_ulong) as winpos_t
                                    as winpos_t;
                            }
                            360 => {
                                iZlast_rx_data_packet = 'h' as i32;
                                cerr = cZretries;
                                if rxcount != 0 as libc::c_int
                                    && fgot_data(
                                        qdaemon,
                                        zZrx_packet_buf,
                                        rxcount as size_t,
                                        0 as *mut libc::c_void as *const libc::c_char,
                                        0 as libc::c_int as size_t,
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int),
                                        -(1 as libc::c_int) as libc::c_long,
                                        1 as libc::c_int,
                                        &mut fexit,
                                    ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                wpZrxbytes = (wpZrxbytes as libc::c_ulong)
                                    .wrapping_add(rxcount as libc::c_ulong) as winpos_t
                                    as winpos_t;
                                continue '_nxthdr;
                            }
                            364 => {
                                iZlast_rx_data_packet = 'l' as i32;
                                cerr = cZretries;
                                wpZrxbytes = (wpZrxbytes as libc::c_ulong)
                                    .wrapping_add(rxcount as libc::c_ulong) as winpos_t
                                    as winpos_t;
                                if fzfinish_rx(qdaemon) == 0 {
                                    return 0 as libc::c_int;
                                }
                                if fgot_data(
                                    qdaemon,
                                    zZrx_packet_buf,
                                    rxcount as size_t,
                                    0 as *mut libc::c_void as *const libc::c_char,
                                    0 as libc::c_int as size_t,
                                    -(1 as libc::c_int),
                                    -(1 as libc::c_int),
                                    -(1 as libc::c_int) as libc::c_long,
                                    1 as libc::c_int,
                                    &mut fexit,
                                ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                return 1 as libc::c_int;
                            }
                            _ => return 0 as libc::c_int,
                        }
                    }
                }
            }
            _ => {
                ulog(
                    LOG_FATAL,
                    b"fzwait: received header %s\0" as *const u8 as *const libc::c_char,
                    azZframe_types[(if ((c + 3 as libc::c_int) as size_t)
                        < (::std::mem::size_of::<[*const libc::c_char; 12]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        (c + 3 as libc::c_int) as size_t
                    } else {
                        (::std::mem::size_of::<[*const libc::c_char; 12]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    }) as usize],
                );
                return 0 as libc::c_int;
            }
        }
        if fzsend_hdr(
            qdaemon,
            'B' as i32,
            2 as libc::c_int,
            hvzencode_data_hdr(wpZrxbytes),
            0 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn fzfile(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
    mut fstart: boolean,
    mut fsend: boolean,
    mut cbytes: libc::c_long,
    mut pfhandled: *mut boolean,
) -> boolean {
    let mut iredo: libc::c_long = 0;
    *pfhandled = 0 as libc::c_int;
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fzfile: fstart=%d, fsend=%d\0" as *const u8 as *const libc::c_char,
            fstart,
            fsend,
        );
    }
    if fsend != 0 {
        if fstart != 0 {
            return fzstart_tx();
        }
        if fzfinish_tx(qdaemon, &mut iredo) == 0 {
            return 0 as libc::c_int;
        }
        if iredo >= 0 as libc::c_int as libc::c_long {
            if ((*qtrans).e).is_null() {
                ulog(
                    LOG_ERROR,
                    b"Attempt to back up non-file\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            if !(fseek((*qtrans).e, iredo, 0 as libc::c_int) == 0 as libc::c_int) {
                ulog(
                    LOG_ERROR,
                    b"seek: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            *pfhandled = 1 as libc::c_int;
            (*qtrans).fsendfile = 1 as libc::c_int;
            return fqueue_send(qdaemon, qtrans);
        }
    }
    return 1 as libc::c_int;
}
static mut crc_32_tab: [libc::c_ulong; 256] = [
    0 as libc::c_long as libc::c_ulong,
    0x77073096 as libc::c_long as libc::c_ulong,
    0xee0e612c as libc::c_long as libc::c_ulong,
    0x990951ba as libc::c_long as libc::c_ulong,
    0x76dc419 as libc::c_long as libc::c_ulong,
    0x706af48f as libc::c_long as libc::c_ulong,
    0xe963a535 as libc::c_long as libc::c_ulong,
    0x9e6495a3 as libc::c_long as libc::c_ulong,
    0xedb8832 as libc::c_long as libc::c_ulong,
    0x79dcb8a4 as libc::c_long as libc::c_ulong,
    0xe0d5e91e as libc::c_long as libc::c_ulong,
    0x97d2d988 as libc::c_long as libc::c_ulong,
    0x9b64c2b as libc::c_long as libc::c_ulong,
    0x7eb17cbd as libc::c_long as libc::c_ulong,
    0xe7b82d07 as libc::c_long as libc::c_ulong,
    0x90bf1d91 as libc::c_long as libc::c_ulong,
    0x1db71064 as libc::c_long as libc::c_ulong,
    0x6ab020f2 as libc::c_long as libc::c_ulong,
    0xf3b97148 as libc::c_long as libc::c_ulong,
    0x84be41de as libc::c_long as libc::c_ulong,
    0x1adad47d as libc::c_long as libc::c_ulong,
    0x6ddde4eb as libc::c_long as libc::c_ulong,
    0xf4d4b551 as libc::c_long as libc::c_ulong,
    0x83d385c7 as libc::c_long as libc::c_ulong,
    0x136c9856 as libc::c_long as libc::c_ulong,
    0x646ba8c0 as libc::c_long as libc::c_ulong,
    0xfd62f97a as libc::c_long as libc::c_ulong,
    0x8a65c9ec as libc::c_long as libc::c_ulong,
    0x14015c4f as libc::c_long as libc::c_ulong,
    0x63066cd9 as libc::c_long as libc::c_ulong,
    0xfa0f3d63 as libc::c_long as libc::c_ulong,
    0x8d080df5 as libc::c_long as libc::c_ulong,
    0x3b6e20c8 as libc::c_long as libc::c_ulong,
    0x4c69105e as libc::c_long as libc::c_ulong,
    0xd56041e4 as libc::c_long as libc::c_ulong,
    0xa2677172 as libc::c_long as libc::c_ulong,
    0x3c03e4d1 as libc::c_long as libc::c_ulong,
    0x4b04d447 as libc::c_long as libc::c_ulong,
    0xd20d85fd as libc::c_long as libc::c_ulong,
    0xa50ab56b as libc::c_long as libc::c_ulong,
    0x35b5a8fa as libc::c_long as libc::c_ulong,
    0x42b2986c as libc::c_long as libc::c_ulong,
    0xdbbbc9d6 as libc::c_long as libc::c_ulong,
    0xacbcf940 as libc::c_long as libc::c_ulong,
    0x32d86ce3 as libc::c_long as libc::c_ulong,
    0x45df5c75 as libc::c_long as libc::c_ulong,
    0xdcd60dcf as libc::c_long as libc::c_ulong,
    0xabd13d59 as libc::c_long as libc::c_ulong,
    0x26d930ac as libc::c_long as libc::c_ulong,
    0x51de003a as libc::c_long as libc::c_ulong,
    0xc8d75180 as libc::c_long as libc::c_ulong,
    0xbfd06116 as libc::c_long as libc::c_ulong,
    0x21b4f4b5 as libc::c_long as libc::c_ulong,
    0x56b3c423 as libc::c_long as libc::c_ulong,
    0xcfba9599 as libc::c_long as libc::c_ulong,
    0xb8bda50f as libc::c_long as libc::c_ulong,
    0x2802b89e as libc::c_long as libc::c_ulong,
    0x5f058808 as libc::c_long as libc::c_ulong,
    0xc60cd9b2 as libc::c_long as libc::c_ulong,
    0xb10be924 as libc::c_long as libc::c_ulong,
    0x2f6f7c87 as libc::c_long as libc::c_ulong,
    0x58684c11 as libc::c_long as libc::c_ulong,
    0xc1611dab as libc::c_long as libc::c_ulong,
    0xb6662d3d as libc::c_long as libc::c_ulong,
    0x76dc4190 as libc::c_long as libc::c_ulong,
    0x1db7106 as libc::c_long as libc::c_ulong,
    0x98d220bc as libc::c_long as libc::c_ulong,
    0xefd5102a as libc::c_long as libc::c_ulong,
    0x71b18589 as libc::c_long as libc::c_ulong,
    0x6b6b51f as libc::c_long as libc::c_ulong,
    0x9fbfe4a5 as libc::c_long as libc::c_ulong,
    0xe8b8d433 as libc::c_long as libc::c_ulong,
    0x7807c9a2 as libc::c_long as libc::c_ulong,
    0xf00f934 as libc::c_long as libc::c_ulong,
    0x9609a88e as libc::c_long as libc::c_ulong,
    0xe10e9818 as libc::c_long as libc::c_ulong,
    0x7f6a0dbb as libc::c_long as libc::c_ulong,
    0x86d3d2d as libc::c_long as libc::c_ulong,
    0x91646c97 as libc::c_long as libc::c_ulong,
    0xe6635c01 as libc::c_long as libc::c_ulong,
    0x6b6b51f4 as libc::c_long as libc::c_ulong,
    0x1c6c6162 as libc::c_long as libc::c_ulong,
    0x856530d8 as libc::c_long as libc::c_ulong,
    0xf262004e as libc::c_long as libc::c_ulong,
    0x6c0695ed as libc::c_long as libc::c_ulong,
    0x1b01a57b as libc::c_long as libc::c_ulong,
    0x8208f4c1 as libc::c_long as libc::c_ulong,
    0xf50fc457 as libc::c_long as libc::c_ulong,
    0x65b0d9c6 as libc::c_long as libc::c_ulong,
    0x12b7e950 as libc::c_long as libc::c_ulong,
    0x8bbeb8ea as libc::c_long as libc::c_ulong,
    0xfcb9887c as libc::c_long as libc::c_ulong,
    0x62dd1ddf as libc::c_long as libc::c_ulong,
    0x15da2d49 as libc::c_long as libc::c_ulong,
    0x8cd37cf3 as libc::c_long as libc::c_ulong,
    0xfbd44c65 as libc::c_long as libc::c_ulong,
    0x4db26158 as libc::c_long as libc::c_ulong,
    0x3ab551ce as libc::c_long as libc::c_ulong,
    0xa3bc0074 as libc::c_long as libc::c_ulong,
    0xd4bb30e2 as libc::c_long as libc::c_ulong,
    0x4adfa541 as libc::c_long as libc::c_ulong,
    0x3dd895d7 as libc::c_long as libc::c_ulong,
    0xa4d1c46d as libc::c_long as libc::c_ulong,
    0xd3d6f4fb as libc::c_long as libc::c_ulong,
    0x4369e96a as libc::c_long as libc::c_ulong,
    0x346ed9fc as libc::c_long as libc::c_ulong,
    0xad678846 as libc::c_long as libc::c_ulong,
    0xda60b8d0 as libc::c_long as libc::c_ulong,
    0x44042d73 as libc::c_long as libc::c_ulong,
    0x33031de5 as libc::c_long as libc::c_ulong,
    0xaa0a4c5f as libc::c_long as libc::c_ulong,
    0xdd0d7cc9 as libc::c_long as libc::c_ulong,
    0x5005713c as libc::c_long as libc::c_ulong,
    0x270241aa as libc::c_long as libc::c_ulong,
    0xbe0b1010 as libc::c_long as libc::c_ulong,
    0xc90c2086 as libc::c_long as libc::c_ulong,
    0x5768b525 as libc::c_long as libc::c_ulong,
    0x206f85b3 as libc::c_long as libc::c_ulong,
    0xb966d409 as libc::c_long as libc::c_ulong,
    0xce61e49f as libc::c_long as libc::c_ulong,
    0x5edef90e as libc::c_long as libc::c_ulong,
    0x29d9c998 as libc::c_long as libc::c_ulong,
    0xb0d09822 as libc::c_long as libc::c_ulong,
    0xc7d7a8b4 as libc::c_long as libc::c_ulong,
    0x59b33d17 as libc::c_long as libc::c_ulong,
    0x2eb40d81 as libc::c_long as libc::c_ulong,
    0xb7bd5c3b as libc::c_long as libc::c_ulong,
    0xc0ba6cad as libc::c_long as libc::c_ulong,
    0xedb88320 as libc::c_long as libc::c_ulong,
    0x9abfb3b6 as libc::c_long as libc::c_ulong,
    0x3b6e20c as libc::c_long as libc::c_ulong,
    0x74b1d29a as libc::c_long as libc::c_ulong,
    0xead54739 as libc::c_long as libc::c_ulong,
    0x9dd277af as libc::c_long as libc::c_ulong,
    0x4db2615 as libc::c_long as libc::c_ulong,
    0x73dc1683 as libc::c_long as libc::c_ulong,
    0xe3630b12 as libc::c_long as libc::c_ulong,
    0x94643b84 as libc::c_long as libc::c_ulong,
    0xd6d6a3e as libc::c_long as libc::c_ulong,
    0x7a6a5aa8 as libc::c_long as libc::c_ulong,
    0xe40ecf0b as libc::c_long as libc::c_ulong,
    0x9309ff9d as libc::c_long as libc::c_ulong,
    0xa00ae27 as libc::c_long as libc::c_ulong,
    0x7d079eb1 as libc::c_long as libc::c_ulong,
    0xf00f9344 as libc::c_long as libc::c_ulong,
    0x8708a3d2 as libc::c_long as libc::c_ulong,
    0x1e01f268 as libc::c_long as libc::c_ulong,
    0x6906c2fe as libc::c_long as libc::c_ulong,
    0xf762575d as libc::c_long as libc::c_ulong,
    0x806567cb as libc::c_long as libc::c_ulong,
    0x196c3671 as libc::c_long as libc::c_ulong,
    0x6e6b06e7 as libc::c_long as libc::c_ulong,
    0xfed41b76 as libc::c_long as libc::c_ulong,
    0x89d32be0 as libc::c_long as libc::c_ulong,
    0x10da7a5a as libc::c_long as libc::c_ulong,
    0x67dd4acc as libc::c_long as libc::c_ulong,
    0xf9b9df6f as libc::c_long as libc::c_ulong,
    0x8ebeeff9 as libc::c_long as libc::c_ulong,
    0x17b7be43 as libc::c_long as libc::c_ulong,
    0x60b08ed5 as libc::c_long as libc::c_ulong,
    0xd6d6a3e8 as libc::c_long as libc::c_ulong,
    0xa1d1937e as libc::c_long as libc::c_ulong,
    0x38d8c2c4 as libc::c_long as libc::c_ulong,
    0x4fdff252 as libc::c_long as libc::c_ulong,
    0xd1bb67f1 as libc::c_long as libc::c_ulong,
    0xa6bc5767 as libc::c_long as libc::c_ulong,
    0x3fb506dd as libc::c_long as libc::c_ulong,
    0x48b2364b as libc::c_long as libc::c_ulong,
    0xd80d2bda as libc::c_long as libc::c_ulong,
    0xaf0a1b4c as libc::c_long as libc::c_ulong,
    0x36034af6 as libc::c_long as libc::c_ulong,
    0x41047a60 as libc::c_long as libc::c_ulong,
    0xdf60efc3 as libc::c_long as libc::c_ulong,
    0xa867df55 as libc::c_long as libc::c_ulong,
    0x316e8eef as libc::c_long as libc::c_ulong,
    0x4669be79 as libc::c_long as libc::c_ulong,
    0xcb61b38c as libc::c_long as libc::c_ulong,
    0xbc66831a as libc::c_long as libc::c_ulong,
    0x256fd2a0 as libc::c_long as libc::c_ulong,
    0x5268e236 as libc::c_long as libc::c_ulong,
    0xcc0c7795 as libc::c_long as libc::c_ulong,
    0xbb0b4703 as libc::c_long as libc::c_ulong,
    0x220216b9 as libc::c_long as libc::c_ulong,
    0x5505262f as libc::c_long as libc::c_ulong,
    0xc5ba3bbe as libc::c_long as libc::c_ulong,
    0xb2bd0b28 as libc::c_long as libc::c_ulong,
    0x2bb45a92 as libc::c_long as libc::c_ulong,
    0x5cb36a04 as libc::c_long as libc::c_ulong,
    0xc2d7ffa7 as libc::c_long as libc::c_ulong,
    0xb5d0cf31 as libc::c_long as libc::c_ulong,
    0x2cd99e8b as libc::c_long as libc::c_ulong,
    0x5bdeae1d as libc::c_long as libc::c_ulong,
    0x9b64c2b0 as libc::c_long as libc::c_ulong,
    0xec63f226 as libc::c_long as libc::c_ulong,
    0x756aa39c as libc::c_long as libc::c_ulong,
    0x26d930a as libc::c_long as libc::c_ulong,
    0x9c0906a9 as libc::c_long as libc::c_ulong,
    0xeb0e363f as libc::c_long as libc::c_ulong,
    0x72076785 as libc::c_long as libc::c_ulong,
    0x5005713 as libc::c_long as libc::c_ulong,
    0x95bf4a82 as libc::c_long as libc::c_ulong,
    0xe2b87a14 as libc::c_long as libc::c_ulong,
    0x7bb12bae as libc::c_long as libc::c_ulong,
    0xcb61b38 as libc::c_long as libc::c_ulong,
    0x92d28e9b as libc::c_long as libc::c_ulong,
    0xe5d5be0d as libc::c_long as libc::c_ulong,
    0x7cdcefb7 as libc::c_long as libc::c_ulong,
    0xbdbdf21 as libc::c_long as libc::c_ulong,
    0x86d3d2d4 as libc::c_long as libc::c_ulong,
    0xf1d4e242 as libc::c_long as libc::c_ulong,
    0x68ddb3f8 as libc::c_long as libc::c_ulong,
    0x1fda836e as libc::c_long as libc::c_ulong,
    0x81be16cd as libc::c_long as libc::c_ulong,
    0xf6b9265b as libc::c_long as libc::c_ulong,
    0x6fb077e1 as libc::c_long as libc::c_ulong,
    0x18b74777 as libc::c_long as libc::c_ulong,
    0x88085ae6 as libc::c_long as libc::c_ulong,
    0xff0f6a70 as libc::c_long as libc::c_ulong,
    0x66063bca as libc::c_long as libc::c_ulong,
    0x11010b5c as libc::c_long as libc::c_ulong,
    0x8f659eff as libc::c_long as libc::c_ulong,
    0xf862ae69 as libc::c_long as libc::c_ulong,
    0x616bffd3 as libc::c_long as libc::c_ulong,
    0x166ccf45 as libc::c_long as libc::c_ulong,
    0xa00ae278 as libc::c_long as libc::c_ulong,
    0xd70dd2ee as libc::c_long as libc::c_ulong,
    0x4e048354 as libc::c_long as libc::c_ulong,
    0x3903b3c2 as libc::c_long as libc::c_ulong,
    0xa7672661 as libc::c_long as libc::c_ulong,
    0xd06016f7 as libc::c_long as libc::c_ulong,
    0x4969474d as libc::c_long as libc::c_ulong,
    0x3e6e77db as libc::c_long as libc::c_ulong,
    0xaed16a4a as libc::c_long as libc::c_ulong,
    0xd9d65adc as libc::c_long as libc::c_ulong,
    0x40df0b66 as libc::c_long as libc::c_ulong,
    0x37d83bf0 as libc::c_long as libc::c_ulong,
    0xa9bcae53 as libc::c_long as libc::c_ulong,
    0xdebb9ec5 as libc::c_long as libc::c_ulong,
    0x47b2cf7f as libc::c_long as libc::c_ulong,
    0x30b5ffe9 as libc::c_long as libc::c_ulong,
    0xbdbdf21c as libc::c_long as libc::c_ulong,
    0xcabac28a as libc::c_long as libc::c_ulong,
    0x53b39330 as libc::c_long as libc::c_ulong,
    0x24b4a3a6 as libc::c_long as libc::c_ulong,
    0xbad03605 as libc::c_long as libc::c_ulong,
    0xcdd70693 as libc::c_long as libc::c_ulong,
    0x54de5729 as libc::c_long as libc::c_ulong,
    0x23d967bf as libc::c_long as libc::c_ulong,
    0xb3667a2e as libc::c_long as libc::c_ulong,
    0xc4614ab8 as libc::c_long as libc::c_ulong,
    0x5d681b02 as libc::c_long as libc::c_ulong,
    0x2a6f2b94 as libc::c_long as libc::c_ulong,
    0xb40bbe37 as libc::c_long as libc::c_ulong,
    0xc30c8ea1 as libc::c_long as libc::c_ulong,
    0x5a05df1b as libc::c_long as libc::c_ulong,
    0x2d02ef8d as libc::c_long as libc::c_ulong,
];
unsafe extern "C" fn fzstart_proto(mut qdaemon: *mut sdaemon) -> boolean {
    let mut i: libc::c_int = 0;
    let mut tx_hdr: achdrval_t = [0; 4];
    let mut rx_hdr: achdrval_t = [0; 4];
    let mut current_block_17: u64;
    i = 0 as libc::c_int;
    while i < cZstartup_retries {
        stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
        tx_hdr[3 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        if fZesc_ctl != 0 {
            tx_hdr[2 as libc::c_int
                as usize] = (tx_hdr[2 as libc::c_int as usize] as libc::c_int
                | 1 as libc::c_int) as libc::c_uchar;
        }
        match izexchange_init(
            qdaemon,
            0 as libc::c_int,
            tx_hdr.as_mut_ptr(),
            rx_hdr.as_mut_ptr(),
        ) {
            -1 => return 0 as libc::c_int,
            0 => {}
            1 | _ => {
                fZesc_ctl = (fZesc_ctl != 0
                    || rx_hdr[2 as libc::c_int as usize] as libc::c_int
                        & 1 as libc::c_int != 0 as libc::c_int) as libc::c_int;
                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                match izexchange_init(
                    qdaemon,
                    1 as libc::c_int,
                    tx_hdr.as_mut_ptr(),
                    rx_hdr.as_mut_ptr(),
                ) {
                    -1 => {
                        current_block_17 = 14103470621526068625;
                        match current_block_17 {
                            14103470621526068625 => return 0 as libc::c_int,
                            _ => {
                                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                                match izexchange_init(
                                    qdaemon,
                                    3 as libc::c_int,
                                    tx_hdr.as_mut_ptr(),
                                    rx_hdr.as_mut_ptr(),
                                ) {
                                    -1 => {
                                        current_block_17 = 5161946086944071447;
                                        match current_block_17 {
                                            5161946086944071447 => return 0 as libc::c_int,
                                            _ => {
                                                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                                                match izexchange_init(
                                                    qdaemon,
                                                    6 as libc::c_int,
                                                    tx_hdr.as_mut_ptr(),
                                                    rx_hdr.as_mut_ptr(),
                                                ) {
                                                    -1 => {
                                                        current_block_17 = 14140296040720354316;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
                                                            }
                                                        }
                                                    }
                                                    0 => {}
                                                    1 | _ => {
                                                        current_block_17 = 12124785117276362961;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    0 => {}
                                    1 | _ => {
                                        current_block_17 = 1054647088692577877;
                                        match current_block_17 {
                                            5161946086944071447 => return 0 as libc::c_int,
                                            _ => {
                                                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                                                match izexchange_init(
                                                    qdaemon,
                                                    6 as libc::c_int,
                                                    tx_hdr.as_mut_ptr(),
                                                    rx_hdr.as_mut_ptr(),
                                                ) {
                                                    -1 => {
                                                        current_block_17 = 14140296040720354316;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
                                                            }
                                                        }
                                                    }
                                                    0 => {}
                                                    1 | _ => {
                                                        current_block_17 = 12124785117276362961;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
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
                    0 => {}
                    1 | _ => {
                        current_block_17 = 1841672684692190573;
                        match current_block_17 {
                            14103470621526068625 => return 0 as libc::c_int,
                            _ => {
                                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                                match izexchange_init(
                                    qdaemon,
                                    3 as libc::c_int,
                                    tx_hdr.as_mut_ptr(),
                                    rx_hdr.as_mut_ptr(),
                                ) {
                                    -1 => {
                                        current_block_17 = 5161946086944071447;
                                        match current_block_17 {
                                            5161946086944071447 => return 0 as libc::c_int,
                                            _ => {
                                                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                                                match izexchange_init(
                                                    qdaemon,
                                                    6 as libc::c_int,
                                                    tx_hdr.as_mut_ptr(),
                                                    rx_hdr.as_mut_ptr(),
                                                ) {
                                                    -1 => {
                                                        current_block_17 = 14140296040720354316;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
                                                            }
                                                        }
                                                    }
                                                    0 => {}
                                                    1 | _ => {
                                                        current_block_17 = 12124785117276362961;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    0 => {}
                                    1 | _ => {
                                        current_block_17 = 1054647088692577877;
                                        match current_block_17 {
                                            5161946086944071447 => return 0 as libc::c_int,
                                            _ => {
                                                stohdr(0 as libc::c_long as hdrval_t, tx_hdr.as_mut_ptr());
                                                match izexchange_init(
                                                    qdaemon,
                                                    6 as libc::c_int,
                                                    tx_hdr.as_mut_ptr(),
                                                    rx_hdr.as_mut_ptr(),
                                                ) {
                                                    -1 => {
                                                        current_block_17 = 14140296040720354316;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
                                                            }
                                                        }
                                                    }
                                                    0 => {}
                                                    1 | _ => {
                                                        current_block_17 = 12124785117276362961;
                                                        match current_block_17 {
                                                            14140296040720354316 => return 0 as libc::c_int,
                                                            _ => {
                                                                if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                                                                    ulog(
                                                                        LOG_DEBUG,
                                                                        b"fzstart_proto: Protocol started\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int;
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
        i += 1;
        i;
    }
    ulog(LOG_ERROR, b"Protocol init failed\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn izexchange_init(
    mut qdaemon: *mut sdaemon,
    mut send_type: libc::c_int,
    mut send_val: *mut libc::c_uchar,
    mut recv_val: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut recv_type: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut current_block_15: u64;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if fzsend_hdr(
            qdaemon,
            if send_type == 1 as libc::c_int { 'A' as i32 } else { 'B' as i32 },
            send_type,
            rclhdr(send_val),
            0 as libc::c_int,
        ) == 0
        {
            return -(1 as libc::c_int);
        }
        if send_type == 1 as libc::c_int {
            count = czbuild_data_packet(
                zZtx_packet_buf,
                b"\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
                'l' as i32,
            );
            if fsend_data(
                (*qdaemon).qconn,
                zZtx_packet_buf,
                count as size_t,
                0 as libc::c_int,
            ) == 0
            {
                return -(1 as libc::c_int);
            }
        }
        recv_type = izrecv_hdr(qdaemon, recv_val);
        match recv_type {
            -3 | 7 => return -(1 as libc::c_int),
            0 | 3 | 6 => {
                current_block_15 = 3512920355445576850;
            }
            1 => {
                if zrdat32(qdaemon, zZrx_packet_buf, 1024 as libc::c_int, &mut count)
                    == 'l' as i32 | 0o400 as libc::c_int
                {
                    current_block_15 = 3512920355445576850;
                } else {
                    current_block_15 = 11174649648027449784;
                }
            }
            -2 | -1 | _ => {
                current_block_15 = 11174649648027449784;
            }
        }
        match current_block_15 {
            3512920355445576850 => {
                if recv_type == send_type {
                    return 1 as libc::c_int;
                }
                if recv_type > send_type && send_type != 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
                if recv_type == 0 as libc::c_int && send_type == 6 as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fzshutdown_proto(mut qdaemon: *mut sdaemon) -> boolean {
    fzsend_hdr(
        qdaemon,
        'B' as i32,
        7 as libc::c_int,
        0 as libc::c_long as hdrval_t,
        0 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn fzstart_tx() -> boolean {
    iZlast_tx_data_packet = -(1 as libc::c_int);
    cZblklen = 1024 as libc::c_int as libc::c_uint;
    wpZlastsync = -(1 as libc::c_long) as winpos_t;
    iZbeenhereb4 = 0 as libc::c_int;
    iZtleft = 0 as libc::c_int;
    iZjunk_count = 0 as libc::c_int;
    wpZtxpos = wpZtxpos.wrapping_add(1024 as libc::c_long as libc::c_ulong)
        & !(1023 as libc::c_long) as libc::c_ulong;
    wpZrxpos = wpZtxpos;
    wpZlrxpos = wpZrxpos;
    wpZtxstart = wpZtxpos;
    return 1 as libc::c_int;
}
unsafe extern "C" fn fzfinish_tx(
    mut qdaemon: *mut sdaemon,
    mut plredo: *mut libc::c_long,
) -> boolean {
    let mut c: libc::c_int = 0;
    let mut cerr: libc::c_int = 0;
    let mut ctimeouts: libc::c_int = 0;
    let mut rx_hdr: achdrval_t = [0; 4];
    let mut rx_bytes: winpos_t = 0;
    *plredo = -(1 as libc::c_int) as libc::c_long;
    cerr = cZretries;
    ctimeouts = 0 as libc::c_int;
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fzfinish_tx: txpos=0x%lx, rxpos=0x%lx, lrxpos=0x%lx, rxbytes=0x%lx\0"
                as *const u8 as *const libc::c_char,
            wpZtxpos,
            wpZrxpos,
            wpZlrxpos,
            wpZrxbytes,
        );
    }
    loop {
        c = izrecv_hdr(qdaemon, rx_hdr.as_mut_ptr());
        match c {
            2 => {
                wpZrxpos = lzupdate_rxpos(
                    rx_hdr.as_mut_ptr(),
                    wpZrxpos,
                    wpZlrxpos,
                    wpZtxpos,
                );
                if wpZtxpos.wrapping_add(1024 as libc::c_int as libc::c_ulong)
                    & !(1023 as libc::c_int) as libc::c_ulong == wpZrxpos
                {
                    return 1 as libc::c_int;
                }
                cZbytes_resent = cZbytes_resent
                    .wrapping_add(wpZtxpos.wrapping_sub(wpZrxpos));
                wpZtxpos = wpZrxpos;
                wpZlrxpos = wpZtxpos;
                if wpZlastsync == wpZrxpos {
                    iZbeenhereb4 += 1;
                    if iZbeenhereb4 > 4 as libc::c_int {
                        if cZblklen > 32 as libc::c_int as libc::c_uint {
                            cZblklen = cZblklen
                                .wrapping_div(2 as libc::c_int as libc::c_uint);
                        }
                    }
                }
                wpZlastsync = wpZrxpos;
                iZlast_tx_data_packet = 'k' as i32;
                *plredo = wpZrxpos.wrapping_sub(wpZtxstart) as libc::c_long;
                return 1 as libc::c_int;
            }
            3 => {
                wpZrxpos = lzupdate_rxpos(
                    rx_hdr.as_mut_ptr(),
                    wpZrxpos,
                    wpZlrxpos,
                    wpZtxpos,
                );
                wpZlrxpos = wpZrxpos;
                if wpZtxpos == wpZrxpos {
                    return 1 as libc::c_int;
                }
            }
            1 => {
                zdecode_data_hdr(rclhdr(rx_hdr.as_mut_ptr()), &mut rx_bytes);
                if wpZrxbytes.wrapping_add(1024 as libc::c_long as libc::c_ulong)
                    & !(1023 as libc::c_long) as libc::c_ulong == rx_bytes
                {
                    iZpkt_rcvd_kludge = 1 as libc::c_int;
                    hvZpkt_hdrval_kludge = rclhdr(rx_hdr.as_mut_ptr());
                    return 1 as libc::c_int;
                }
            }
            4 => {
                zdecode_data_hdr(rclhdr(rx_hdr.as_mut_ptr()), &mut rx_bytes);
                if wpZrxbytes.wrapping_add(1024 as libc::c_long as libc::c_ulong)
                    & !(1023 as libc::c_long) as libc::c_ulong == rx_bytes
                {
                    return 1 as libc::c_int;
                }
                if rx_bytes == wpZrxbytes {
                    if fzsend_hdr(
                        qdaemon,
                        'B' as i32,
                        3 as libc::c_int,
                        hvzencode_data_hdr(wpZrxbytes),
                        1 as libc::c_int,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
            7 | -3 => return 0 as libc::c_int,
            -2 => {
                cerr -= 1;
                if cerr < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"fzfinish_tx: retries exhausted\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
                ctimeouts += 1;
                if ctimeouts % 2 as libc::c_int == 0 as libc::c_int {
                    if fzsend_hdr(
                        qdaemon,
                        'B' as i32,
                        4 as libc::c_int,
                        hvzencode_data_hdr(wpZtxpos),
                        1 as libc::c_int,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
            -1 | _ => {
                cerr -= 1;
                if cerr < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"fzfinish_tx: retries exhausted\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
                if fzsend_hdr(
                    qdaemon,
                    'B' as i32,
                    4 as libc::c_int,
                    hvzencode_data_hdr(wpZtxpos),
                    1 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
        }
    };
}
unsafe extern "C" fn fzstart_rx() -> boolean {
    wpZrxbytes = wpZrxbytes.wrapping_add(1024 as libc::c_long as libc::c_ulong)
        & !(1023 as libc::c_long) as libc::c_ulong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn fzfinish_rx(mut qdaemon: *mut sdaemon) -> boolean {
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fzfinish_rx: message/file received\0" as *const u8 as *const libc::c_char,
        );
    }
    return fzsend_hdr(
        qdaemon,
        'B' as i32,
        3 as libc::c_int,
        hvzencode_data_hdr(wpZrxbytes),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn fzsend_hdr(
    mut qdaemon: *mut sdaemon,
    mut ipkttype: libc::c_int,
    mut ihdrtype: libc::c_int,
    mut hdrval: hdrval_t,
    mut fcheckreceive: boolean,
) -> boolean {
    let mut cpacketlen: libc::c_int = 0;
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fzsend_hdr: %s, data = 0x%lx\0" as *const u8 as *const libc::c_char,
            azZframe_types[(if ((ihdrtype + 3 as libc::c_int) as size_t)
                < (::std::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                (ihdrtype + 3 as libc::c_int) as size_t
            } else {
                (::std::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            }) as usize],
            hdrval,
        );
    }
    cpacketlen = czbuild_header(zZtx_packet_buf, ipkttype, ihdrtype, hdrval);
    cZheaders_sent = cZheaders_sent.wrapping_add(1);
    cZheaders_sent;
    return fsend_data(
        (*qdaemon).qconn,
        zZtx_packet_buf,
        cpacketlen as size_t,
        fcheckreceive,
    );
}
unsafe extern "C" fn fzsend_data_packet(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut frameend: libc::c_int,
    mut fcheckreceive: boolean,
) -> boolean {
    let mut cpacketlen: libc::c_int = 0;
    cpacketlen = czbuild_data_packet(zZtx_packet_buf, zdata, cdata, frameend);
    return fsend_data(
        (*qdaemon).qconn,
        zZtx_packet_buf,
        cpacketlen as size_t,
        fcheckreceive,
    );
}
unsafe extern "C" fn czbuild_header(
    mut zresult: *mut libc::c_char,
    mut ipkttype: libc::c_int,
    mut ihdrtype: libc::c_int,
    mut hdrval: hdrval_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut crc: libc::c_ulong = 0;
    let mut achdrval: achdrval_t = [0; 4];
    p = zresult;
    match ipkttype {
        65 => {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '*' as i32 as libc::c_char;
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = 0o30 as libc::c_int as libc::c_char;
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = 'A' as i32 as libc::c_char;
            p = zputchar(p, ihdrtype);
            crc = 0xffffffff as libc::c_ulong;
            crc = crc_32_tab[((crc as libc::c_uint ^ ihdrtype as libc::c_uint)
                & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
            stohdr(hdrval, achdrval.as_mut_ptr());
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                p = zputchar(p, achdrval[i as usize] as libc::c_int);
                crc = crc_32_tab[((crc as libc::c_uint
                    ^ achdrval[i as usize] as libc::c_uint)
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                    ^ crc >> 8 as libc::c_int
                        & 0xffffff as libc::c_long as libc::c_ulong;
                i += 1;
                i;
            }
            crc = !crc;
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                p = zputchar(p, crc as libc::c_char as libc::c_int);
                crc >>= 8 as libc::c_int;
                i += 1;
                i;
            }
        }
        66 => {
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = '*' as i32 as libc::c_char;
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = '*' as i32 as libc::c_char;
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = 0o30 as libc::c_int as libc::c_char;
            let fresh6 = p;
            p = p.offset(1);
            *fresh6 = 'B' as i32 as libc::c_char;
            p = zputhex(p, ihdrtype);
            crc = 0xffffffff as libc::c_ulong;
            crc = crc_32_tab[((crc as libc::c_uint ^ ihdrtype as libc::c_uint)
                & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
            stohdr(hdrval, achdrval.as_mut_ptr());
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                p = zputhex(p, achdrval[i as usize] as libc::c_int);
                crc = crc_32_tab[((crc as libc::c_uint
                    ^ achdrval[i as usize] as libc::c_uint)
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                    ^ crc >> 8 as libc::c_int
                        & 0xffffff as libc::c_long as libc::c_ulong;
                i += 1;
                i;
            }
            crc = !crc;
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                p = zputhex(p, crc as libc::c_char as libc::c_int);
                crc >>= 8 as libc::c_int;
                i += 1;
                i;
            }
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = 0o15 as libc::c_int as libc::c_char;
            if ihdrtype != 7 as libc::c_int && ihdrtype != 3 as libc::c_int {
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = 0o21 as libc::c_int as libc::c_char;
            }
        }
        _ => {
            ulog(
                LOG_FATAL,
                b"czbuild_header: ipkttype == %d\0" as *const u8 as *const libc::c_char,
                ipkttype,
            );
        }
    }
    return p.offset_from(zresult) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn czbuild_data_packet(
    mut zresult: *mut libc::c_char,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
    mut frameend: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crc: libc::c_ulong = 0;
    p = zresult;
    crc = 0xffffffff as libc::c_ulong;
    loop {
        let fresh9 = cdata;
        cdata = cdata.wrapping_sub(1);
        if !(fresh9 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut c: libc::c_char = 0;
        c = *zdata;
        if c as libc::c_int & 0o140 as libc::c_int != 0 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = c;
        } else {
            p = zputchar(p, c as libc::c_int);
        }
        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uchar as libc::c_uint)
            & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
        zdata = zdata.offset(1);
        zdata;
    }
    let fresh11 = p;
    p = p.offset(1);
    *fresh11 = 0o30 as libc::c_int as libc::c_char;
    let fresh12 = p;
    p = p.offset(1);
    *fresh12 = frameend as libc::c_char;
    crc = crc_32_tab[((crc as libc::c_uint ^ frameend as libc::c_uint)
        & 0xff as libc::c_int as libc::c_uint) as usize]
        ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
    crc = !crc;
    cdata = 0 as libc::c_int as size_t;
    while cdata < 4 as libc::c_int as libc::c_ulong {
        p = zputchar(p, crc as libc::c_char as libc::c_int);
        crc >>= 8 as libc::c_int;
        cdata = cdata.wrapping_add(1);
        cdata;
    }
    if frameend == 'k' as i32 || frameend == 'h' as i32 || frameend == 'l' as i32 {
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = 0o15 as libc::c_int as libc::c_char;
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = 0o21 as libc::c_int as libc::c_char;
    }
    return p.offset_from(zresult) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn izrecv_hdr(
    mut qdaemon: *mut sdaemon,
    mut hdr: *mut libc::c_uchar,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut cerr: libc::c_int = 0;
    if iZpkt_rcvd_kludge != -(1 as libc::c_int) {
        c = iZpkt_rcvd_kludge;
        iZpkt_rcvd_kludge = -(1 as libc::c_int);
        stohdr(hvZpkt_hdrval_kludge, hdr);
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"izrecv_hdr: queued %s, data = 0x%lx\0" as *const u8
                    as *const libc::c_char,
                azZframe_types[(if ((c + 3 as libc::c_int) as size_t)
                    < (::std::mem::size_of::<[*const libc::c_char; 12]>()
                        as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    (c + 3 as libc::c_int) as size_t
                } else {
                    (::std::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                }) as usize],
                rclhdr(hdr),
            );
        }
        cZheaders_received = cZheaders_received.wrapping_add(1);
        cZheaders_received;
        return c;
    }
    cerr = cZmax_garbage;
    '_again: loop {
        c = noxrd7(qdaemon);
        match c {
            -2 | -1 | -3 => {
                break;
            }
            42 => {
                loop {
                    c = noxrd7(qdaemon);
                    match c {
                        42 => {
                            cerr -= 1;
                            if !(cerr < 0 as libc::c_int) {
                                continue;
                            }
                            c = -(1 as libc::c_int);
                            break '_again;
                        }
                        -2 | -3 => {
                            break '_again;
                        }
                        24 => {
                            c = noxrd7(qdaemon);
                            match c {
                                -2 | -3 => {
                                    break '_again;
                                }
                                65 => {
                                    current_block = 4907125128173134938;
                                    break;
                                }
                                66 => {
                                    current_block = 16016268339543350935;
                                    break;
                                }
                                _ => {
                                    current_block = 11442764142639834802;
                                    break;
                                }
                            }
                        }
                        _ => {
                            cerr -= 1;
                            if !(cerr < 0 as libc::c_int) {
                                continue '_again;
                            }
                            c = -(1 as libc::c_int);
                            break '_again;
                        }
                    }
                }
                match current_block {
                    4907125128173134938 => {
                        c = zrbhdr32(qdaemon, hdr);
                        break;
                    }
                    16016268339543350935 => {
                        c = zrhhdr(qdaemon, hdr);
                        break;
                    }
                    _ => {
                        cerr -= 1;
                        if !(cerr < 0 as libc::c_int) {
                            continue;
                        }
                        c = -(1 as libc::c_int);
                        break;
                    }
                }
            }
            13 | _ => {
                cerr -= 1;
                if !(cerr < 0 as libc::c_int) {
                    continue;
                }
                c = -(1 as libc::c_int);
                break;
            }
        }
    }
    match c {
        -2 => {
            cZtimeouts = cZtimeouts.wrapping_add(1);
            cZtimeouts;
        }
        -1 => {
            cZerrors = cZerrors.wrapping_add(1);
            cZerrors;
        }
        -3 => {}
        _ => {
            cZheaders_received = cZheaders_received.wrapping_add(1);
            cZheaders_received;
        }
    }
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"izrecv_hdr: %s, data = 0x%lx\0" as *const u8 as *const libc::c_char,
            azZframe_types[(if ((c + 3 as libc::c_int) as size_t)
                < (::std::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                (c + 3 as libc::c_int) as size_t
            } else {
                (::std::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            }) as usize],
            rclhdr(hdr),
        );
    }
    return c;
}
unsafe extern "C" fn zrbhdr32(
    mut qdaemon: *mut sdaemon,
    mut hdr: *mut libc::c_uchar,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut crc: libc::c_ulong = 0;
    c = zdlread(qdaemon);
    if c & !(0o377 as libc::c_int) != 0 {
        return c;
    }
    type_0 = c;
    crc = 0xffffffff as libc::c_ulong;
    crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
        & 0xff as libc::c_int as libc::c_uint) as usize]
        ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        c = zdlread(qdaemon);
        if c & !(0o377 as libc::c_int) != 0 {
            return c;
        }
        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
            & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
        *hdr.offset(i as isize) = c as libc::c_char as libc::c_uchar;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        c = zdlread(qdaemon);
        if c & !(0o377 as libc::c_int) != 0 {
            return c;
        }
        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
            & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
        i += 1;
        i;
    }
    if crc != 0xdebb20e3 as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return type_0;
}
unsafe extern "C" fn zrhhdr(
    mut qdaemon: *mut sdaemon,
    mut hdr: *mut libc::c_uchar,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut crc: libc::c_ulong = 0;
    c = zgethex(qdaemon);
    if c < 0 as libc::c_int {
        return c;
    }
    type_0 = c;
    crc = 0xffffffff as libc::c_ulong;
    crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
        & 0xff as libc::c_int as libc::c_uint) as usize]
        ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        c = zgethex(qdaemon);
        if c < 0 as libc::c_int {
            return c;
        }
        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
            & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
        *hdr.offset(i as isize) = c as libc::c_char as libc::c_uchar;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        c = zgethex(qdaemon);
        if c < 0 as libc::c_int {
            return c;
        }
        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
            & 0xff as libc::c_int as libc::c_uint) as usize]
            ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
        i += 1;
        i;
    }
    if crc != 0xdebb20e3 as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return type_0;
}
unsafe extern "C" fn zrdat32(
    mut qdaemon: *mut sdaemon,
    mut buf: *mut libc::c_char,
    mut length: libc::c_int,
    mut iprxcount: *mut libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut crc: libc::c_ulong = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    crc = 0xffffffff as libc::c_ulong;
    *iprxcount = 0 as libc::c_int;
    end = buf.offset(length as isize);
    while buf <= end {
        c = zdlread(qdaemon);
        if c & !(0o377 as libc::c_int) != 0 {
            loop {
                match c {
                    360 | 361 | 362 | 363 | 364 => {
                        d = c;
                        c &= 0o377 as libc::c_int;
                        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
                            & 0xff as libc::c_int as libc::c_uint) as usize]
                            ^ crc >> 8 as libc::c_int
                                & 0xffffff as libc::c_long as libc::c_ulong;
                        c = zdlread(qdaemon);
                        if c & !(0o377 as libc::c_int) != 0 {
                            continue;
                        }
                        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
                            & 0xff as libc::c_int as libc::c_uint) as usize]
                            ^ crc >> 8 as libc::c_int
                                & 0xffffff as libc::c_long as libc::c_ulong;
                        c = zdlread(qdaemon);
                        if c & !(0o377 as libc::c_int) != 0 {
                            continue;
                        }
                        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
                            & 0xff as libc::c_int as libc::c_uint) as usize]
                            ^ crc >> 8 as libc::c_int
                                & 0xffffff as libc::c_long as libc::c_ulong;
                        c = zdlread(qdaemon);
                        if c & !(0o377 as libc::c_int) != 0 {
                            continue;
                        }
                        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
                            & 0xff as libc::c_int as libc::c_uint) as usize]
                            ^ crc >> 8 as libc::c_int
                                & 0xffffff as libc::c_long as libc::c_ulong;
                        c = zdlread(qdaemon);
                        if c & !(0o377 as libc::c_int) != 0 {
                            continue;
                        }
                        crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
                            & 0xff as libc::c_int as libc::c_uint) as usize]
                            ^ crc >> 8 as libc::c_int
                                & 0xffffff as libc::c_long as libc::c_ulong;
                        if crc != 0xdebb20e3 as libc::c_ulong {
                            return -(1 as libc::c_int);
                        }
                        *iprxcount = (length as libc::c_long
                            - end.offset_from(buf) as libc::c_long) as libc::c_int;
                        return d;
                    }
                    -2 | -3 => return c,
                    _ => return -(1 as libc::c_int),
                }
            }
        } else {
            let fresh15 = buf;
            buf = buf.offset(1);
            *fresh15 = c as libc::c_char;
            crc = crc_32_tab[((crc as libc::c_uint ^ c as libc::c_uint)
                & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crc >> 8 as libc::c_int & 0xffffff as libc::c_long as libc::c_ulong;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn getinsync(
    mut qdaemon: *mut sdaemon,
    mut flag: boolean,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut cerr: libc::c_int = 0;
    let mut rx_hdr: achdrval_t = [0; 4];
    cerr = cZretries;
    loop {
        c = izrecv_hdr(qdaemon, rx_hdr.as_mut_ptr());
        match c {
            2 => {
                wpZrxpos = lzupdate_rxpos(
                    rx_hdr.as_mut_ptr(),
                    wpZrxpos,
                    wpZlrxpos,
                    wpZtxpos,
                );
                cZbytes_resent = cZbytes_resent
                    .wrapping_add(wpZtxpos.wrapping_sub(wpZrxpos));
                wpZtxpos = wpZrxpos;
                wpZlrxpos = wpZtxpos;
                if wpZlastsync == wpZrxpos {
                    iZbeenhereb4 += 1;
                    if iZbeenhereb4 > 4 as libc::c_int {
                        if cZblklen > 32 as libc::c_int as libc::c_uint {
                            cZblklen = cZblklen
                                .wrapping_div(2 as libc::c_int as libc::c_uint);
                        }
                    }
                }
                wpZlastsync = wpZrxpos;
                return 2 as libc::c_int;
            }
            3 => {
                wpZrxpos = lzupdate_rxpos(
                    rx_hdr.as_mut_ptr(),
                    wpZrxpos,
                    wpZlrxpos,
                    wpZtxpos,
                );
                wpZlrxpos = wpZrxpos;
                if flag != 0 || wpZtxpos == wpZrxpos {
                    return 3 as libc::c_int;
                }
            }
            4 => {
                let mut rx_bytes: winpos_t = 0;
                zdecode_data_hdr(rclhdr(rx_hdr.as_mut_ptr()), &mut rx_bytes);
                if rx_bytes == wpZrxbytes {
                    if fzsend_hdr(
                        qdaemon,
                        'B' as i32,
                        3 as libc::c_int,
                        hvzencode_data_hdr(wpZrxbytes),
                        1 as libc::c_int,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
            }
            7 | -3 => return c,
            -2 => {
                cerr -= 1;
                if cerr < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"getinsync: retries exhausted\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            -1 | _ => {
                cerr -= 1;
                if cerr < 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"getinsync: retries exhausted\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if fzsend_hdr(
                    qdaemon,
                    'B' as i32,
                    4 as libc::c_int,
                    hvzencode_data_hdr(wpZtxpos),
                    1 as libc::c_int,
                ) == 0
                {
                    return -(1 as libc::c_int);
                }
            }
        }
    };
}
unsafe extern "C" fn zputhex(
    mut p: *mut libc::c_char,
    mut ch: libc::c_int,
) -> *mut libc::c_char {
    static mut digits: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"0123456789abcdef\0")
    };
    let fresh16 = p;
    p = p.offset(1);
    *fresh16 = digits[((ch & 0xf0 as libc::c_int) >> 4 as libc::c_int) as usize];
    let fresh17 = p;
    p = p.offset(1);
    *fresh17 = digits[(ch & 0xf as libc::c_int) as usize];
    return p;
}
unsafe extern "C" fn zputchar(
    mut p: *mut libc::c_char,
    mut ch: libc::c_int,
) -> *mut libc::c_char {
    let mut c: libc::c_char = ch as libc::c_char;
    if c as libc::c_int & 0o140 as libc::c_int != 0 {
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = c;
    } else {
        let mut current_block_11: u64;
        match c as libc::c_int & 0o377 as libc::c_int {
            24 => {
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 = 0o30 as libc::c_int as libc::c_char;
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = (c as libc::c_int ^ 0o100 as libc::c_int) as libc::c_char;
                current_block_11 = 12800627514080957624;
            }
            13 => {
                current_block_11 = 18248585479086678598;
            }
            16 | 17 | 19 => {
                current_block_11 = 18248585479086678598;
            }
            _ => {
                if fZesc_ctl != 0 && c as libc::c_int & 0o140 as libc::c_int == 0 {
                    let fresh23 = p;
                    p = p.offset(1);
                    *fresh23 = 0o30 as libc::c_int as libc::c_char;
                    c = (c as libc::c_int ^ 0o100 as libc::c_int) as libc::c_char;
                }
                let fresh24 = p;
                p = p.offset(1);
                *fresh24 = c;
                current_block_11 = 12800627514080957624;
            }
        }
        match current_block_11 {
            18248585479086678598 => {
                let fresh21 = p;
                p = p.offset(1);
                *fresh21 = 0o30 as libc::c_int as libc::c_char;
                c = (c as libc::c_int ^ 0o100 as libc::c_int) as libc::c_char;
                let fresh22 = p;
                p = p.offset(1);
                *fresh22 = c;
            }
            _ => {}
        }
    }
    return p;
}
unsafe extern "C" fn zgethex(mut qdaemon: *mut sdaemon) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    c = noxrd7(qdaemon);
    if c < 0 as libc::c_int {
        return c;
    }
    n = c - '0' as i32;
    if n > 9 as libc::c_int {
        n -= 'a' as i32 - ':' as i32;
    }
    if n & !(0xf as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    c = noxrd7(qdaemon);
    if c < 0 as libc::c_int {
        return c;
    }
    c -= '0' as i32;
    if c > 9 as libc::c_int {
        c -= 'a' as i32 - ':' as i32;
    }
    if c & !(0xf as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    c += n << 4 as libc::c_int;
    return c;
}
unsafe extern "C" fn zdlread(mut qdaemon: *mut sdaemon) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        if iPrecstart != iPrecend {
            c = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
            iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            c = realreadchar(qdaemon, cZtimeout);
        };
        if c < 0 as libc::c_int {
            return c;
        }
        if c & 0o140 as libc::c_int != 0 {
            return c;
        }
        match c {
            24 => {
                break;
            }
            17 => {}
            19 => {
                if iPrecstart != iPrecend {
                    c = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                } else {
                    c = realreadchar(qdaemon, 10 as libc::c_int);
                };
            }
            _ => {
                if fZesc_ctl != 0 && c & 0o140 as libc::c_int == 0 {
                    continue;
                }
                return c;
            }
        }
    }
    loop {
        if iPrecstart != iPrecend {
            c = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
            iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            c = realreadchar(qdaemon, cZtimeout);
        };
        if c < 0 as libc::c_int {
            return c;
        }
        match c {
            104 | 105 | 106 | 107 | 108 => return c | 0o400 as libc::c_int,
            109 => return 0o177 as libc::c_int,
            110 => return 0o377 as libc::c_int,
            17 => {}
            19 => {
                if iPrecstart != iPrecend {
                    c = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                } else {
                    c = realreadchar(qdaemon, 10 as libc::c_int);
                };
            }
            _ => {
                if fZesc_ctl != 0 && c & 0o140 as libc::c_int == 0 {
                    continue;
                }
                if c & 0o140 as libc::c_int == 0o100 as libc::c_int {
                    return c ^ 0o100 as libc::c_int;
                }
                return -(1 as libc::c_int);
            }
        }
    };
}
unsafe extern "C" fn noxrd7(mut qdaemon: *mut sdaemon) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        if iPrecstart != iPrecend {
            c = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
            iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            c = realreadchar(qdaemon, cZtimeout);
        };
        if c < 0 as libc::c_int {
            return c;
        }
        c &= 0o177 as libc::c_int;
        match c {
            17 => {}
            19 => {
                if iPrecstart != iPrecend {
                    c = abPrecbuf[iPrecstart as usize] as libc::c_uchar as libc::c_int;
                    iPrecstart = (iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int;
                } else {
                    c = realreadchar(qdaemon, 10 as libc::c_int);
                };
            }
            13 | 24 => return c,
            _ => {
                if fZesc_ctl != 0 && c & 0o140 as libc::c_int == 0 {
                    continue;
                }
                return c;
            }
        }
    };
}
unsafe extern "C" fn realreadchar(
    mut qdaemon: *mut sdaemon,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = breceive_char((*qdaemon).qconn, timeout, 1 as libc::c_int);
    if c >= 0 as libc::c_int {
        return c;
    }
    match c {
        -1 => return -(2 as libc::c_int),
        -2 => return -(3 as libc::c_int),
        _ => {}
    }
    ulog(
        LOG_FATAL,
        b"realreadchar: breceive_char() returned %d\0" as *const u8
            as *const libc::c_char,
        c,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn fzreceive_ready() -> boolean {
    return (iPrecstart != iPrecend) as libc::c_int;
}
unsafe extern "C" fn stohdr(mut val: hdrval_t, mut hdr: *mut libc::c_uchar) {
    *hdr.offset(0 as libc::c_int as isize) = val as libc::c_char as libc::c_uchar;
    *hdr
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int) as libc::c_char as libc::c_uchar;
    *hdr
        .offset(
            2 as libc::c_int as isize,
        ) = (val >> 16 as libc::c_int) as libc::c_char as libc::c_uchar;
    *hdr
        .offset(
            3 as libc::c_int as isize,
        ) = (val >> 24 as libc::c_int) as libc::c_char as libc::c_uchar;
}
unsafe extern "C" fn rclhdr(mut hdr: *mut libc::c_uchar) -> hdrval_t {
    let mut v: hdrval_t = 0;
    v = (*hdr.offset(3 as libc::c_int as isize) as libc::c_int & 0o377 as libc::c_int)
        as hdrval_t;
    v = v << 8 as libc::c_int
        | (*hdr.offset(2 as libc::c_int as isize) as libc::c_int & 0o377 as libc::c_int)
            as libc::c_ulong;
    v = v << 8 as libc::c_int
        | (*hdr.offset(1 as libc::c_int as isize) as libc::c_int & 0o377 as libc::c_int)
            as libc::c_ulong;
    v = v << 8 as libc::c_int
        | (*hdr.offset(0 as libc::c_int as isize) as libc::c_int & 0o377 as libc::c_int)
            as libc::c_ulong;
    return v;
}
unsafe extern "C" fn hvzencode_data_hdr(mut cbytes: winpos_t) -> hdrval_t {
    return cbytes;
}
unsafe extern "C" fn zdecode_data_hdr(mut hdrval: hdrval_t, mut pcbytes: *mut winpos_t) {
    *pcbytes = hdrval;
}
unsafe extern "C" fn lzupdate_rxpos(
    mut rx_hdr: *mut libc::c_uchar,
    mut rxpos: winpos_t,
    mut lrxpos: winpos_t,
    mut txpos: winpos_t,
) -> winpos_t {
    let mut rx_pktpos: winpos_t = 0;
    zdecode_data_hdr(rclhdr(rx_hdr), &mut rx_pktpos);
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"lzupdate_rxpos: rx_pktpos=0x%lx, rxpos=0x%lx, lrxpos=0x%lx, txpos=0x%lx\0"
                as *const u8 as *const libc::c_char,
            rx_pktpos,
            rxpos,
            lrxpos,
            txpos,
        );
    }
    if rx_pktpos < wpZlrxpos
        || rx_pktpos
            > wpZtxpos.wrapping_add(1024 as libc::c_long as libc::c_ulong)
                & !(1023 as libc::c_long) as libc::c_ulong
    {
        return rxpos;
    }
    return rx_pktpos;
}
