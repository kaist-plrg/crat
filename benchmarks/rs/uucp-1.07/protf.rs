use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    fn fconn_set(
        qconn: *mut sconnection,
        tparity: tparitysetting,
        tstrip: tstripsetting,
        txonxoff: txonxoffsetting,
    ) -> boolean;
    fn fqueue_send(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_receive(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
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
    fn usysdep_sleep(cseconds: libc::c_int);
    fn esysdep_truncate(e: openfile_t, zname: *const libc::c_char) -> openfile_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub uuconf_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfinfo {
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
    pub bsend: libc::c_char,
}
pub static mut protf_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: protf.c,v 1.36 2002/03/05 19:10:41 ian Rel $\0")
};
static mut cFtimeout: libc::c_int = 120 as libc::c_int;
static mut cFmaxretries: libc::c_int = 2 as libc::c_int;
static mut zFbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut fFfile: boolean = 0;
static mut iFcheck: libc::c_uint = 0;
static mut bFspecial: libc::c_char = 0;
static mut cFretries: libc::c_int = 0;
static mut fFacked: boolean = 0;
pub static mut asFproto_params: [uuconf_cmdtab; 3] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cFtimeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"retries\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cFmaxretries as *const libc::c_int as *mut libc::c_int
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
static mut cFsent_data: libc::c_long = 0;
static mut cFsent_bytes: libc::c_long = 0;
static mut cFrec_data: libc::c_long = 0;
static mut cFrec_bytes: libc::c_long = 0;
static mut cFsend_retries: libc::c_long = 0;
static mut cFrec_retries: libc::c_long = 0;
pub unsafe extern "C" fn ffstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    *pzlog = 0 as *mut libc::c_char;
    cFsent_data = 0 as libc::c_int as libc::c_long;
    cFsent_bytes = 0 as libc::c_int as libc::c_long;
    cFrec_data = 0 as libc::c_int as libc::c_long;
    cFrec_bytes = 0 as libc::c_int as libc::c_long;
    cFsend_retries = 0 as libc::c_int as libc::c_long;
    cFrec_retries = 0 as libc::c_int as libc::c_long;
    if fconn_set(
        (*qdaemon).qconn,
        PARITYSETTING_DEFAULT,
        STRIPSETTING_SEVENBITS,
        XONXOFF_ON,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    usysdep_sleep(2 as libc::c_int);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ffshutdown(mut qdaemon: *mut sdaemon) -> boolean {
    xfree(zFbuf as pointer);
    zFbuf = 0 as *mut libc::c_char;
    ulog(
        LOG_NORMAL,
        b"Protocol 'f': sent %ld bytes for %ld, received %ld bytes for %ld\0"
            as *const u8 as *const libc::c_char,
        cFsent_bytes,
        cFsent_data,
        cFrec_bytes,
        cFrec_data,
    );
    if cFsend_retries != 0 as libc::c_int as libc::c_long
        || cFrec_retries != 0 as libc::c_int as libc::c_long
    {
        ulog(
            LOG_NORMAL,
            b"Protocol 'f' file retries: %ld sending, %ld receiving\0" as *const u8
                as *const libc::c_char,
            cFsend_retries,
            cFrec_retries,
        );
    }
    cFtimeout = 120 as libc::c_int;
    cFmaxretries = 2 as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ffsendcmd(
    mut qdaemon: *mut sdaemon,
    mut z: *const libc::c_char,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
) -> boolean {
    let mut clen: size_t = 0;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"ffsendcmd: Sending command \"%s\"\0" as *const u8 as *const libc::c_char,
            z,
        );
    }
    clen = strlen(z);
    zalc = zbufalc(clen.wrapping_add(2 as libc::c_int as libc::c_ulong));
    memcpy(zalc as *mut libc::c_void, z as *const libc::c_void, clen);
    *zalc.offset(clen as isize) = '\r' as i32 as libc::c_char;
    *zalc
        .offset(
            clen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    fret = fsend_data(
        (*qdaemon).qconn,
        zalc,
        clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    );
    ubuffree(zalc);
    return fret;
}
pub unsafe extern "C" fn zfgetspace(
    mut qdaemon: *mut sdaemon,
    mut pclen: *mut size_t,
) -> *mut libc::c_char {
    *pclen = 256 as libc::c_int as size_t;
    if zFbuf.is_null() {
        zFbuf = xmalloc(256 as libc::c_int as size_t) as *mut libc::c_char;
    }
    return zFbuf;
}
pub unsafe extern "C" fn ffsenddata(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
    mut ipos: libc::c_long,
) -> boolean {
    let mut ab: [libc::c_char; 512] = [0; 512];
    let mut ze: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itmpchk: libc::c_uint = 0;
    cFsent_data = (cFsent_data as libc::c_ulong).wrapping_add(cdata) as libc::c_long
        as libc::c_long;
    ze = ab.as_mut_ptr();
    itmpchk = iFcheck;
    loop {
        let fresh0 = cdata;
        cdata = cdata.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut b: libc::c_int = 0;
        if itmpchk & 0x8000 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            itmpchk <<= 1 as libc::c_int;
        } else {
            itmpchk <<= 1 as libc::c_int;
            itmpchk = itmpchk.wrapping_add(1);
            itmpchk;
        }
        let fresh1 = zdata;
        zdata = zdata.offset(1);
        b = *fresh1 as libc::c_int & 0xff as libc::c_int;
        itmpchk = itmpchk.wrapping_add(b as libc::c_uint);
        if b <= 0o177 as libc::c_int {
            if b <= 0o37 as libc::c_int {
                let fresh2 = ze;
                ze = ze.offset(1);
                *fresh2 = 'z' as i32 as libc::c_char;
                let fresh3 = ze;
                ze = ze.offset(1);
                *fresh3 = (b + 0o100 as libc::c_int) as libc::c_char;
            } else if b <= 0o171 as libc::c_int {
                let fresh4 = ze;
                ze = ze.offset(1);
                *fresh4 = b as libc::c_char;
            } else {
                let fresh5 = ze;
                ze = ze.offset(1);
                *fresh5 = '{' as i32 as libc::c_char;
                let fresh6 = ze;
                ze = ze.offset(1);
                *fresh6 = (b - 0o100 as libc::c_int) as libc::c_char;
            }
        } else if b <= 0o237 as libc::c_int {
            let fresh7 = ze;
            ze = ze.offset(1);
            *fresh7 = '|' as i32 as libc::c_char;
            let fresh8 = ze;
            ze = ze.offset(1);
            *fresh8 = (b - 0o100 as libc::c_int) as libc::c_char;
        } else if b <= 0o371 as libc::c_int {
            let fresh9 = ze;
            ze = ze.offset(1);
            *fresh9 = '}' as i32 as libc::c_char;
            let fresh10 = ze;
            ze = ze.offset(1);
            *fresh10 = (b - 0o200 as libc::c_int) as libc::c_char;
        } else {
            let fresh11 = ze;
            ze = ze.offset(1);
            *fresh11 = '~' as i32 as libc::c_char;
            let fresh12 = ze;
            ze = ze.offset(1);
            *fresh12 = (b - 0o300 as libc::c_int) as libc::c_char;
        }
    }
    iFcheck = itmpchk;
    cFsent_bytes += ze.offset_from(ab.as_mut_ptr()) as libc::c_long;
    return fsend_data(
        (*qdaemon).qconn,
        ab.as_mut_ptr(),
        ze.offset_from(ab.as_mut_ptr()) as libc::c_long as size_t,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn ffprocess_data(
    mut qdaemon: *mut sdaemon,
    mut pfexit: *mut boolean,
    mut pcneed: *mut size_t,
) -> boolean {
    let mut i: libc::c_int = 0;
    let mut itmpchk: libc::c_uint = 0;
    *pfexit = 0 as libc::c_int;
    if !pcneed.is_null() {
        *pcneed = 1 as libc::c_int as size_t;
    }
    if fFfile == 0 {
        while iPrecstart != iPrecend {
            i = iPrecstart;
            while i < 16384 as libc::c_int && i != iPrecend {
                abPrecbuf[i
                    as usize] = (abPrecbuf[i as usize] as libc::c_int
                    & 0x7f as libc::c_int) as libc::c_char;
                if abPrecbuf[i as usize] as libc::c_int == '\r' as i32 {
                    let mut istart: libc::c_int = 0;
                    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                        ulog(
                            LOG_DEBUG,
                            b"ffprocess_data: Got %d command bytes\0" as *const u8
                                as *const libc::c_char,
                            i - iPrecstart + 1 as libc::c_int,
                        );
                    }
                    abPrecbuf[i as usize] = '\0' as i32 as libc::c_char;
                    istart = iPrecstart;
                    iPrecstart = (i + 1 as libc::c_int) % 16384 as libc::c_int;
                    if !pcneed.is_null() {
                        *pcneed = 0 as libc::c_int as size_t;
                    }
                    return fgot_data(
                        qdaemon,
                        abPrecbuf.as_mut_ptr().offset(istart as isize),
                        (i - istart + 1 as libc::c_int) as size_t,
                        0 as *mut libc::c_void as *const libc::c_char,
                        0 as libc::c_int as size_t,
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        -(1 as libc::c_int) as libc::c_long,
                        1 as libc::c_int,
                        pfexit,
                    );
                }
                i += 1;
                i;
            }
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"ffprocess_data: Got %d command bytes\0" as *const u8
                        as *const libc::c_char,
                    i - iPrecstart,
                );
            }
            if fgot_data(
                qdaemon,
                abPrecbuf.as_mut_ptr().offset(iPrecstart as isize),
                (i - iPrecstart) as size_t,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_int as size_t,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int) as libc::c_long,
                1 as libc::c_int,
                pfexit,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            iPrecstart = i % 16384 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    itmpchk = iFcheck;
    while iPrecstart != iPrecend {
        let mut zstart: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_int = 0;
        zstart = abPrecbuf.as_mut_ptr().offset(iPrecstart as isize);
        zfrom = zstart;
        zto = zfrom;
        c = iPrecend - iPrecstart;
        if c < 0 as libc::c_int {
            c = 16384 as libc::c_int - iPrecstart;
        }
        loop {
            let fresh13 = c;
            c = c - 1;
            if !(fresh13 != 0 as libc::c_int) {
                break;
            }
            let mut b: libc::c_int = 0;
            let fresh14 = zfrom;
            zfrom = zfrom.offset(1);
            b = *fresh14 as libc::c_int & 0x7f as libc::c_int;
            if b < 0o40 as libc::c_int || b > 0o176 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"Illegal byte %d\0" as *const u8 as *const libc::c_char,
                    b,
                );
            } else if b >= 0o172 as libc::c_int {
                if bFspecial as libc::c_int != 0 as libc::c_int {
                    if bFspecial as libc::c_int != 0o176 as libc::c_int
                        || b != 0o176 as libc::c_int
                    {
                        ulog(
                            LOG_ERROR,
                            b"Illegal bytes %d %d\0" as *const u8 as *const libc::c_char,
                            bFspecial as libc::c_int,
                            b,
                        );
                        bFspecial = 0 as libc::c_int as libc::c_char;
                    } else {
                        if zto != zstart {
                            cFrec_bytes
                                += zfrom.offset_from(zstart) as libc::c_long
                                    - 2 as libc::c_int as libc::c_long;
                            cFrec_data += zto.offset_from(zstart) as libc::c_long;
                            if fgot_data(
                                qdaemon,
                                zstart,
                                zto.offset_from(zstart) as libc::c_long as size_t,
                                0 as *mut libc::c_void as *const libc::c_char,
                                0 as libc::c_int as size_t,
                                -(1 as libc::c_int),
                                -(1 as libc::c_int),
                                -(1 as libc::c_int) as libc::c_long,
                                1 as libc::c_int,
                                pfexit,
                            ) == 0
                            {
                                return 0 as libc::c_int;
                            }
                        }
                        iPrecstart = (zfrom
                            .offset(iPrecstart as isize)
                            .offset_from(zstart) as libc::c_long
                            % 16384 as libc::c_int as libc::c_long) as libc::c_int;
                        iFcheck = itmpchk;
                        if !pcneed.is_null() {
                            *pcneed = 0 as libc::c_int as size_t;
                        }
                        return fgot_data(
                            qdaemon,
                            0 as *mut libc::c_void as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            0 as *mut libc::c_void as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            -(1 as libc::c_int),
                            -(1 as libc::c_int),
                            -(1 as libc::c_int) as libc::c_long,
                            1 as libc::c_int,
                            pfexit,
                        );
                    }
                } else {
                    bFspecial = b as libc::c_char;
                }
            } else {
                let mut bnext: libc::c_int = 0;
                match bFspecial as libc::c_int {
                    122 => {
                        bnext = b - 0o100 as libc::c_int;
                    }
                    123 | 124 => {
                        bnext = b + 0o100 as libc::c_int;
                    }
                    125 => {
                        bnext = b + 0o200 as libc::c_int;
                    }
                    126 => {
                        bnext = b + 0o300 as libc::c_int;
                    }
                    _ => {
                        bnext = b;
                    }
                }
                let fresh15 = zto;
                zto = zto.offset(1);
                *fresh15 = bnext as libc::c_char;
                bFspecial = 0 as libc::c_int as libc::c_char;
                if itmpchk & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    itmpchk <<= 1 as libc::c_int;
                } else {
                    itmpchk <<= 1 as libc::c_int;
                    itmpchk = itmpchk.wrapping_add(1);
                    itmpchk;
                }
                itmpchk = itmpchk.wrapping_add(bnext as libc::c_uint);
            }
        }
        if zto != zstart {
            if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"ffprocess_data: Got %d bytes\0" as *const u8
                        as *const libc::c_char,
                    zto.offset_from(zstart) as libc::c_long,
                );
            }
            cFrec_data += zto.offset_from(zstart) as libc::c_long;
            if fgot_data(
                qdaemon,
                zstart,
                zto.offset_from(zstart) as libc::c_long as size_t,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_int as size_t,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int) as libc::c_long,
                1 as libc::c_int,
                pfexit,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        cFrec_bytes += zfrom.offset_from(zstart) as libc::c_long;
        iPrecstart = (zfrom.offset(iPrecstart as isize).offset_from(zstart)
            as libc::c_long % 16384 as libc::c_int as libc::c_long) as libc::c_int;
    }
    iFcheck = itmpchk;
    if !pcneed.is_null() {
        if bFspecial as libc::c_int == 0o176 as libc::c_int {
            *pcneed = 6 as libc::c_int as size_t;
        } else {
            *pcneed = 7 as libc::c_int as size_t;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ffwait(mut qdaemon: *mut sdaemon) -> boolean {
    loop {
        let mut fexit: boolean = 0;
        let mut cneed: size_t = 0;
        let mut crec: size_t = 0;
        if ffprocess_data(qdaemon, &mut fexit, &mut cneed) == 0 {
            return 0 as libc::c_int;
        }
        if fexit != 0 {
            return 1 as libc::c_int;
        }
        if cneed > 0 as libc::c_int as libc::c_ulong {
            if freceive_data(
                (*qdaemon).qconn,
                cneed,
                &mut crec,
                cFtimeout,
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if crec == 0 as libc::c_int as libc::c_ulong {
                ulog(
                    LOG_ERROR,
                    b"Timed out waiting for data\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
        }
    };
}
pub unsafe extern "C" fn fffile(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
    mut fstart: boolean,
    mut fsend: boolean,
    mut cbytes: libc::c_long,
    mut pfhandled: *mut boolean,
) -> boolean {
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fffile: fstart %s; fsend %s; fFacked %s\0" as *const u8
                as *const libc::c_char,
            if fstart != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            if fsend != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
            if fFacked != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    }
    *pfhandled = 0 as libc::c_int;
    if fstart != 0 {
        iFcheck = 0xffff as libc::c_int as libc::c_uint;
        cFretries = 0 as libc::c_int;
        fFacked = 0 as libc::c_int;
        if fsend == 0 {
            bFspecial = 0 as libc::c_int as libc::c_char;
            fFfile = 1 as libc::c_int;
        }
        return 1 as libc::c_int;
    } else {
        let mut qinfo: *mut sfinfo = 0 as *mut sfinfo;
        if fFacked != 0 {
            fFacked = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        if fsend != 0 {
            let mut ab: [libc::c_char; 8] = [0; 8];
            sprintf(
                ab.as_mut_ptr(),
                b"~~%04x\r\0" as *const u8 as *const libc::c_char,
                iFcheck & 0xffff as libc::c_int as libc::c_uint,
            );
            if fsend_data(
                (*qdaemon).qconn,
                ab.as_mut_ptr(),
                7 as libc::c_int as size_t,
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            fFfile = 0 as libc::c_int;
            qinfo = xmalloc(::std::mem::size_of::<sfinfo>() as libc::c_ulong)
                as *mut sfinfo;
            (*qinfo).psendfn = (*qtrans).psendfn;
            (*qinfo).precfn = (*qtrans).precfn;
            (*qinfo).pinfo = (*qtrans).pinfo;
            (*qtrans).psendfn = None;
            (*qtrans)
                .precfn = Some(
                ffawait_ack
                    as unsafe extern "C" fn(
                        *mut stransfer,
                        *mut sdaemon,
                        *const libc::c_char,
                        size_t,
                    ) -> boolean,
            );
            (*qtrans).pinfo = qinfo as pointer;
            (*qtrans).fcmd = 1 as libc::c_int;
            *pfhandled = 1 as libc::c_int;
            return fqueue_receive(qdaemon, qtrans);
        } else {
            fFfile = 0 as libc::c_int;
            qinfo = xmalloc(::std::mem::size_of::<sfinfo>() as libc::c_ulong)
                as *mut sfinfo;
            (*qinfo).psendfn = (*qtrans).psendfn;
            (*qinfo).precfn = (*qtrans).precfn;
            (*qinfo).pinfo = (*qtrans).pinfo;
            (*qtrans).psendfn = None;
            (*qtrans)
                .precfn = Some(
                ffawait_cksum
                    as unsafe extern "C" fn(
                        *mut stransfer,
                        *mut sdaemon,
                        *const libc::c_char,
                        size_t,
                    ) -> boolean,
            );
            (*qtrans).pinfo = qinfo as pointer;
            (*qtrans).fcmd = 1 as libc::c_int;
            *pfhandled = 1 as libc::c_int;
            return fqueue_receive(qdaemon, qtrans);
        }
    };
}
unsafe extern "C" fn ffawait_ack(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut sfinfo = (*qtrans).pinfo as *mut sfinfo;
    (*qtrans).precfn = None;
    if *zdata as libc::c_int == 'R' as i32 {
        if ((*qtrans).e).is_null() {
            ulog(
                LOG_ERROR,
                b"Request to resent non-file\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        cFretries += 1;
        cFretries;
        if cFretries > cFmaxretries {
            ulog(LOG_ERROR, b"Too many retries\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        ulog(LOG_NORMAL, b"Resending file\0" as *const u8 as *const libc::c_char);
        if !(fseek((*qtrans).e, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
            == 0 as libc::c_int)
        {
            ulog(
                LOG_ERROR,
                b"rewind: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        (*qtrans).ipos = 0 as libc::c_int as libc::c_long;
        iFcheck = 0xffff as libc::c_int as libc::c_uint;
        cFsend_retries += 1;
        cFsend_retries;
        (*qtrans).psendfn = (*qinfo).psendfn;
        (*qtrans).precfn = (*qinfo).precfn;
        (*qtrans).pinfo = (*qinfo).pinfo;
        xfree(qinfo as pointer);
        (*qtrans).fsendfile = 1 as libc::c_int;
        return fqueue_send(qdaemon, qtrans);
    }
    if *zdata as libc::c_int != 'G' as i32 {
        if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fffile: Got \"%s\"\0" as *const u8 as *const libc::c_char,
                zdata,
            );
        }
        ulog(LOG_ERROR, b"File send failed\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    (*qtrans).psendfn = (*qinfo).psendfn;
    (*qtrans).precfn = (*qinfo).precfn;
    (*qtrans).pinfo = (*qinfo).pinfo;
    xfree(qinfo as pointer);
    fFacked = 1 as libc::c_int;
    return (Some(((*qtrans).psendfn).unwrap())).unwrap()(qtrans, qdaemon);
}
unsafe extern "C" fn ffawait_cksum(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut sfinfo = (*qtrans).pinfo as *mut sfinfo;
    let mut icheck: libc::c_uint = 0;
    (*qtrans).precfn = None;
    if *(*__ctype_b_loc())
        .offset(*zdata.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        || *(*__ctype_b_loc())
            .offset(*zdata.offset(1 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(*zdata.offset(2 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(*zdata.offset(3 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *zdata.offset(4 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        ulog(LOG_ERROR, b"Bad checksum format\0" as *const u8 as *const libc::c_char);
        xfree((*qtrans).pinfo);
        return 0 as libc::c_int;
    }
    icheck = strtol(
        zdata as *mut libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        16 as libc::c_int,
    ) as libc::c_uint;
    if icheck != iFcheck & 0xffff as libc::c_int as libc::c_uint {
        if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"Checksum failed; calculated 0x%x, got 0x%x\0" as *const u8
                    as *const libc::c_char,
                iFcheck & 0xffff as libc::c_int as libc::c_uint,
                icheck,
            );
        }
        if ((*qtrans).e).is_null() {
            ulog(
                LOG_ERROR,
                b"Failed to get non-file\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        cFretries += 1;
        cFretries;
        if cFretries > cFmaxretries {
            ulog(LOG_ERROR, b"Too many retries\0" as *const u8 as *const libc::c_char);
            (*qinfo).bsend = 'Q' as i32 as libc::c_char;
        } else {
            ulog(LOG_NORMAL, b"File being resent\0" as *const u8 as *const libc::c_char);
            (*qtrans).e = esysdep_truncate((*qtrans).e, (*qtrans).s.ztemp);
            if ((*qtrans).e).is_null() {
                return 0 as libc::c_int;
            }
            (*qtrans).ipos = 0 as libc::c_int as libc::c_long;
            iFcheck = 0xffff as libc::c_int as libc::c_uint;
            bFspecial = 0 as libc::c_int as libc::c_char;
            fFfile = 1 as libc::c_int;
            cFrec_retries += 1;
            cFrec_retries;
            (*qinfo).bsend = 'R' as i32 as libc::c_char;
        }
    } else {
        (*qinfo).bsend = 'G' as i32 as libc::c_char;
    }
    (*qtrans)
        .psendfn = Some(
        ffsend_ack as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    return fqueue_send(qdaemon, qtrans);
}
unsafe extern "C" fn ffsend_ack(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut sfinfo = (*qtrans).pinfo as *mut sfinfo;
    let mut ab: [libc::c_char; 2] = [0; 2];
    ab[0 as libc::c_int as usize] = (*qinfo).bsend;
    ab[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if ffsendcmd(qdaemon, ab.as_mut_ptr(), 0 as libc::c_int, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    (*qtrans).psendfn = (*qinfo).psendfn;
    (*qtrans).precfn = (*qinfo).precfn;
    (*qtrans).pinfo = (*qinfo).pinfo;
    xfree(qinfo as pointer);
    if ab[0 as libc::c_int as usize] as libc::c_int == 'Q' as i32 {
        return 0 as libc::c_int;
    }
    if ab[0 as libc::c_int as usize] as libc::c_int == 'R' as i32 {
        (*qtrans).frecfile = 1 as libc::c_int;
        return fqueue_receive(qdaemon, qtrans);
    }
    fFacked = 1 as libc::c_int;
    return (Some(((*qtrans).precfn).unwrap()))
        .unwrap()(
        qtrans,
        qdaemon,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
}
