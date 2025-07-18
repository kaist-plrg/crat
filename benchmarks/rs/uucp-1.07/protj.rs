use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn cescape(zbuf: *mut libc::c_char) -> size_t;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn fconn_set(
        qconn: *mut sconnection,
        tparity: tparitysetting,
        tstrip: tstripsetting,
        txonxoff: txonxoffsetting,
    ) -> boolean;
    fn usysdep_sleep(cseconds: libc::c_int);
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
    fn breceive_char(
        qconn: *mut sconnection,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> libc::c_int;
    static mut abPrecbuf: [libc::c_char; 16384];
    static mut iPrecstart: libc::c_int;
    static mut iPrecend: libc::c_int;
    static mut zJavoid_parameter: *const libc::c_char;
    static mut cIsync_timeout: libc::c_int;
    fn fijstart(
        qdaemon: *mut sdaemon,
        pzlog: *mut *mut libc::c_char,
        imaxpacksize: libc::c_int,
        pfsend: Option::<
            unsafe extern "C" fn(
                *mut sconnection,
                *const libc::c_char,
                size_t,
                boolean,
            ) -> boolean,
        >,
        pfreceive: Option::<
            unsafe extern "C" fn(
                *mut sconnection,
                size_t,
                *mut size_t,
                libc::c_int,
                boolean,
            ) -> boolean,
        >,
    ) -> boolean;
    fn fishutdown(qdaemon: *mut sdaemon) -> boolean;
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
pub static mut protj_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: protj.c,v 1.9 2002/03/05 19:10:41 ian Rel $\0")
};
static mut zJavoid: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut cJavoid: size_t = 0;
static mut zJbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut iJrecend: libc::c_int = 0;
pub unsafe extern "C" fn fjstart(
    mut qdaemon: *mut sdaemon,
    mut pzlog: *mut *mut libc::c_char,
) -> boolean {
    let mut clen: size_t = 0;
    let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: libc::c_int = 0;
    let mut cbuf: size_t = 0;
    let mut cgot: size_t = 0;
    let mut zbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    clen = strlen(zJavoid_parameter);
    zsend = zbufalc(clen.wrapping_add(3 as libc::c_int as libc::c_ulong));
    *zsend.offset(0 as libc::c_int as isize) = '^' as i32 as libc::c_char;
    memcpy(
        zsend.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        zJavoid_parameter as *const libc::c_void,
        clen,
    );
    *zsend
        .offset(
            clen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '~' as i32 as libc::c_char;
    *zsend
        .offset(
            clen.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    if fsend_data(
        (*qdaemon).qconn,
        zsend,
        clen.wrapping_add(2 as libc::c_int as libc::c_ulong),
        1 as libc::c_int,
    ) == 0
    {
        ubuffree(zsend);
        return 0 as libc::c_int;
    }
    ubuffree(zsend);
    loop {
        b = breceive_char((*qdaemon).qconn, cIsync_timeout, 1 as libc::c_int);
        if !(b != '^' as i32) {
            break;
        }
        if b < 0 as libc::c_int {
            if b == -(1 as libc::c_int) {
                ulog(
                    LOG_ERROR,
                    b"Timed out in 'j' protocol startup\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int;
        }
    }
    cbuf = 20 as libc::c_int as size_t;
    zbuf = zbufalc(cbuf);
    cgot = 0 as libc::c_int as size_t;
    loop {
        b = breceive_char((*qdaemon).qconn, cIsync_timeout, 1 as libc::c_int);
        if !(b != '~' as i32) {
            break;
        }
        if b < 0 as libc::c_int {
            ubuffree(zbuf);
            if b == -(1 as libc::c_int) {
                ulog(
                    LOG_ERROR,
                    b"Timed out in 'j' protocol startup\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int;
        }
        if cgot.wrapping_add(1 as libc::c_int as libc::c_ulong) >= cbuf {
            let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
            cbuf = (cbuf as libc::c_ulong)
                .wrapping_add(20 as libc::c_int as libc::c_ulong) as size_t as size_t;
            znew = zbufalc(cbuf);
            memcpy(znew as *mut libc::c_void, zbuf as *const libc::c_void, cgot);
            ubuffree(zbuf);
            zbuf = znew;
        }
        *zbuf.offset(cgot as isize) = b as libc::c_char;
        cgot = cgot.wrapping_add(1);
        cgot;
    }
    *zbuf.offset(cgot as isize) = '\0' as i32 as libc::c_char;
    cgot = cescape(zbuf);
    clen = strlen(zJavoid_parameter);
    zJavoid = zbufalc(
        clen.wrapping_add(cgot).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        zJavoid as *mut libc::c_void,
        zJavoid_parameter as *const libc::c_void,
        clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    cJavoid = cescape(zJavoid);
    i = 0 as libc::c_int as size_t;
    while i < cgot {
        if (memchr(
            zJavoid as *const libc::c_void,
            *zbuf.offset(i as isize) as libc::c_int,
            cJavoid,
        ))
            .is_null()
        {
            *zJavoid.offset(cJavoid as isize) = *zbuf.offset(i as isize);
            cJavoid = cJavoid.wrapping_add(1);
            cJavoid;
        }
        i = i.wrapping_add(1);
        i;
    }
    ubuffree(zbuf);
    if cJavoid == 0 as libc::c_int as libc::c_ulong {
        ulog(
            LOG_ERROR,
            b"No characters to avoid in 'j' protocol\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < cJavoid {
        if *zJavoid.offset(i as isize) as libc::c_int >= 32 as libc::c_int
            && *zJavoid.offset(i as isize) as libc::c_int <= 126 as libc::c_int
        {
            ulog(
                LOG_ERROR,
                b"'j' protocol can't avoid character '\\%03o'\0" as *const u8
                    as *const libc::c_char,
                *zJavoid.offset(i as isize) as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !(memchr(zJavoid as *const libc::c_void, '\u{11}' as i32, cJavoid)).is_null()
        && !(memchr(zJavoid as *const libc::c_void, '\u{13}' as i32, cJavoid)).is_null()
    {
        if fconn_set(
            (*qdaemon).qconn,
            PARITYSETTING_NONE,
            STRIPSETTING_EIGHTBITS,
            XONXOFF_ON,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    usysdep_sleep(2 as libc::c_int);
    zJbuf = zbufalc(
        (7 as libc::c_int
            + ((125 as libc::c_int - 32 as libc::c_int) * 32 as libc::c_int
                + 31 as libc::c_int) * 3 as libc::c_int + 1 as libc::c_int) as size_t,
    );
    *zJbuf.offset(0 as libc::c_int as isize) = '^' as i32 as libc::c_char;
    *zJbuf.offset(3 as libc::c_int as isize) = '=' as i32 as libc::c_char;
    *zJbuf.offset(6 as libc::c_int as isize) = '@' as i32 as libc::c_char;
    iJrecend = iPrecend;
    iPrecend = iPrecstart;
    return fijstart(
        qdaemon,
        pzlog,
        (125 as libc::c_int - 32 as libc::c_int) * 32 as libc::c_int + 31 as libc::c_int,
        Some(
            fjsend_data
                as unsafe extern "C" fn(
                    *mut sconnection,
                    *const libc::c_char,
                    size_t,
                    boolean,
                ) -> boolean,
        ),
        Some(
            fjreceive_data
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
pub unsafe extern "C" fn fjshutdown(mut qdaemon: *mut sdaemon) -> boolean {
    let mut fret: boolean = 0;
    fret = fishutdown(qdaemon);
    ubuffree(zJavoid);
    ubuffree(zJbuf);
    return fret;
}
unsafe extern "C" fn fjsend_data(
    mut qconn: *mut sconnection,
    mut zsend: *const libc::c_char,
    mut csend: size_t,
    mut fdoread: boolean,
) -> boolean {
    let mut zput: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zindex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
    let mut zend: *const libc::c_char = 0 as *const libc::c_char;
    let mut bfirst: libc::c_char = 0;
    let mut bsecond: libc::c_char = 0;
    let mut iprecendhold: libc::c_int = 0;
    let mut fret: boolean = 0;
    zput = zJbuf.offset(7 as libc::c_int as isize);
    zindex = zput.offset(csend as isize);
    zfrom = zsend;
    zend = zsend.offset(csend as isize);
    bfirst = *zJavoid.offset(0 as libc::c_int as isize);
    if cJavoid <= 1 as libc::c_int as libc::c_ulong {
        bsecond = bfirst;
    } else {
        bsecond = *zJavoid.offset(1 as libc::c_int as isize);
    }
    while zfrom < zend {
        let mut b: libc::c_char = 0;
        let mut f128: boolean = 0;
        let mut f32: boolean = 0;
        let mut i: libc::c_int = 0;
        let mut ihigh: libc::c_int = 0;
        let mut ilow: libc::c_int = 0;
        let fresh0 = zfrom;
        zfrom = zfrom.offset(1);
        b = *fresh0;
        if b as libc::c_int != bfirst as libc::c_int
            && b as libc::c_int != bsecond as libc::c_int
        {
            let mut ca: libc::c_int = 0;
            let mut za: *mut libc::c_char = 0 as *mut libc::c_char;
            if cJavoid <= 2 as libc::c_int as libc::c_ulong {
                let fresh1 = zput;
                zput = zput.offset(1);
                *fresh1 = b;
                continue;
            } else {
                ca = cJavoid.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    as libc::c_int;
                za = zJavoid.offset(2 as libc::c_int as isize);
                loop {
                    let fresh2 = ca;
                    ca = ca - 1;
                    if !(fresh2 != 0 as libc::c_int) {
                        break;
                    }
                    let fresh3 = za;
                    za = za.offset(1);
                    if *fresh3 as libc::c_int == b as libc::c_int {
                        break;
                    }
                }
                if ca < 0 as libc::c_int {
                    let fresh4 = zput;
                    zput = zput.offset(1);
                    *fresh4 = b;
                    continue;
                }
            }
        }
        if b as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            f128 = 0 as libc::c_int;
        } else {
            b = (b as libc::c_int & !(0x80 as libc::c_int)) as libc::c_char;
            f128 = 1 as libc::c_int;
        }
        if b as libc::c_int >= 32 as libc::c_int
            && b as libc::c_int != 127 as libc::c_int
        {
            f32 = 0 as libc::c_int;
        } else {
            b = (b as libc::c_int ^ 0x20 as libc::c_int) as libc::c_char;
            f32 = 1 as libc::c_int;
        }
        i = zput.offset_from(zJbuf.offset(7 as libc::c_int as isize)) as libc::c_long
            as libc::c_int;
        ihigh = i / 32 as libc::c_int;
        ilow = i % 32 as libc::c_int;
        if !(f128 != 0 && f32 == 0) {
            if f32 != 0 && f128 == 0 {
                ilow += 32 as libc::c_int;
            } else if ilow != 32 as libc::c_int - 1 as libc::c_int {
                ilow += 2 as libc::c_int * 32 as libc::c_int;
            } else {
                ilow = ihigh;
                ihigh = 94 as libc::c_int;
            }
        }
        let fresh5 = zindex;
        zindex = zindex.offset(1);
        *fresh5 = (ihigh + 32 as libc::c_int) as libc::c_char;
        let fresh6 = zindex;
        zindex = zindex.offset(1);
        *fresh6 = (ilow + 32 as libc::c_int) as libc::c_char;
        let fresh7 = zput;
        zput = zput.offset(1);
        *fresh7 = b;
    }
    let fresh8 = zindex;
    zindex = zindex.offset(1);
    *fresh8 = '~' as i32 as libc::c_char;
    *zJbuf
        .offset(
            1 as libc::c_int as isize,
        ) = (zindex.offset_from(zJbuf) as libc::c_long
        / 64 as libc::c_int as libc::c_long + 32 as libc::c_int as libc::c_long)
        as libc::c_char;
    *zJbuf
        .offset(
            2 as libc::c_int as isize,
        ) = (zindex.offset_from(zJbuf) as libc::c_long
        % 64 as libc::c_int as libc::c_long + 32 as libc::c_int as libc::c_long)
        as libc::c_char;
    *zJbuf
        .offset(
            4 as libc::c_int as isize,
        ) = csend
        .wrapping_div(64 as libc::c_int as libc::c_ulong)
        .wrapping_add(32 as libc::c_int as libc::c_ulong) as libc::c_char;
    *zJbuf
        .offset(
            5 as libc::c_int as isize,
        ) = csend
        .wrapping_rem(64 as libc::c_int as libc::c_ulong)
        .wrapping_add(32 as libc::c_int as libc::c_ulong) as libc::c_char;
    iprecendhold = iPrecend;
    iPrecend = iJrecend;
    fret = fsend_data(
        qconn,
        zJbuf,
        zindex.offset_from(zJbuf) as libc::c_long as size_t,
        fdoread,
    );
    iJrecend = iPrecend;
    iPrecend = iprecendhold;
    if fret != 0 && iPrecend != iJrecend {
        if fjprocess_data(0 as *mut libc::c_void as *mut size_t) == 0 {
            return 0 as libc::c_int;
        }
    }
    return fret;
}
unsafe extern "C" fn fjreceive_data(
    mut qconn: *mut sconnection,
    mut cineed: size_t,
    mut pcrec: *mut size_t,
    mut ctimeout: libc::c_int,
    mut freport: boolean,
) -> boolean {
    let mut iprecendstart: libc::c_int = 0;
    let mut cjneed: size_t = 0;
    let mut crec: size_t = 0;
    let mut cnew: libc::c_int = 0;
    iprecendstart = iPrecend;
    if fjprocess_data(&mut cjneed) == 0 {
        return 0 as libc::c_int;
    }
    loop {
        let mut iprecendhold: libc::c_int = 0;
        let mut cneed: size_t = 0;
        if cjneed > cineed {
            cneed = cjneed;
        } else {
            cneed = cineed;
        }
        iprecendhold = iPrecend;
        iPrecend = iJrecend;
        if freceive_data(qconn, cneed, &mut crec, ctimeout, freport) == 0 {
            return 0 as libc::c_int;
        }
        iJrecend = iPrecend;
        iPrecend = iprecendhold;
        if fjprocess_data(&mut cjneed) == 0 {
            return 0 as libc::c_int;
        }
        cnew = iPrecend - iprecendstart;
        if cnew < 0 as libc::c_int {
            cnew += 16384 as libc::c_int;
        }
        if cnew as size_t > cineed {
            cineed = 0 as libc::c_int as size_t;
        } else {
            cineed = (cineed as libc::c_ulong).wrapping_sub(cnew as libc::c_ulong)
                as size_t as size_t;
        }
        ctimeout -= 1;
        ctimeout;
        if !(cnew == 0 as libc::c_int && crec > 0 as libc::c_int as libc::c_ulong
            && ctimeout > 0 as libc::c_int)
        {
            break;
        }
    }
    if iDebug & 0o20 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fjreceive_data: Got %d decoded bytes\0" as *const u8
                as *const libc::c_char,
            cnew,
        );
    }
    *pcrec = cnew as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn fjprocess_data(mut pcneed: *mut size_t) -> boolean {
    let mut istart: libc::c_int = 0;
    istart = iPrecend;
    while istart != iJrecend {
        let mut i: libc::c_int = 0;
        let mut iget: libc::c_int = 0;
        let mut ab: [libc::c_char; 7] = [0; 7];
        let mut cpacket: libc::c_int = 0;
        let mut cdata: libc::c_int = 0;
        let mut chave: libc::c_int = 0;
        let mut iindex: libc::c_int = 0;
        let mut iendindex: libc::c_int = 0;
        if abPrecbuf[istart as usize] as libc::c_int != '^' as i32 {
            let mut cintro: libc::c_int = 0;
            let mut zintro: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cskipped: size_t = 0;
            cintro = iJrecend - istart;
            if cintro < 0 as libc::c_int {
                cintro = 16384 as libc::c_int - istart;
            }
            zintro = memchr(
                abPrecbuf.as_mut_ptr().offset(istart as isize) as *const libc::c_void,
                '^' as i32,
                cintro as size_t,
            ) as *mut libc::c_char;
            if zintro.is_null() {
                bzero(
                    abPrecbuf.as_mut_ptr().offset(istart as isize) as *mut libc::c_void,
                    cintro as size_t,
                );
                istart = (istart + cintro) % 16384 as libc::c_int;
                iPrecend = istart;
                continue;
            } else {
                cskipped = zintro
                    .offset_from(abPrecbuf.as_mut_ptr().offset(istart as isize))
                    as libc::c_long as size_t;
                bzero(
                    abPrecbuf.as_mut_ptr().offset(istart as isize) as *mut libc::c_void,
                    cskipped,
                );
                istart = (istart as libc::c_ulong).wrapping_add(cskipped) as libc::c_int
                    as libc::c_int;
                iPrecend = istart;
            }
        }
        i = 0 as libc::c_int;
        iget = istart;
        while i < 7 as libc::c_int && iget != iJrecend {
            ab[i as usize] = abPrecbuf[iget as usize];
            i += 1;
            i;
            iget = (iget + 1 as libc::c_int) % 16384 as libc::c_int;
        }
        if i < 7 as libc::c_int {
            if !pcneed.is_null() {
                *pcneed = (7 as libc::c_int - i) as size_t;
            }
            return 1 as libc::c_int;
        }
        cpacket = (ab[1 as libc::c_int as usize] as libc::c_int - 32 as libc::c_int)
            * 64 as libc::c_int
            + (ab[2 as libc::c_int as usize] as libc::c_int - 32 as libc::c_int);
        cdata = (ab[4 as libc::c_int as usize] as libc::c_int - 32 as libc::c_int)
            * 64 as libc::c_int
            + (ab[5 as libc::c_int as usize] as libc::c_int - 32 as libc::c_int);
        if ab[3 as libc::c_int as usize] as libc::c_int != '=' as i32
            || ab[6 as libc::c_int as usize] as libc::c_int != '@' as i32
            || cdata > cpacket - 7 as libc::c_int - 1 as libc::c_int
            || (cpacket - cdata - 7 as libc::c_int - 1 as libc::c_int) % 2 as libc::c_int
                == 1 as libc::c_int
        {
            istart = (istart + 1 as libc::c_int) % 16384 as libc::c_int;
        } else {
            chave = iJrecend - istart;
            if chave < 0 as libc::c_int {
                chave += 16384 as libc::c_int;
            }
            if chave < cpacket {
                if !pcneed.is_null() {
                    *pcneed = (cpacket - chave) as size_t;
                }
                return 1 as libc::c_int;
            }
            iindex = (istart + 7 as libc::c_int + cdata) % 16384 as libc::c_int;
            iendindex = (istart + cpacket - 1 as libc::c_int) % 16384 as libc::c_int;
            if abPrecbuf[iendindex as usize] as libc::c_int != '~' as i32 {
                istart = (istart + 1 as libc::c_int) % 16384 as libc::c_int;
            } else {
                while iindex != iendindex {
                    let mut ihigh: libc::c_int = 0;
                    let mut ilow: libc::c_int = 0;
                    let mut f32: boolean = 0;
                    let mut f128: boolean = 0;
                    let mut iset: libc::c_int = 0;
                    ihigh = abPrecbuf[iindex as usize] as libc::c_int
                        - 32 as libc::c_int;
                    abPrecbuf[iindex as usize] = 0 as libc::c_int as libc::c_char;
                    iindex = (iindex + 1 as libc::c_int) % 16384 as libc::c_int;
                    ilow = abPrecbuf[iindex as usize] as libc::c_int - 32 as libc::c_int;
                    abPrecbuf[iindex as usize] = 0 as libc::c_int as libc::c_char;
                    iindex = (iindex + 1 as libc::c_int) % 16384 as libc::c_int;
                    f128 = 1 as libc::c_int;
                    f32 = 1 as libc::c_int;
                    if ihigh == 94 as libc::c_int {
                        iset = ilow * 32 as libc::c_int + 32 as libc::c_int
                            - 1 as libc::c_int;
                    } else {
                        iset = ihigh * 32 as libc::c_int + ilow % 32 as libc::c_int;
                        if ilow < 32 as libc::c_int {
                            f32 = 0 as libc::c_int;
                        } else if ilow < 2 as libc::c_int * 32 as libc::c_int {
                            f128 = 0 as libc::c_int;
                        }
                    }
                    iset = (istart + 7 as libc::c_int + iset) % 16384 as libc::c_int;
                    if f128 != 0 {
                        abPrecbuf[iset
                            as usize] = (abPrecbuf[iset as usize] as libc::c_int
                            | 0x80 as libc::c_int) as libc::c_char;
                    }
                    if f32 != 0 {
                        abPrecbuf[iset
                            as usize] = (abPrecbuf[iset as usize] as libc::c_int
                            ^ 0x20 as libc::c_int) as libc::c_char;
                    }
                }
                i = 0 as libc::c_int;
                iget = istart;
                while i < 7 as libc::c_int && iget != iJrecend {
                    abPrecbuf[iget as usize] = 0 as libc::c_int as libc::c_char;
                    i += 1;
                    i;
                    iget = (iget + 1 as libc::c_int) % 16384 as libc::c_int;
                }
                abPrecbuf[iendindex as usize] = 0 as libc::c_int as libc::c_char;
                iPrecend = (iendindex + 1 as libc::c_int) % 16384 as libc::c_int;
                istart = iPrecend;
            }
        }
    }
    if !pcneed.is_null() {
        *pcneed = (7 as libc::c_int + 1 as libc::c_int) as size_t;
    }
    return 1 as libc::c_int;
}
