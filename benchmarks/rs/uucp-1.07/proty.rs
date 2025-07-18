use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
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
pub static mut proty_id: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: proty.c,v 1.9 2003/05/29 06:00:49 ian Rel $\0")
};
static mut iYlocal_packsize: size_t = 1024 as libc::c_int as size_t;
static mut iYremote_packsize: size_t = 1024 as libc::c_int as size_t;
static mut iYlocal_pktnum: libc::c_ushort = 0;
static mut iYremote_pktnum: libc::c_ushort = 0;
static mut cYtimeout: libc::c_int = 60 as libc::c_int;
static mut zYbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut asYproto_params: [uuconf_cmdtab; 3] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cYtimeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"packet-size\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &iYlocal_packsize as *const size_t as *mut size_t
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
unsafe extern "C" fn fyxchg_syncs(mut qdaemon: *mut sdaemon) -> boolean {
    let mut inithdr: [libc::c_char; 4] = [0; 4];
    let mut header: [libc::c_ushort; 3] = [0; 3];
    let mut ichk: libc::c_ushort = 0;
    let mut clen: size_t = 0;
    let mut cfirst: size_t = 0;
    let mut rpktsize: libc::c_int = 0;
    inithdr[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char;
    inithdr[1 as libc::c_int
        as usize] = (iYlocal_packsize >> 8 as libc::c_int) as libc::c_char;
    *inithdr
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *inithdr
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (0 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    if fysend_pkt(
        qdaemon,
        inithdr.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if fywait_for_header(qdaemon, header.as_mut_ptr(), cYtimeout) == 0 {
        return 0 as libc::c_int;
    }
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fyxchg_syncs: Got sync header\0" as *const u8 as *const libc::c_char,
        );
    }
    clen = header[1 as libc::c_int as usize] as size_t;
    if clen < 4 as libc::c_int as libc::c_ulong
        || clen > 1024 as libc::c_int as libc::c_ulong
    {
        ulog(
            LOG_ERROR,
            b"Bad 'y' protocol sync packet length\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if fyread_data(qdaemon, clen, cYtimeout) == 0 {
        return 0 as libc::c_int;
    }
    cfirst = (16384 as libc::c_int - iPrecstart) as size_t;
    ichk = iychecksum2(
        abPrecbuf.as_mut_ptr().offset(iPrecstart as isize),
        cfirst,
        abPrecbuf.as_mut_ptr(),
        clen.wrapping_sub(cfirst),
    );
    rpktsize = abPrecbuf[((iPrecstart + 1 as libc::c_int) % 16384 as libc::c_int)
        as usize] as libc::c_uchar as libc::c_int;
    if rpktsize == 0 as libc::c_int
        || header[2 as libc::c_int as usize] as libc::c_int != ichk as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"Bad 'y' protocol sync packet\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    iYremote_packsize = (rpktsize << 8 as libc::c_int) as size_t;
    if iYremote_packsize > iYlocal_packsize {
        iYremote_packsize = iYlocal_packsize;
    }
    iPrecstart = (iPrecstart as libc::c_ulong)
        .wrapping_add(clen)
        .wrapping_rem(16384 as libc::c_int as libc::c_ulong) as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fystart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
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
    iYremote_pktnum = 0 as libc::c_int as libc::c_ushort;
    iYlocal_pktnum = iYremote_pktnum;
    iYlocal_packsize &= !(0xff as libc::c_int) as libc::c_ulong;
    if iYlocal_packsize < 256 as libc::c_int as libc::c_ulong
        || iYlocal_packsize > (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
    {
        iYlocal_packsize = 1024 as libc::c_int as size_t;
    }
    if fyxchg_syncs(qdaemon) == 0 {
        cYtimeout = 60 as libc::c_int;
        iYlocal_packsize = 1024 as libc::c_int as size_t;
        return 0 as libc::c_int;
    }
    zYbuf = xmalloc((1024 as libc::c_int + 6 as libc::c_int) as size_t)
        as *mut libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fyshutdown(mut qdaemon: *mut sdaemon) -> boolean {
    xfree(zYbuf as pointer);
    zYbuf = 0 as *mut libc::c_char;
    cYtimeout = 60 as libc::c_int;
    iYlocal_packsize = 1024 as libc::c_int as size_t;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fysendcmd(
    mut qdaemon: *mut sdaemon,
    mut z: *const libc::c_char,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
) -> boolean {
    let mut clen: size_t = 0;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fysendcmd: Sending command \"%s\"\0" as *const u8 as *const libc::c_char,
            z,
        );
    }
    clen = (strlen(z)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    while clen > 0 as libc::c_int as libc::c_ulong {
        let mut csize: size_t = 0;
        csize = clen;
        if csize > iYremote_packsize {
            csize = iYremote_packsize;
        }
        if fysend_pkt(qdaemon, z as *const libc::c_void, csize) == 0 {
            return 0 as libc::c_int;
        }
        z = z.offset(csize as isize);
        clen = (clen as libc::c_ulong).wrapping_sub(csize) as size_t as size_t;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zygetspace(
    mut qdaemon: *mut sdaemon,
    mut pclen: *mut size_t,
) -> *mut libc::c_char {
    *pclen = iYremote_packsize;
    return zYbuf.offset(6 as libc::c_int as isize);
}
pub unsafe extern "C" fn fysenddata(
    mut qdaemon: *mut sdaemon,
    mut zdata: *mut libc::c_char,
    mut cdata: size_t,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
    mut ipos: libc::c_long,
) -> boolean {
    if cdata > iYremote_packsize {
        ulog(
            LOG_FATAL,
            b"fysend_packet: Packet size too large\0" as *const u8 as *const libc::c_char,
        );
    }
    *zYbuf
        .offset(0 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (iYlocal_pktnum as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *zYbuf
        .offset(0 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (iYlocal_pktnum as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_char;
    iYlocal_pktnum = iYlocal_pktnum.wrapping_add(1);
    iYlocal_pktnum;
    *zYbuf
        .offset(2 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (cdata & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    *zYbuf
        .offset(2 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (cdata >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    *zYbuf
        .offset(4 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (iychecksum(zdata, cdata) as libc::c_int & 0xff as libc::c_int)
        as libc::c_char;
    *zYbuf
        .offset(4 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (iychecksum(zdata, cdata) as libc::c_int >> 8 as libc::c_int
        & 0xff as libc::c_int) as libc::c_char;
    return fsend_data(
        (*qdaemon).qconn,
        zYbuf,
        cdata.wrapping_add(6 as libc::c_int as libc::c_ulong),
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn fywait(mut qdaemon: *mut sdaemon) -> boolean {
    let mut fexit: boolean = 0 as libc::c_int;
    while fexit == 0 {
        if fywait_for_packet(qdaemon, &mut fexit) == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fyfile(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
    mut fstart: boolean,
    mut fsend: boolean,
    mut cbytes: libc::c_long,
    mut pfhandled: *mut boolean,
) -> boolean {
    let mut header: [libc::c_ushort; 3] = [0; 3];
    *pfhandled = 0 as libc::c_int;
    if fstart == 0 {
        if fsend != 0 {
            if fywait_for_header(
                qdaemon,
                header.as_mut_ptr(),
                cYtimeout * 2 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if header[1 as libc::c_int as usize] as libc::c_int
                != 0xfffe as libc::c_int as libc::c_ushort as libc::c_int
            {
                if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int)
                    != 0 as libc::c_int
                {
                    ulog(
                        LOG_DEBUG,
                        b"fyfile: Error from remote: 0x%04X\0" as *const u8
                            as *const libc::c_char,
                        header[1 as libc::c_int as usize] as libc::c_int,
                    );
                }
                ulog(
                    LOG_ERROR,
                    b"Received 'y' protocol error from remote\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
        } else {
            return fysend_control(qdaemon, 0xfffe as libc::c_int)
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fysend_control(
    mut qdaemon: *mut sdaemon,
    mut itype: libc::c_int,
) -> boolean {
    let mut header: [libc::c_char; 6] = [0; 6];
    *header
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (iYlocal_pktnum as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (iYlocal_pktnum as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_char;
    iYlocal_pktnum = iYlocal_pktnum.wrapping_add(1);
    iYlocal_pktnum;
    *header
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (itype & 0xff as libc::c_int) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (itype >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (0 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    return fsend_data(
        (*qdaemon).qconn,
        header.as_mut_ptr(),
        6 as libc::c_int as size_t,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn fysend_pkt(
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_void,
    mut cdata: size_t,
) -> boolean {
    let mut header: [libc::c_char; 6] = [0; 6];
    *header
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (iYlocal_pktnum as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (iYlocal_pktnum as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_char;
    iYlocal_pktnum = iYlocal_pktnum.wrapping_add(1);
    iYlocal_pktnum;
    *header
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (cdata & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (cdata >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (iychecksum(zdata as *const libc::c_char, cdata) as libc::c_int
        & 0xff as libc::c_int) as libc::c_char;
    *header
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (iychecksum(zdata as *const libc::c_char, cdata) as libc::c_int
        >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    if fsend_data(
        (*qdaemon).qconn,
        header.as_mut_ptr(),
        6 as libc::c_int as size_t,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return fsend_data(
        (*qdaemon).qconn,
        zdata as *const libc::c_char,
        cdata,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn fyread_data(
    mut qdaemon: *mut sdaemon,
    mut clen: size_t,
    mut timeout: libc::c_int,
) -> boolean {
    let mut cinbuf: libc::c_int = 0;
    let mut crec: size_t = 0;
    cinbuf = iPrecend - iPrecstart;
    if cinbuf < 0 as libc::c_int {
        cinbuf += 16384 as libc::c_int;
    }
    if (cinbuf as size_t) < clen {
        if freceive_data(
            (*qdaemon).qconn,
            clen.wrapping_sub(cinbuf as libc::c_ulong),
            &mut crec,
            timeout,
            1 as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        cinbuf = (cinbuf as libc::c_ulong).wrapping_add(crec) as libc::c_int
            as libc::c_int;
        if (cinbuf as size_t) < clen {
            if freceive_data(
                (*qdaemon).qconn,
                clen.wrapping_sub(cinbuf as libc::c_ulong),
                &mut crec,
                timeout,
                1 as libc::c_int,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        cinbuf = (cinbuf as libc::c_ulong).wrapping_add(crec) as libc::c_int
            as libc::c_int;
        if (cinbuf as size_t) < clen {
            ulog(
                LOG_ERROR,
                b"Timed out waiting for data\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fywait_for_header(
    mut qdaemon: *mut sdaemon,
    mut header: *mut libc::c_ushort,
    mut timeout: libc::c_int,
) -> boolean {
    if fyread_data(qdaemon, 6 as libc::c_int as size_t, timeout) == 0 {
        return 0 as libc::c_int;
    }
    if iPrecstart <= 16384 as libc::c_int - 6 as libc::c_int {
        *header
            .offset(
                0 as libc::c_int as isize,
            ) = ((*abPrecbuf
            .as_mut_ptr()
            .offset(iPrecstart as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            + ((*abPrecbuf
                .as_mut_ptr()
                .offset(iPrecstart as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int)) as libc::c_ushort;
        *header
            .offset(
                1 as libc::c_int as isize,
            ) = ((*abPrecbuf
            .as_mut_ptr()
            .offset(iPrecstart as isize)
            .offset(2 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            + ((*abPrecbuf
                .as_mut_ptr()
                .offset(iPrecstart as isize)
                .offset(2 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int)) as libc::c_ushort;
        *header
            .offset(
                2 as libc::c_int as isize,
            ) = ((*abPrecbuf
            .as_mut_ptr()
            .offset(iPrecstart as isize)
            .offset(4 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            + ((*abPrecbuf
                .as_mut_ptr()
                .offset(iPrecstart as isize)
                .offset(4 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int)) as libc::c_ushort;
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        i = j;
        while j < 6 as libc::c_int {
            *header
                .offset(
                    i as isize,
                ) = (((abPrecbuf[((iPrecstart + j + 1 as libc::c_int)
                % 16384 as libc::c_int) as usize] as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int)
                + (abPrecbuf[((iPrecstart + j) % 16384 as libc::c_int) as usize]
                    as libc::c_int & 0xff as libc::c_int)) as libc::c_ushort;
            i += 1;
            i;
            j += 2 as libc::c_int;
        }
    }
    iPrecstart = (iPrecstart + 6 as libc::c_int) % 16384 as libc::c_int;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fywait_for_header: Got header: 0x%04X, 0x%04X, 0x%04X\0" as *const u8
                as *const libc::c_char,
            *header.offset(0 as libc::c_int as isize) as libc::c_int,
            *header.offset(1 as libc::c_int as isize) as libc::c_int,
            *header.offset(2 as libc::c_int as isize) as libc::c_int,
        );
    }
    let fresh0 = iYremote_pktnum;
    iYremote_pktnum = iYremote_pktnum.wrapping_add(1);
    if *header.offset(0 as libc::c_int as isize) as libc::c_int != fresh0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"Incorrect 'y' packet sequence\0" as *const u8 as *const libc::c_char,
        );
        fysend_control(qdaemon, 0xfffc as libc::c_int);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fywait_for_packet(
    mut qdaemon: *mut sdaemon,
    mut pfexit: *mut boolean,
) -> boolean {
    let mut header: [libc::c_ushort; 3] = [0; 3];
    let mut ichk: libc::c_ushort = 0;
    let mut clen: size_t = 0;
    let mut cfirst: size_t = 0;
    if fywait_for_header(qdaemon, header.as_mut_ptr(), cYtimeout) == 0 {
        return 0 as libc::c_int;
    }
    clen = header[1 as libc::c_int as usize] as size_t;
    if clen == 0 as libc::c_int as libc::c_ulong && !pfexit.is_null() {
        return fgot_data(
            qdaemon,
            abPrecbuf.as_mut_ptr(),
            0 as libc::c_int as size_t,
            abPrecbuf.as_mut_ptr(),
            0 as libc::c_int as size_t,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            -(1 as libc::c_int) as libc::c_long,
            1 as libc::c_int,
            pfexit,
        );
    }
    if clen & 0x8000 as libc::c_int as libc::c_ulong != 0 {
        if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fywait_for_packet: Error from remote: 0x%04X\0" as *const u8
                    as *const libc::c_char,
                header[1 as libc::c_int as usize] as libc::c_int,
            );
        }
        ulog(LOG_ERROR, b"Remote error packet\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if clen > iYlocal_packsize {
        ulog(LOG_ERROR, b"Packet too large\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if fyread_data(qdaemon, clen, cYtimeout) == 0 {
        return 0 as libc::c_int;
    }
    cfirst = (16384 as libc::c_int - iPrecstart) as size_t;
    if cfirst > clen {
        cfirst = clen;
    }
    if cfirst == clen {
        ichk = iychecksum(abPrecbuf.as_mut_ptr().offset(iPrecstart as isize), clen);
    } else {
        ichk = iychecksum2(
            abPrecbuf.as_mut_ptr().offset(iPrecstart as isize),
            cfirst,
            abPrecbuf.as_mut_ptr(),
            clen.wrapping_sub(cfirst),
        );
    }
    if header[2 as libc::c_int as usize] as libc::c_int != ichk as libc::c_int {
        if iDebug & (0o20 as libc::c_int | 0o1 as libc::c_int) != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fywait_for_packet: Bad checksum 0x%x != 0x%x\0" as *const u8
                    as *const libc::c_char,
                header[2 as libc::c_int as usize] as libc::c_int,
                ichk as libc::c_int,
            );
        }
        fysend_control(qdaemon, 0xfffd as libc::c_int);
        ulog(LOG_ERROR, b"Checksum error\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    if !pfexit.is_null()
        && fgot_data(
            qdaemon,
            abPrecbuf.as_mut_ptr().offset(iPrecstart as isize),
            cfirst,
            abPrecbuf.as_mut_ptr(),
            clen.wrapping_sub(cfirst),
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            -(1 as libc::c_int) as libc::c_long,
            1 as libc::c_int,
            pfexit,
        ) == 0
    {
        return 0 as libc::c_int;
    }
    iPrecstart = (iPrecstart as libc::c_ulong)
        .wrapping_add(clen)
        .wrapping_rem(16384 as libc::c_int as libc::c_ulong) as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn iychecksum(
    mut z: *const libc::c_char,
    mut c: size_t,
) -> libc::c_ushort {
    let mut ichk: libc::c_ushort = 0;
    ichk = 0xffff as libc::c_int as libc::c_ushort;
    loop {
        let fresh1 = c;
        c = c.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        ichk = (ichk as libc::c_int
            + (ichk as libc::c_int
                + ((ichk as libc::c_int & 0x8000 as libc::c_int) >> 15 as libc::c_int)))
            as libc::c_ushort;
        let fresh2 = z;
        z = z.offset(1);
        ichk = (ichk as libc::c_int + *fresh2 as libc::c_uchar as libc::c_int)
            as libc::c_ushort;
    }
    return ichk;
}
unsafe extern "C" fn iychecksum2(
    mut zfirst: *const libc::c_char,
    mut cfirst: size_t,
    mut zsecond: *const libc::c_char,
    mut csecond: size_t,
) -> libc::c_ushort {
    let mut ichk: libc::c_ushort = 0;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: size_t = 0;
    z = zfirst;
    c = cfirst.wrapping_add(csecond);
    ichk = 0xffff as libc::c_int as libc::c_ushort;
    loop {
        let fresh3 = c;
        c = c.wrapping_sub(1);
        if !(fresh3 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        ichk = (ichk as libc::c_int
            + (ichk as libc::c_int
                + ((ichk as libc::c_int & 0x8000 as libc::c_int) >> 15 as libc::c_int)))
            as libc::c_ushort;
        let fresh4 = z;
        z = z.offset(1);
        ichk = (ichk as libc::c_int + *fresh4 as libc::c_uchar as libc::c_int)
            as libc::c_ushort;
        cfirst = cfirst.wrapping_sub(1);
        if cfirst == 0 as libc::c_int as libc::c_ulong {
            z = zsecond;
        }
    }
    return ichk;
}
