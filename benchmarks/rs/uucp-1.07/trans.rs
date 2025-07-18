use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sconnection;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut iDebug: libc::c_int;
    fn ftimespan_match(
        qspan: *const uuconf_timespan,
        pival: *mut libc::c_long,
        pcretry: *mut libc::c_int,
    ) -> boolean;
    fn fparse_cmd(zcmd: *mut libc::c_char, qcmd: *mut scmd) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_user(zuser: *const libc::c_char);
    fn ulog_close();
    fn ustats(
        fsucceeded: boolean,
        zuser: *const libc::c_char,
        zsystem: *const libc::c_char,
        fsent: boolean,
        cbytes: libc::c_long,
        csecs: libc::c_long,
        cmicros: libc::c_long,
        fcaller: boolean,
    );
    fn ustats_close();
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    static mut fLog_sighup: boolean;
    fn uuconf_grade_cmp(uuconf_b1: libc::c_int, uuconf_b2: libc::c_int) -> libc::c_int;
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn ixsysdep_process_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn fsysdep_get_work_init(
        qsys: *const uuconf_system,
        bgrade: libc::c_int,
        cmax: libc::c_uint,
    ) -> boolean;
    fn fsysdep_get_work(
        qsys: *const uuconf_system,
        bgrade: libc::c_int,
        cmax: libc::c_uint,
        qcmd: *mut scmd,
    ) -> boolean;
    fn fsysdep_did_work(pseq: pointer) -> boolean;
    fn usysdep_get_work_free(qsys: *const uuconf_system);
    fn fsysdep_forget_reception(
        qsys: *const uuconf_system,
        zto: *const libc::c_char,
        ztemp: *const libc::c_char,
    ) -> boolean;
    fn flocal_send_file_init(qdaemon: *mut sdaemon, qcmd: *mut scmd) -> boolean;
    fn fremote_send_file_init(
        qdaemon: *mut sdaemon,
        qcmd: *mut scmd,
        iremote: libc::c_int,
    ) -> boolean;
    fn flocal_rec_file_init(qdaemon: *mut sdaemon, qcmd: *mut scmd) -> boolean;
    fn fremote_rec_file_init(
        qdaemon: *mut sdaemon,
        qcmd: *mut scmd,
        iremote: libc::c_int,
    ) -> boolean;
    fn flocal_xcmd_init(qdaemon: *mut sdaemon, qcmd: *mut scmd) -> boolean;
    fn fremote_xcmd_init(
        qdaemon: *mut sdaemon,
        qcmd: *mut scmd,
        iremote: libc::c_int,
    ) -> boolean;
    fn frec_check_free(qtrans: *mut stransfer, cfree_space: libc::c_long) -> boolean;
    fn frec_discard_temp(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
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
pub struct sreceive_ack {
    pub qnext: *mut sreceive_ack,
    pub zto: *mut libc::c_char,
    pub ztemp: *mut libc::c_char,
    pub fmarked: boolean,
}
pub static mut trans_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: trans.c,v 1.49 2002/03/05 19:10:41 ian Rel $\0")
};
static mut qTlocal: *mut stransfer = 0 as *const stransfer as *mut stransfer;
static mut qTremote: *mut stransfer = 0 as *const stransfer as *mut stransfer;
pub static mut qTsend: *mut stransfer = 0 as *const stransfer as *mut stransfer;
static mut qTreceive: *mut stransfer = 0 as *const stransfer as *mut stransfer;
static mut qTavail: *mut stransfer = 0 as *const stransfer as *mut stransfer;
static mut aqTchan: [*mut stransfer; 17] = [0 as *const stransfer as *mut stransfer; 17];
static mut cTchans: libc::c_int = 0;
static mut iTchan: libc::c_int = 0;
static mut aqTremote: [*mut stransfer; 17] = [0 as *const stransfer
    as *mut stransfer; 17];
static mut qTtiming_rec: *mut stransfer = 0 as *const stransfer as *mut stransfer;
static mut iTrecsecs: libc::c_long = 0;
static mut iTrecmicros: libc::c_long = 0;
static mut iTchecktime: libc::c_long = 0;
static mut cTcmdlen: size_t = 0;
static mut qTreceive_ack: *mut sreceive_ack = 0 as *const sreceive_ack
    as *mut sreceive_ack;
unsafe extern "C" fn utqueue(
    mut pq: *mut *mut stransfer,
    mut q: *mut stransfer,
    mut fhead: boolean,
) {
    if (*pq).is_null() {
        *pq = q;
        (*q).qnext = q;
        (*q).qprev = (*q).qnext;
    } else {
        (*q).qnext = *pq;
        (*q).qprev = (**pq).qprev;
        (*(*q).qprev).qnext = q;
        (*(*q).qnext).qprev = q;
        if fhead != 0 {
            *pq = q;
        }
    }
    (*q).pqqueue = pq;
}
unsafe extern "C" fn utdequeue(mut q: *mut stransfer) {
    if !((*q).pqqueue).is_null() {
        if *(*q).pqqueue == q {
            if (*q).qnext == q {
                *(*q).pqqueue = 0 as *mut stransfer;
            } else {
                *(*q).pqqueue = (*q).qnext;
            }
        }
        (*q).pqqueue = 0 as *mut *mut stransfer;
    }
    if !((*q).qprev).is_null() {
        (*(*q).qprev).qnext = (*q).qnext;
    }
    if !((*q).qnext).is_null() {
        (*(*q).qnext).qprev = (*q).qprev;
    }
    (*q).qprev = 0 as *mut stransfer;
    (*q).qnext = 0 as *mut stransfer;
}
pub unsafe extern "C" fn fqueue_local(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
) -> boolean {
    utdequeue(qtrans);
    utqueue(&mut qTlocal, qtrans, 0 as libc::c_int);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fqueue_remote(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
) -> boolean {
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fqueue_remote: Channel %d\0" as *const u8 as *const libc::c_char,
            (*qtrans).iremote,
        );
    }
    if (*qtrans).iremote > 0 as libc::c_int {
        aqTremote[(*qtrans).iremote as usize] = qtrans;
    }
    utdequeue(qtrans);
    utqueue(&mut qTremote, qtrans, 0 as libc::c_int);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fqueue_send(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
) -> boolean {
    if ((*qtrans).psendfn).is_none() {
        ulog(LOG_FATAL, b"fqueue_send: Bad call\0" as *const u8 as *const libc::c_char);
    }
    utdequeue(qtrans);
    if qTsend.is_null() {
        utqueue(&mut qTsend, qtrans, 0 as libc::c_int);
    } else {
        let mut q: *mut stransfer = 0 as *mut stransfer;
        let mut ffirst: boolean = 0;
        ffirst = 1 as libc::c_int;
        q = qTsend;
        while !((*qtrans).fsendfile == 0 && (*q).fsendfile != 0) {
            if ((*qtrans).fsendfile == 0 || (*q).fsendfile != 0)
                && uuconf_grade_cmp(
                    (*qtrans).s.bgrade as libc::c_int,
                    (*q).s.bgrade as libc::c_int,
                ) < 0 as libc::c_int
            {
                break;
            }
            ffirst = 0 as libc::c_int;
            q = (*q).qnext;
            if !(q != qTsend) {
                break;
            }
        }
        (*qtrans).qnext = q;
        (*qtrans).qprev = (*q).qprev;
        (*q).qprev = qtrans;
        (*(*qtrans).qprev).qnext = qtrans;
        if ffirst != 0 {
            qTsend = qtrans;
        }
        (*qtrans).pqqueue = &mut qTsend;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fqueue_receive(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
) -> boolean {
    if ((*qtrans).precfn).is_none() {
        ulog(
            LOG_FATAL,
            b"fqueue_receive: Bad call\0" as *const u8 as *const libc::c_char,
        );
    }
    if qTreceive.is_null() {
        iTrecsecs = ixsysdep_process_time(&mut iTrecmicros);
    }
    utdequeue(qtrans);
    utqueue(&mut qTreceive, qtrans, 0 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn utchanalc(mut qdaemon: *mut sdaemon, mut qtrans: *mut stransfer) {
    loop {
        iTchan += 1;
        iTchan;
        if iTchan > (*qdaemon).cchans {
            iTchan = 1 as libc::c_int;
        }
        if (aqTchan[iTchan as usize]).is_null() {
            break;
        }
    }
    (*qtrans).ilocal = iTchan;
    aqTchan[iTchan as usize] = qtrans;
    cTchans += 1;
    cTchans;
}
#[inline]
unsafe extern "C" fn qtchan(mut ic: libc::c_int) -> *mut stransfer {
    return aqTchan[ic as usize];
}
#[inline]
unsafe extern "C" fn utchanfree(mut qt: *mut stransfer) {
    if (*qt).ilocal != 0 as libc::c_int {
        aqTchan[(*qt).ilocal as usize] = 0 as *mut stransfer;
        (*qt).ilocal = 0 as libc::c_int;
        cTchans -= 1;
        cTchans;
    }
}
pub unsafe extern "C" fn qtransalc(mut qcmd: *mut scmd) -> *mut stransfer {
    let mut q: *mut stransfer = 0 as *mut stransfer;
    q = qTavail;
    if !q.is_null() {
        utdequeue(q);
    } else {
        q = xmalloc(::std::mem::size_of::<stransfer>() as libc::c_ulong)
            as *mut stransfer;
    }
    (*q).qnext = 0 as *mut stransfer;
    (*q).qprev = 0 as *mut stransfer;
    (*q).pqqueue = 0 as *mut *mut stransfer;
    (*q).psendfn = None;
    (*q).precfn = None;
    (*q).pinfo = 0 as *mut libc::c_void;
    (*q).fsendfile = 0 as libc::c_int;
    (*q).frecfile = 0 as libc::c_int;
    (*q).e = 0 as *mut libc::c_void as *mut FILE;
    (*q).ipos = 0 as libc::c_int as libc::c_long;
    (*q).fcmd = 0 as libc::c_int;
    (*q).zcmd = 0 as *mut libc::c_char;
    (*q).ccmd = 0 as libc::c_int as size_t;
    (*q).ilocal = 0 as libc::c_int;
    (*q).iremote = 0 as libc::c_int;
    if !qcmd.is_null() {
        (*q).s = *qcmd;
        (*q).s.zfrom = zbufcpy((*qcmd).zfrom);
        (*q).s.zto = zbufcpy((*qcmd).zto);
        (*q).s.zuser = zbufcpy((*qcmd).zuser);
        (*q).s.zoptions = zbufcpy((*qcmd).zoptions);
        (*q).s.ztemp = zbufcpy((*qcmd).ztemp);
        (*q).s.znotify = zbufcpy((*qcmd).znotify);
        (*q).s.zcmd = zbufcpy((*qcmd).zcmd);
    } else {
        (*q).s.zfrom = 0 as *const libc::c_char;
        (*q).s.zto = 0 as *const libc::c_char;
        (*q).s.zuser = 0 as *const libc::c_char;
        (*q).s.zoptions = 0 as *const libc::c_char;
        (*q).s.ztemp = 0 as *const libc::c_char;
        (*q).s.znotify = 0 as *const libc::c_char;
        (*q).s.zcmd = 0 as *const libc::c_char;
    }
    (*q).zlog = 0 as *mut libc::c_char;
    (*q).isecs = 0 as libc::c_int as libc::c_long;
    (*q).imicros = 0 as libc::c_int as libc::c_long;
    (*q).cbytes = 0 as libc::c_int as libc::c_long;
    return q;
}
pub unsafe extern "C" fn utransfree(mut q: *mut stransfer) {
    ubuffree((*q).zcmd);
    ubuffree((*q).s.zfrom as *mut libc::c_char);
    ubuffree((*q).s.zto as *mut libc::c_char);
    ubuffree((*q).s.zuser as *mut libc::c_char);
    ubuffree((*q).s.zoptions as *mut libc::c_char);
    ubuffree((*q).s.ztemp as *mut libc::c_char);
    ubuffree((*q).s.znotify as *mut libc::c_char);
    ubuffree((*q).s.zcmd as *mut libc::c_char);
    utchanfree(q);
    if (*q).iremote > 0 as libc::c_int {
        aqTremote[(*q).iremote as usize] = 0 as *mut stransfer;
        (*q).iremote = 0 as libc::c_int;
    }
    if !((*q).e).is_null() {
        fclose((*q).e);
        (*q).e = 0 as *mut libc::c_void as *mut FILE;
    }
    (*q).zcmd = 0 as *mut libc::c_char;
    (*q).s.zfrom = 0 as *const libc::c_char;
    (*q).s.zto = 0 as *const libc::c_char;
    (*q).s.zuser = 0 as *const libc::c_char;
    (*q).s.zoptions = 0 as *const libc::c_char;
    (*q).s.ztemp = 0 as *const libc::c_char;
    (*q).s.znotify = 0 as *const libc::c_char;
    (*q).s.zcmd = 0 as *const libc::c_char;
    (*q).psendfn = None;
    (*q).precfn = None;
    if qTtiming_rec == q {
        qTtiming_rec = 0 as *mut stransfer;
    }
    utdequeue(q);
    utqueue(&mut qTavail, q, 0 as libc::c_int);
}
unsafe extern "C" fn utfree_queue(mut pq: *mut *mut stransfer) {
    while !(*pq).is_null() {
        utransfree(*pq);
    }
}
unsafe extern "C" fn fttime(
    mut qdaemon: *mut sdaemon,
    mut pisecs: *mut libc::c_long,
    mut pimicros: *mut libc::c_long,
) -> boolean {
    *pisecs = ixsysdep_process_time(pimicros);
    if *pisecs - iTchecktime >= 600 as libc::c_int as libc::c_long {
        if fcheck_queue(qdaemon) == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fqueue(
    mut qdaemon: *mut sdaemon,
    mut pfany: *mut boolean,
) -> boolean {
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut ival: libc::c_long = 0;
    let mut bgrade: libc::c_int = 0;
    let mut qlocal_size: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
    let mut qremote_size: *mut uuconf_timespan = 0 as *mut uuconf_timespan;
    if !pfany.is_null() {
        *pfany = 0 as libc::c_int;
    }
    qsys = (*qdaemon).qsys;
    if (*qdaemon).fcaller == 0 {
        if ftimespan_match(
            (*qsys).uuconf_qcalledtimegrade,
            &mut ival,
            0 as *mut libc::c_void as *mut libc::c_int,
        ) == 0
        {
            bgrade = (*qdaemon).bgrade as libc::c_int;
        } else {
            bgrade = ival as libc::c_char as libc::c_int;
        }
    } else if ftimespan_match(
        (*qsys).uuconf_qtimegrade,
        &mut ival,
        0 as *mut libc::c_void as *mut libc::c_int,
    ) == 0
    {
        bgrade = '\0' as i32;
    } else {
        bgrade = ival as libc::c_char as libc::c_int;
    }
    if (*qdaemon).fcaller != 0 {
        qlocal_size = (*qsys).uuconf_qcall_local_size;
        qremote_size = (*qsys).uuconf_qcall_remote_size;
    } else {
        qlocal_size = (*qsys).uuconf_qcalled_local_size;
        qremote_size = (*qsys).uuconf_qcalled_remote_size;
    }
    if ftimespan_match(
        qlocal_size,
        &mut (*qdaemon).clocal_size,
        0 as *mut libc::c_void as *mut libc::c_int,
    ) == 0
    {
        (*qdaemon).clocal_size = -(1 as libc::c_int) as libc::c_long;
    }
    if ftimespan_match(
        qremote_size,
        &mut (*qdaemon).cremote_size,
        0 as *mut libc::c_void as *mut libc::c_int,
    ) == 0
    {
        (*qdaemon).cremote_size = -(1 as libc::c_int) as libc::c_long;
    }
    if bgrade == '\0' as i32 {
        return 1 as libc::c_int;
    }
    if fsysdep_get_work_init(qsys, bgrade, 200 as libc::c_int as libc::c_uint) == 0 {
        return 0 as libc::c_int;
    }
    loop {
        let mut s: scmd = scmd {
            bcmd: 0,
            bgrade: 0,
            pseq: 0 as *mut libc::c_void,
            zfrom: 0 as *const libc::c_char,
            zto: 0 as *const libc::c_char,
            zuser: 0 as *const libc::c_char,
            zoptions: 0 as *const libc::c_char,
            ztemp: 0 as *const libc::c_char,
            imode: 0,
            znotify: 0 as *const libc::c_char,
            cbytes: 0,
            zcmd: 0 as *const libc::c_char,
            ipos: 0,
        };
        if fsysdep_get_work(qsys, bgrade, 200 as libc::c_int as libc::c_uint, &mut s)
            == 0
        {
            return 0 as libc::c_int;
        }
        if s.bcmd as libc::c_int == 'H' as i32 {
            ulog_user(0 as *mut libc::c_void as *const libc::c_char);
            break;
        } else if s.bcmd as libc::c_int == 'P' as i32 {
            let mut qtrans: *mut stransfer = 0 as *mut stransfer;
            ulog_user(0 as *mut libc::c_void as *const libc::c_char);
            qtrans = qtransalc(&mut s);
            (*qtrans)
                .psendfn = Some(
                flocal_poll_file
                    as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
            );
            if fqueue_local(qdaemon, qtrans) == 0 {
                return 0 as libc::c_int;
            }
        } else {
            ulog_user(s.zuser);
            match s.bcmd as libc::c_int {
                83 | 69 => {
                    if flocal_send_file_init(qdaemon, &mut s) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                82 => {
                    if flocal_rec_file_init(qdaemon, &mut s) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                88 => {
                    if flocal_xcmd_init(qdaemon, &mut s) == 0 {
                        return 0 as libc::c_int;
                    }
                }
                _ => {
                    ulog(
                        LOG_FATAL,
                        b"fqueue: Can't happen\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    if !pfany.is_null() {
        *pfany = (qTlocal != 0 as *mut libc::c_void as *mut stransfer) as libc::c_int;
    }
    iTchecktime = ixsysdep_process_time(0 as *mut libc::c_void as *mut libc::c_long);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn uclear_queue(mut qdaemon: *mut sdaemon) {
    let mut i: libc::c_int = 0;
    usysdep_get_work_free((*qdaemon).qsys);
    utfree_queue(&mut qTlocal);
    utfree_queue(&mut qTremote);
    utfree_queue(&mut qTsend);
    utfree_queue(&mut qTreceive);
    cTchans = 0 as libc::c_int;
    iTchan = 0 as libc::c_int;
    qTtiming_rec = 0 as *mut stransfer;
    cTcmdlen = 0 as libc::c_int as size_t;
    if !qTreceive_ack.is_null() {
        utfree_acked();
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int + 1 as libc::c_int {
        aqTchan[i as usize] = 0 as *mut stransfer;
        aqTremote[i as usize] = 0 as *mut stransfer;
        i += 1;
        i;
    }
}
unsafe extern "C" fn fcheck_queue(mut qdaemon: *mut sdaemon) -> boolean {
    if (*qdaemon).fmaster != 0 || (*qdaemon).cchans > 1 as libc::c_int
        || (*qdaemon).frequest_hangup == 0
    {
        let mut fany: boolean = 0;
        if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fcheck_queue: Rechecking work queue\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if fqueue(qdaemon, &mut fany) == 0 {
            return 0 as libc::c_int;
        }
        if fany != 0 && (*qdaemon).fmaster == 0 && (*qdaemon).cchans <= 1 as libc::c_int
        {
            (*qdaemon).frequest_hangup = 1 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn floop(mut qdaemon: *mut sdaemon) -> boolean {
    let mut fret: boolean = 0;
    fret = 1 as libc::c_int;
    while (*qdaemon).fhangup == 0 {
        let mut q: *mut stransfer = 0 as *mut stransfer;
        if iDebug != 0 as libc::c_int {
            ulog_close();
            ustats_close();
        }
        if (*qdaemon).fmaster != 0 {
            let mut fhangup: boolean = 0;
            (*qdaemon).frequest_hangup = 0 as libc::c_int;
            fhangup = 0 as libc::c_int;
            if (*qdaemon).fhangup_requested != 0 && qTsend.is_null()
                && (qTreceive.is_null() || (*qdaemon).cchans > 1 as libc::c_int)
            {
                if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_DEBUG,
                        b"floop: Transferring control at remote request\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                fhangup = 1 as libc::c_int;
            } else if qTremote.is_null() && qTlocal.is_null() && qTsend.is_null()
                && qTreceive.is_null()
            {
                if fqueue(qdaemon, 0 as *mut libc::c_void as *mut boolean) == 0 {
                    fret = 0 as libc::c_int;
                    break;
                } else if qTlocal.is_null() {
                    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                        ulog(
                            LOG_DEBUG,
                            b"floop: No work for master\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    fhangup = 1 as libc::c_int;
                }
            }
            if fhangup != 0 {
                if (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
                    .unwrap()(
                    qdaemon,
                    b"H\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    fret = 0 as libc::c_int;
                    break;
                } else {
                    (*qdaemon).fmaster = 0 as libc::c_int;
                }
            }
        }
        if (*qdaemon).fmaster == 0 {
            (*qdaemon).fhangup_requested = 0 as libc::c_int;
        }
        while !qTremote.is_null() {
            q = qTremote;
            utdequeue(q);
            utqueue(&mut qTsend, q, 1 as libc::c_int);
        }
        if (*qdaemon).fmaster != 0 || (*qdaemon).cchans > 1 as libc::c_int {
            while !qTlocal.is_null() && cTchans < (*qdaemon).cchans {
                q = qTlocal;
                if fqueue_send(qdaemon, q) == 0 {
                    fret = 0 as libc::c_int;
                    break;
                } else {
                    utchanalc(qdaemon, q);
                }
            }
            if fret == 0 {
                break;
            }
        }
        q = qTsend;
        if q.is_null() {
            ulog_user(0 as *mut libc::c_void as *const libc::c_char);
            if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"floop: Waiting for data\0" as *const u8 as *const libc::c_char,
                );
            }
            if !((Some(((*(*qdaemon).qproto).pfwait).unwrap())).unwrap()(qdaemon) == 0) {
                continue;
            }
            fret = 0 as libc::c_int;
            break;
        } else {
            ulog_user((*q).s.zuser);
            if (*q).fsendfile == 0 {
                if !((Some(((*q).psendfn).unwrap())).unwrap()(q, qdaemon) == 0) {
                    continue;
                }
                fret = 0 as libc::c_int;
                break;
            } else {
                let mut isecs: libc::c_long = 0;
                let mut imicros: libc::c_long = 0;
                let mut fcharged: boolean = 0;
                let mut cmax_time: libc::c_long = 0;
                let mut istart: libc::c_long = 0 as libc::c_int as libc::c_long;
                let mut inextsecs: libc::c_long = 0 as libc::c_int as libc::c_long;
                let mut inextmicros: libc::c_long = 0;
                if fttime(qdaemon, &mut isecs, &mut imicros) == 0 {
                    fret = 0 as libc::c_int;
                    break;
                } else {
                    fcharged = 0 as libc::c_int;
                    if !((*q).zlog).is_null() {
                        ulog(
                            LOG_NORMAL,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            (*q).zlog,
                        );
                        ubuffree((*q).zlog);
                        (*q).zlog = 0 as *mut libc::c_char;
                    }
                    cmax_time = (*(*qdaemon).qsys).uuconf_cmax_file_time;
                    if (*qdaemon).cchans <= 1 as libc::c_int {
                        cmax_time = 0 as libc::c_int as libc::c_long;
                    }
                    if cmax_time > 0 as libc::c_int as libc::c_long {
                        istart = ixsysdep_time(0 as *mut libc::c_long);
                    }
                    while q == qTsend && (*q).fsendfile != 0 && qTremote.is_null() {
                        let mut zdata: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut cdata: size_t = 0;
                        let mut ipos: libc::c_long = 0;
                        zdata = (Some(((*(*qdaemon).qproto).pzgetspace).unwrap()))
                            .unwrap()(qdaemon, &mut cdata);
                        if zdata.is_null() {
                            fret = 0 as libc::c_int;
                            break;
                        } else {
                            if feof((*q).e) != 0 {
                                cdata = 0 as libc::c_int as size_t;
                            } else {
                                cdata = fread(
                                    zdata as *mut libc::c_void,
                                    1 as libc::c_int as libc::c_ulong,
                                    cdata,
                                    (*q).e,
                                );
                                if ferror((*q).e) != 0 {
                                    ulog(
                                        LOG_ERROR,
                                        b"read: %s\0" as *const u8 as *const libc::c_char,
                                        strerror(*__errno_location()),
                                    );
                                    fret = 0 as libc::c_int;
                                    break;
                                }
                            }
                            ipos = (*q).ipos;
                            (*q)
                                .ipos = ((*q).ipos as libc::c_ulong).wrapping_add(cdata)
                                as libc::c_long as libc::c_long;
                            (*q)
                                .cbytes = ((*q).cbytes as libc::c_ulong).wrapping_add(cdata)
                                as libc::c_long as libc::c_long;
                            if (Some(((*(*qdaemon).qproto).pfsenddata).unwrap()))
                                .unwrap()(
                                qdaemon,
                                zdata,
                                cdata,
                                (*q).ilocal,
                                (*q).iremote,
                                ipos,
                            ) == 0
                            {
                                fret = 0 as libc::c_int;
                                break;
                            } else if cdata == 0 as libc::c_int as libc::c_ulong {
                                inextsecs = ixsysdep_process_time(&mut inextmicros);
                                if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                                    ulog(
                                        LOG_DEBUG,
                                        b"floop: Charging %ld to %c %s %s\0" as *const u8
                                            as *const libc::c_char,
                                        (inextsecs - isecs) * 1000000 as libc::c_int as libc::c_long
                                            + inextmicros - imicros,
                                        (*q).s.bcmd as libc::c_int,
                                        (*q).s.zfrom,
                                        (*q).s.zto,
                                    );
                                }
                                (*q).isecs += inextsecs - isecs;
                                (*q).imicros += inextmicros - imicros;
                                fcharged = 1 as libc::c_int;
                                (*q).fsendfile = 0 as libc::c_int;
                                if (Some(((*q).psendfn).unwrap())).unwrap()(q, qdaemon) == 0
                                {
                                    fret = 0 as libc::c_int;
                                }
                                break;
                            } else if cmax_time > 0 as libc::c_int as libc::c_long
                                && (*q).qnext != q
                                && ixsysdep_time(0 as *mut libc::c_long) - istart
                                    >= cmax_time
                            {
                                if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                                    ulog(
                                        LOG_DEBUG,
                                        b"floop: Switch file\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                utdequeue(q);
                                utqueue(&mut qTsend, q, 0 as libc::c_int);
                            }
                        }
                    }
                    if fret == 0 {
                        break;
                    }
                    if fcharged == 0 {
                        inextsecs = ixsysdep_process_time(&mut inextmicros);
                        if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                            ulog(
                                LOG_DEBUG,
                                b"floop: Charging %ld to %c %s %s\0" as *const u8
                                    as *const libc::c_char,
                                (inextsecs - isecs) * 1000000 as libc::c_int as libc::c_long
                                    + inextmicros - imicros,
                                (*q).s.bcmd as libc::c_int,
                                (*q).s.zfrom,
                                (*q).s.zto,
                            );
                        }
                        (*q).isecs += inextsecs - isecs;
                        (*q).imicros += inextmicros - imicros;
                    }
                    if !(inextsecs - iTchecktime >= 600 as libc::c_int as libc::c_long) {
                        continue;
                    }
                    if !(fcheck_queue(qdaemon) == 0) {
                        continue;
                    }
                    fret = 0 as libc::c_int;
                    break;
                }
            }
        }
    }
    ulog_user(0 as *mut libc::c_void as *const libc::c_char);
    (Some(((*(*qdaemon).qproto).pfshutdown).unwrap())).unwrap()(qdaemon);
    if fret != 0 {
        uwindow_acked(qdaemon, 1 as libc::c_int);
    } else {
        ufailed(qdaemon);
    }
    return fret;
}
pub unsafe extern "C" fn fgot_data(
    mut qdaemon: *mut sdaemon,
    mut zfirst: *const libc::c_char,
    mut cfirst: size_t,
    mut zsecond: *const libc::c_char,
    mut csecond: size_t,
    mut ilocal: libc::c_int,
    mut iremote: libc::c_int,
    mut ipos: libc::c_long,
    mut fallacked: boolean,
    mut pfexit: *mut boolean,
) -> boolean {
    let mut q: *mut stransfer = 0 as *mut stransfer;
    let mut cwrote: libc::c_int = 0;
    let mut fret: boolean = 0;
    let mut isecs: libc::c_long = 0;
    let mut imicros: libc::c_long = 0;
    if fallacked != 0 && !qTreceive_ack.is_null() {
        uwindow_acked(qdaemon, 1 as libc::c_int);
    }
    if ilocal == -(1 as libc::c_int) && !qTreceive.is_null() {
        q = qTreceive;
    } else if ilocal == 0 as libc::c_int && iremote > 0 as libc::c_int
        && !(aqTremote[iremote as usize]).is_null()
    {
        q = aqTremote[iremote as usize];
    } else if ilocal <= 0 as libc::c_int {
        let mut znull: *const libc::c_char = 0 as *const libc::c_char;
        ulog_user(0 as *mut libc::c_void as *const libc::c_char);
        znull = memchr(zfirst as *const libc::c_void, '\0' as i32, cfirst)
            as *const libc::c_char;
        if !znull.is_null() {
            fret = ftadd_cmd(
                qdaemon,
                zfirst,
                znull.offset_from(zfirst) as libc::c_long as size_t,
                iremote,
                1 as libc::c_int,
            );
        } else {
            fret = ftadd_cmd(qdaemon, zfirst, cfirst, iremote, 0 as libc::c_int);
            if fret != 0 && csecond > 0 as libc::c_int as libc::c_ulong {
                znull = memchr(zsecond as *const libc::c_void, '\0' as i32, csecond)
                    as *const libc::c_char;
                if !znull.is_null() {
                    fret = ftadd_cmd(
                        qdaemon,
                        zsecond,
                        znull.offset_from(zsecond) as libc::c_long as size_t,
                        iremote,
                        1 as libc::c_int,
                    );
                } else {
                    fret = ftadd_cmd(
                        qdaemon,
                        zsecond,
                        csecond,
                        iremote,
                        0 as libc::c_int,
                    );
                }
            }
        }
        if !pfexit.is_null() && ((*qdaemon).fhangup != 0 || !qTremote.is_null()) {
            *pfexit = 1 as libc::c_int;
        }
        if fttime(qdaemon, &mut iTrecsecs, &mut iTrecmicros) == 0 {
            fret = 0 as libc::c_int;
        }
        return fret;
    } else {
        q = qtchan(ilocal);
    }
    if q.is_null() || ((*q).precfn).is_none() {
        ulog(
            LOG_ERROR,
            b"Protocol error: %lu bytes remote %d local %d\0" as *const u8
                as *const libc::c_char,
            cfirst.wrapping_add(csecond),
            iremote,
            ilocal,
        );
        return 0 as libc::c_int;
    }
    ulog_user((*q).s.zuser);
    fret = 1 as libc::c_int;
    if !((*q).zlog).is_null() && (*q).fsendfile == 0 {
        ulog(LOG_NORMAL, b"%s\0" as *const u8 as *const libc::c_char, (*q).zlog);
        ubuffree((*q).zlog);
        (*q).zlog = 0 as *mut libc::c_char;
    }
    if cfirst == 0 as libc::c_int as libc::c_ulong || (*q).fcmd != 0
        || (*q).frecfile == 0 || q != qTtiming_rec
    {
        let mut qcharge: *mut stransfer = 0 as *mut stransfer;
        if fttime(qdaemon, &mut isecs, &mut imicros) == 0 {
            fret = 0 as libc::c_int;
        }
        if !qTtiming_rec.is_null() {
            qcharge = qTtiming_rec;
        } else if !qTsend.is_null() {
            qcharge = 0 as *mut stransfer;
        } else {
            qcharge = q;
        }
        if !qcharge.is_null() {
            if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fgot_data: Charging %ld to %c %s %s\0" as *const u8
                        as *const libc::c_char,
                    (isecs - iTrecsecs) * 1000000 as libc::c_int as libc::c_long
                        + imicros - iTrecmicros,
                    (*qcharge).s.bcmd as libc::c_int,
                    (*qcharge).s.zfrom,
                    (*qcharge).s.zto,
                );
            }
            (*qcharge).isecs += isecs - iTrecsecs;
            (*qcharge).imicros += imicros - iTrecmicros;
        }
        iTrecsecs = isecs;
        iTrecmicros = imicros;
        if cfirst == 0 as libc::c_int as libc::c_ulong || (*q).fcmd != 0
            || (*q).frecfile == 0
        {
            qTtiming_rec = 0 as *mut stransfer;
        } else {
            qTtiming_rec = q;
        }
    }
    if (*q).fcmd != 0 {
        let mut znull_0: *const libc::c_char = 0 as *const libc::c_char;
        znull_0 = 0 as *const libc::c_char;
        while cfirst > 0 as libc::c_int as libc::c_ulong {
            let mut cnew: size_t = 0;
            let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
            znull_0 = memchr(zfirst as *const libc::c_void, '\0' as i32, cfirst)
                as *const libc::c_char;
            if !znull_0.is_null() {
                cnew = znull_0.offset_from(zfirst) as libc::c_long as size_t;
            } else {
                cnew = cfirst;
            }
            znew = zbufalc(
                ((*q).ccmd)
                    .wrapping_add(cnew)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if (*q).ccmd > 0 as libc::c_int as libc::c_ulong {
                memcpy(
                    znew as *mut libc::c_void,
                    (*q).zcmd as *const libc::c_void,
                    (*q).ccmd,
                );
            }
            memcpy(
                znew.offset((*q).ccmd as isize) as *mut libc::c_void,
                zfirst as *const libc::c_void,
                cnew,
            );
            *znew
                .offset(
                    ((*q).ccmd).wrapping_add(cnew) as isize,
                ) = '\0' as i32 as libc::c_char;
            ubuffree((*q).zcmd);
            (*q).zcmd = znew;
            (*q)
                .ccmd = ((*q).ccmd as libc::c_ulong).wrapping_add(cnew) as size_t
                as size_t;
            if !znull_0.is_null() {
                break;
            }
            zfirst = zsecond;
            cfirst = csecond;
            csecond = 0 as libc::c_int as size_t;
        }
        if !znull_0.is_null() {
            let mut zcmd: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ccmd: size_t = 0;
            zcmd = (*q).zcmd;
            ccmd = (*q).ccmd;
            (*q).fcmd = 0 as libc::c_int;
            (*q).zcmd = 0 as *mut libc::c_char;
            (*q).ccmd = 0 as libc::c_int as size_t;
            if (Some(((*q).precfn).unwrap()))
                .unwrap()(
                q,
                qdaemon,
                zcmd,
                ccmd.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                fret = 0 as libc::c_int;
            }
            ubuffree(zcmd);
        }
        if !pfexit.is_null()
            && ((*qdaemon).fhangup != 0 || (*qdaemon).fmaster != 0 || !qTsend.is_null())
        {
            *pfexit = 1 as libc::c_int;
        }
    } else if (*q).frecfile == 0 || cfirst == 0 as libc::c_int as libc::c_ulong {
        (*q).frecfile = 0 as libc::c_int;
        if (Some(((*q).precfn).unwrap())).unwrap()(q, qdaemon, zfirst, cfirst) == 0 {
            fret = 0 as libc::c_int;
        }
        if fret != 0 && csecond > 0 as libc::c_int as libc::c_ulong {
            return fgot_data(
                qdaemon,
                zsecond,
                csecond,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_int as size_t,
                ilocal,
                iremote,
                ipos + cfirst as libc::c_long,
                0 as libc::c_int,
                pfexit,
            );
        }
        if !pfexit.is_null()
            && ((*qdaemon).fhangup != 0 || (*qdaemon).fmaster != 0 || !qTsend.is_null())
        {
            *pfexit = 1 as libc::c_int;
        }
    } else {
        if ipos != -(1 as libc::c_int) as libc::c_long && ipos != (*q).ipos {
            if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fgot_data: Seeking to %ld\0" as *const u8 as *const libc::c_char,
                    ipos,
                );
            }
            if !(fseek((*q).e, ipos, 0 as libc::c_int) == 0 as libc::c_int) {
                ulog(
                    LOG_ERROR,
                    b"seek: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                fret = 0 as libc::c_int;
            }
            (*q).ipos = ipos;
        }
        if fret != 0 {
            while cfirst > 0 as libc::c_int as libc::c_ulong {
                cwrote = fwrite(
                    zfirst as *mut libc::c_char as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    cfirst,
                    (*q).e,
                ) as libc::c_int;
                if cwrote >= 0 as libc::c_int && cwrote as size_t == cfirst {
                    let mut cfree_space: libc::c_long = 0;
                    cfree_space = (*(*qdaemon).qsys).uuconf_cfree_space;
                    if cfree_space > 0 as libc::c_int as libc::c_long
                        && ((*q).cbytes / 10240 as libc::c_int as libc::c_long) as size_t
                            != ((*q).cbytes as libc::c_ulong)
                                .wrapping_add(cfirst)
                                .wrapping_div(10240 as libc::c_int as libc::c_ulong)
                        && frec_check_free(q, cfree_space) == 0
                    {
                        fret = 0 as libc::c_int;
                        break;
                    } else {
                        (*q)
                            .cbytes = ((*q).cbytes as libc::c_ulong).wrapping_add(cfirst)
                            as libc::c_long as libc::c_long;
                        (*q)
                            .ipos = ((*q).ipos as libc::c_ulong).wrapping_add(cfirst)
                            as libc::c_long as libc::c_long;
                        zfirst = zsecond;
                        cfirst = csecond;
                        csecond = 0 as libc::c_int as size_t;
                    }
                } else {
                    if ferror((*q).e) != 0 {
                        ulog(
                            LOG_ERROR,
                            b"write: %s\0" as *const u8 as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                    } else {
                        ulog(
                            LOG_ERROR,
                            b"Wrote %d to file when trying to write %lu\0" as *const u8
                                as *const libc::c_char,
                            cwrote,
                            cfirst,
                        );
                    }
                    fret = 0 as libc::c_int;
                    break;
                }
            }
        }
        if !pfexit.is_null() && (*qdaemon).fhangup != 0 {
            *pfexit = 1 as libc::c_int;
        }
    }
    return fret;
}
unsafe extern "C" fn ftadd_cmd(
    mut qdaemon: *mut sdaemon,
    mut z: *const libc::c_char,
    mut clen: size_t,
    mut iremote: libc::c_int,
    mut flast: boolean,
) -> boolean {
    static mut zbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut cbuf: size_t = 0;
    let mut cneed: size_t = 0;
    let mut s: scmd = scmd {
        bcmd: 0,
        bgrade: 0,
        pseq: 0 as *mut libc::c_void,
        zfrom: 0 as *const libc::c_char,
        zto: 0 as *const libc::c_char,
        zuser: 0 as *const libc::c_char,
        zoptions: 0 as *const libc::c_char,
        ztemp: 0 as *const libc::c_char,
        imode: 0,
        znotify: 0 as *const libc::c_char,
        cbytes: 0,
        zcmd: 0 as *const libc::c_char,
        ipos: 0,
    };
    cneed = cTcmdlen.wrapping_add(clen).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if cneed > cbuf {
        zbuf = xrealloc(zbuf as pointer, cneed) as *mut libc::c_char;
        cbuf = cneed;
    }
    memcpy(
        zbuf.offset(cTcmdlen as isize) as *mut libc::c_void,
        z as *const libc::c_void,
        clen,
    );
    *zbuf.offset(cTcmdlen.wrapping_add(clen) as isize) = '\0' as i32 as libc::c_char;
    if flast == 0 {
        cTcmdlen = (cTcmdlen as libc::c_ulong).wrapping_add(clen) as size_t as size_t;
        return 1 as libc::c_int;
    }
    cTcmdlen = 0 as libc::c_int as size_t;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"ftadd_cmd: Got command \"%s\"\0" as *const u8 as *const libc::c_char,
            zbuf,
        );
    }
    if fparse_cmd(zbuf, &mut s) == 0 || s.bcmd as libc::c_int == 'P' as i32 {
        ulog(
            LOG_ERROR,
            b"Received garbled command \"%s\"\0" as *const u8 as *const libc::c_char,
            zbuf,
        );
        return 1 as libc::c_int;
    }
    if (*qdaemon).ifeatures & 0o1 as libc::c_int == 0 as libc::c_int {
        s.cbytes = -(1 as libc::c_int) as libc::c_long;
    }
    if s.bcmd as libc::c_int != 'H' as i32 && s.bcmd as libc::c_int != 'Y' as i32
        && s.bcmd as libc::c_int != 'N' as i32
    {
        ulog_user(s.zuser);
    } else {
        ulog_user(0 as *mut libc::c_void as *const libc::c_char);
    }
    match s.bcmd as libc::c_int {
        83 | 69 => return fremote_send_file_init(qdaemon, &mut s, iremote),
        82 => return fremote_rec_file_init(qdaemon, &mut s, iremote),
        88 => return fremote_xcmd_init(qdaemon, &mut s, iremote),
        72 => {
            ulog_close();
            ustats_close();
            let mut q: *mut stransfer = 0 as *mut stransfer;
            q = qtransalc(0 as *mut libc::c_void as *mut scmd);
            (*q)
                .psendfn = Some(
                fremote_hangup_reply
                    as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
            );
            (*q).iremote = iremote;
            (*q).s.bcmd = 'H' as i32 as libc::c_char;
            return fqueue_remote(qdaemon, q);
        }
        78 => return 1 as libc::c_int,
        89 => {
            if (*qdaemon).fhangup != 0 {
                return 1 as libc::c_int;
            }
            if (*qdaemon).fmaster != 0 {
                ulog(
                    LOG_ERROR,
                    b"Got hangup reply as master\0" as *const u8 as *const libc::c_char,
                );
            }
            fLog_sighup = 0 as libc::c_int;
            (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
                .unwrap()(
                qdaemon,
                b"HY\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                iremote,
            );
            (*qdaemon).fhangup = 1 as libc::c_int;
            return 1 as libc::c_int;
        }
        _ => {
            ulog(
                LOG_FATAL,
                b"ftadd_cmd: Can't happen\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn fremote_hangup_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut fret: boolean = 0;
    utransfree(qtrans);
    if qTremote.is_null() && qTlocal.is_null() && qTsend.is_null() && qTreceive.is_null()
    {
        if fqueue(qdaemon, 0 as *mut libc::c_void as *mut boolean) == 0 {
            return 0 as libc::c_int;
        }
        if qTlocal.is_null() {
            if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fremote_hangup_reply: No work\0" as *const u8
                        as *const libc::c_char,
                );
            }
            fret = ((Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
                .unwrap()(
                qdaemon,
                b"HY\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
            ) != 0
                && (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
                    .unwrap()(
                    qdaemon,
                    b"HY\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ) != 0) as libc::c_int;
            (*qdaemon).fhangup = 1 as libc::c_int;
            return fret;
        }
    }
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fremote_hangup_reply: Found work\0" as *const u8 as *const libc::c_char,
        );
    }
    fret = (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(
        qdaemon,
        b"HN\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    (*qdaemon).fmaster = 1 as libc::c_int;
    return fret;
}
static mut qTfree_receive_ack: *mut sreceive_ack = 0 as *const sreceive_ack
    as *mut sreceive_ack;
pub unsafe extern "C" fn usent_receive_ack(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
) {
    let mut q: *mut sreceive_ack = 0 as *mut sreceive_ack;
    if qTfree_receive_ack.is_null() {
        q = xmalloc(::std::mem::size_of::<sreceive_ack>() as libc::c_ulong)
            as *mut sreceive_ack;
    } else {
        q = qTfree_receive_ack;
        qTfree_receive_ack = (*q).qnext;
    }
    (*q).qnext = qTreceive_ack;
    (*q).zto = zbufcpy((*qtrans).s.zto);
    (*q).ztemp = zbufcpy((*qtrans).s.ztemp);
    (*q).fmarked = 0 as libc::c_int;
    qTreceive_ack = q;
}
unsafe extern "C" fn utfree_receive_ack(mut q: *mut sreceive_ack) {
    ubuffree((*q).zto);
    ubuffree((*q).ztemp);
    (*q).qnext = qTfree_receive_ack;
    qTfree_receive_ack = q;
}
pub unsafe extern "C" fn uwindow_acked(
    mut qdaemon: *mut sdaemon,
    mut fallacked: boolean,
) {
    let mut pq: *mut *mut sreceive_ack = 0 as *mut *mut sreceive_ack;
    pq = &mut qTreceive_ack;
    while !(*pq).is_null() {
        if fallacked != 0 || (**pq).fmarked != 0 {
            let mut q: *mut sreceive_ack = 0 as *mut sreceive_ack;
            q = *pq;
            fsysdep_forget_reception((*qdaemon).qsys, (*q).zto, (*q).ztemp);
            *pq = (*q).qnext;
            utfree_receive_ack(q);
        } else {
            (**pq).fmarked = 1 as libc::c_int;
            pq = &mut (**pq).qnext;
        }
    }
}
unsafe extern "C" fn utfree_acked() {
    let mut q: *mut sreceive_ack = 0 as *mut sreceive_ack;
    q = qTreceive_ack;
    while !q.is_null() {
        let mut qnext: *mut sreceive_ack = 0 as *mut sreceive_ack;
        qnext = (*q).qnext;
        utfree_receive_ack(q);
        q = qnext;
    }
    qTreceive_ack = 0 as *mut sreceive_ack;
}
pub unsafe extern "C" fn ufailed(mut qdaemon: *mut sdaemon) {
    let mut q: *mut stransfer = 0 as *mut stransfer;
    if !qTsend.is_null() {
        q = qTsend;
        loop {
            if ((*q).fsendfile != 0 || (*q).frecfile != 0)
                && (*q).cbytes > 0 as libc::c_int as libc::c_long
            {
                ustats(
                    0 as libc::c_int,
                    (*q).s.zuser,
                    (*(*qdaemon).qsys).uuconf_zname,
                    (*q).fsendfile,
                    (*q).cbytes,
                    (*q).isecs,
                    (*q).imicros,
                    (*qdaemon).fcaller,
                );
                if (*q).fsendfile != 0 {
                    (*qdaemon).csent += (*q).cbytes;
                } else {
                    (*qdaemon).creceived += (*q).cbytes;
                }
            }
            if (*q).frecfile != 0 {
                frec_discard_temp(qdaemon, q);
            }
            q = (*q).qnext;
            if !(q != qTsend) {
                break;
            }
        }
    }
    if !qTreceive.is_null() {
        q = qTreceive;
        loop {
            if ((*q).fsendfile != 0 || (*q).frecfile != 0)
                && (*q).cbytes > 0 as libc::c_int as libc::c_long
            {
                ustats(
                    0 as libc::c_int,
                    (*q).s.zuser,
                    (*(*qdaemon).qsys).uuconf_zname,
                    (*q).fsendfile,
                    (*q).cbytes,
                    (*q).isecs,
                    (*q).imicros,
                    (*qdaemon).fcaller,
                );
                if (*q).fsendfile != 0 {
                    (*qdaemon).csent += (*q).cbytes;
                } else {
                    (*qdaemon).creceived += (*q).cbytes;
                }
            }
            if (*q).frecfile != 0 {
                frec_discard_temp(qdaemon, q);
            }
            q = (*q).qnext;
            if !(q != qTreceive) {
                break;
            }
        }
    }
}
unsafe extern "C" fn flocal_poll_file(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut fret: boolean = 0;
    fret = fsysdep_did_work((*qtrans).s.pseq);
    utransfree(qtrans);
    return fret;
}
