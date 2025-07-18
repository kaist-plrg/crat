use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sconnection;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut iDebug: libc::c_int;
    fn fspool_file(zfile: *const libc::c_char) -> boolean;
    fn fmail_transfer(
        fok: boolean,
        zuser: *const libc::c_char,
        zmail: *const libc::c_char,
        zwhy: *const libc::c_char,
        zfrom: *const libc::c_char,
        zfromsys: *const libc::c_char,
        zto: *const libc::c_char,
        ztosys: *const libc::c_char,
        zsaved: *const libc::c_char,
    ) -> boolean;
    fn fin_directory_list(
        zfile: *const libc::c_char,
        pzdirs: *mut *mut libc::c_char,
        zpubdir: *const libc::c_char,
        fcheck: boolean,
        freadable: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn fcmd_needs_quotes(qcmd: *const scmd) -> boolean;
    fn uquote_cmd(qorig: *const scmd, qnew: *mut scmd);
    fn ufree_quoted_cmd(qcmd: *mut scmd);
    fn zquote_cmd_string(
        zorig: *const libc::c_char,
        fbackslashonly: boolean,
    ) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
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
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    fn uuconf_system_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zsystem: *const libc::c_char,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_system_local(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn zsysdep_localname() -> *const libc::c_char;
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_mail(
        zto: *const libc::c_char,
        zsubject: *const libc::c_char,
        cstrs: libc::c_int,
        paz: *mut *const libc::c_char,
    ) -> boolean;
    fn fsysdep_did_work(pseq: pointer) -> boolean;
    fn zsysdep_add_base(
        zfile: *const libc::c_char,
        zname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn zsysdep_spool_file_name(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        pseq: pointer,
    ) -> *mut libc::c_char;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn esysdep_fopen(
        zfile: *const libc::c_char,
        fpublic: boolean,
        fappend: boolean,
        fmkdirs: boolean,
    ) -> *mut FILE;
    fn zsysdep_receive_temp(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        ztemp: *const libc::c_char,
        frestart: boolean,
    ) -> *mut libc::c_char;
    fn esysdep_open_receive(
        qsys: *const uuconf_system,
        zto: *const libc::c_char,
        ztemp: *const libc::c_char,
        zreceive: *const libc::c_char,
        pcrestart: *mut libc::c_long,
    ) -> openfile_t;
    fn fsysdep_move_file(
        zorig: *const libc::c_char,
        zto: *const libc::c_char,
        fmkdirs: boolean,
        fpublic: boolean,
        fcheck: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn fsysdep_change_mode(zfile: *const libc::c_char, imode: libc::c_uint) -> boolean;
    fn fsysdep_sync(e: openfile_t, zmsg: *const libc::c_char) -> boolean;
    fn fsysdep_remember_reception(
        qsys: *const uuconf_system,
        zto: *const libc::c_char,
        ztemp: *const libc::c_char,
    ) -> boolean;
    fn fsysdep_already_received(
        qsys: *const uuconf_system,
        zto: *const libc::c_char,
        ztemp: *const libc::c_char,
    ) -> boolean;
    fn csysdep_bytes_free(zfile: *const libc::c_char) -> libc::c_long;
    fn qtransalc(qcmd: *mut scmd) -> *mut stransfer;
    fn utransfree(qtrans: *mut stransfer);
    fn fqueue_local(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_remote(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_send(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_receive(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn usent_receive_ack(qdaemon: *mut sdaemon, qtrans: *mut stransfer);
    fn fspawn_uuxqt(
        ffork: boolean,
        zsys: *const libc::c_char,
        zconfig: *const libc::c_char,
    ) -> boolean;
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
pub type tfailure = libc::c_uint;
pub const FAILURE_RECEIVED: tfailure = 4;
pub const FAILURE_SIZE: tfailure = 3;
pub const FAILURE_OPEN: tfailure = 2;
pub const FAILURE_PERM: tfailure = 1;
pub const FAILURE_NONE: tfailure = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srecinfo {
    pub zmail: *mut libc::c_char,
    pub zfile: *mut libc::c_char,
    pub ztemp: *mut libc::c_char,
    pub fspool: boolean,
    pub flocal: boolean,
    pub freceived: boolean,
    pub freplied: boolean,
    pub fmoved: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srecfailinfo {
    pub twhy: tfailure,
    pub fsent: boolean,
    pub freceived: boolean,
}
pub static mut rec_rcsid: [libc::c_char; 48] = unsafe {
    *::std::mem::transmute::<
        &[u8; 48],
        &[libc::c_char; 48],
    >(b"$Id: rec.c,v 1.48 2002/03/05 19:10:41 ian Rel $\0")
};
unsafe extern "C" fn urrec_free(mut qtrans: *mut stransfer) {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    if !qinfo.is_null() {
        ubuffree((*qinfo).zmail);
        ubuffree((*qinfo).zfile);
        ubuffree((*qinfo).ztemp);
        xfree((*qtrans).pinfo);
    }
    utransfree(qtrans);
}
pub unsafe extern "C" fn flocal_rec_file_init(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
) -> boolean {
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut fspool: boolean = 0;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qinfo: *mut srecinfo = 0 as *mut srecinfo;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    qsys = (*qdaemon).qsys;
    if if (*qdaemon).fcaller != 0 {
        ((*qsys).uuconf_fcall_transfer == 0) as libc::c_int
    } else {
        ((*qsys).uuconf_fcalled_transfer == 0) as libc::c_int
    } != 0
    {
        if (*qsys).uuconf_fcall_transfer == 0 && (*qsys).uuconf_fcalled_transfer == 0 {
            return flocal_rec_fail(
                0 as *mut libc::c_void as *mut stransfer,
                qcmd,
                qsys,
                b"not permitted to request files\0" as *const u8 as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    fspool = fspool_file((*qcmd).zto);
    if fspool != 0 {
        let mut puuconf: pointer = 0 as *mut libc::c_void;
        let mut iuuconf: libc::c_int = 0;
        let mut zlocalname: *const libc::c_char = 0 as *const libc::c_char;
        let mut slocalsys: uuconf_system = uuconf_system {
            uuconf_zname: 0 as *mut libc::c_char,
            uuconf_pzalias: 0 as *mut *mut libc::c_char,
            uuconf_qalternate: 0 as *mut uuconf_system,
            uuconf_zalternate: 0 as *mut libc::c_char,
            uuconf_fcall: 0,
            uuconf_fcalled: 0,
            uuconf_qtimegrade: 0 as *mut uuconf_timespan,
            uuconf_qcalltimegrade: 0 as *mut uuconf_timespan,
            uuconf_qcalledtimegrade: 0 as *mut uuconf_timespan,
            uuconf_cmax_retries: 0,
            uuconf_csuccess_wait: 0,
            uuconf_qcall_local_size: 0 as *mut uuconf_timespan,
            uuconf_qcall_remote_size: 0 as *mut uuconf_timespan,
            uuconf_qcalled_local_size: 0 as *mut uuconf_timespan,
            uuconf_qcalled_remote_size: 0 as *mut uuconf_timespan,
            uuconf_ibaud: 0,
            uuconf_ihighbaud: 0,
            uuconf_zport: 0 as *mut libc::c_char,
            uuconf_qport: 0 as *mut uuconf_port,
            uuconf_zphone: 0 as *mut libc::c_char,
            uuconf_schat: uuconf_chat {
                uuconf_pzchat: 0 as *mut *mut libc::c_char,
                uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                uuconf_ctimeout: 0,
                uuconf_pzfail: 0 as *mut *mut libc::c_char,
                uuconf_fstrip: 0,
            },
            uuconf_zcall_login: 0 as *mut libc::c_char,
            uuconf_zcall_password: 0 as *mut libc::c_char,
            uuconf_zcalled_login: 0 as *mut libc::c_char,
            uuconf_fcallback: 0,
            uuconf_fsequence: 0,
            uuconf_zprotocols: 0 as *mut libc::c_char,
            uuconf_qproto_params: 0 as *mut uuconf_proto_param,
            uuconf_scalled_chat: uuconf_chat {
                uuconf_pzchat: 0 as *mut *mut libc::c_char,
                uuconf_pzprogram: 0 as *mut *mut libc::c_char,
                uuconf_ctimeout: 0,
                uuconf_pzfail: 0 as *mut *mut libc::c_char,
                uuconf_fstrip: 0,
            },
            uuconf_zdebug: 0 as *mut libc::c_char,
            uuconf_zmax_remote_debug: 0 as *mut libc::c_char,
            uuconf_fsend_request: 0,
            uuconf_frec_request: 0,
            uuconf_fcall_transfer: 0,
            uuconf_fcalled_transfer: 0,
            uuconf_pzlocal_send: 0 as *mut *mut libc::c_char,
            uuconf_pzremote_send: 0 as *mut *mut libc::c_char,
            uuconf_pzlocal_receive: 0 as *mut *mut libc::c_char,
            uuconf_pzremote_receive: 0 as *mut *mut libc::c_char,
            uuconf_pzpath: 0 as *mut *mut libc::c_char,
            uuconf_pzcmds: 0 as *mut *mut libc::c_char,
            uuconf_cfree_space: 0,
            uuconf_pzforward_from: 0 as *mut *mut libc::c_char,
            uuconf_pzforward_to: 0 as *mut *mut libc::c_char,
            uuconf_zpubdir: 0 as *const libc::c_char,
            uuconf_zlocalname: 0 as *mut libc::c_char,
            uuconf_cmax_file_time: 0,
            uuconf_palloc: 0 as *mut libc::c_void,
        };
        if *((*qcmd).zto).offset(0 as libc::c_int as isize) as libc::c_int != 'D' as i32
            || (strchr((*qcmd).zoptions, '9' as i32)).is_null()
        {
            return flocal_rec_fail(
                0 as *mut libc::c_void as *mut stransfer,
                qcmd,
                qsys,
                b"not permitted to receive\0" as *const u8 as *const libc::c_char,
            );
        }
        puuconf = (*qdaemon).puuconf;
        iuuconf = uuconf_localname(puuconf, &mut zlocalname);
        if iuuconf == 1 as libc::c_int {
            zlocalname = zsysdep_localname();
            if zlocalname.is_null() {
                return 0 as libc::c_int;
            }
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            return 0 as libc::c_int;
        }
        iuuconf = uuconf_system_info(puuconf, zlocalname, &mut slocalsys);
        if iuuconf == 1 as libc::c_int {
            iuuconf = uuconf_system_local(puuconf, &mut slocalsys);
            if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                return 0 as libc::c_int;
            }
            slocalsys.uuconf_zname = zlocalname as *mut libc::c_char;
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            return 0 as libc::c_int;
        }
        zfile = zsysdep_spool_file_name(&mut slocalsys, (*qcmd).zto, (*qcmd).pseq);
        uuconf_free_block(slocalsys.uuconf_palloc);
        if zfile.is_null() {
            return 0 as libc::c_int;
        }
    } else {
        zfile = zsysdep_add_base((*qcmd).zto, (*qcmd).zfrom);
        if zfile.is_null() {
            return 0 as libc::c_int;
        }
        if fin_directory_list(
            zfile,
            (*qsys).uuconf_pzlocal_receive,
            (*qsys).uuconf_zpubdir,
            1 as libc::c_int,
            0 as libc::c_int,
            (*qcmd).zuser,
        ) == 0
        {
            ubuffree(zfile);
            return flocal_rec_fail(
                0 as *mut libc::c_void as *mut stransfer,
                qcmd,
                qsys,
                b"not permitted to receive\0" as *const u8 as *const libc::c_char,
            );
        }
        if (strchr((*qcmd).zoptions, 'f' as i32)).is_null() {
            if fsysdep_make_dirs(zfile, 1 as libc::c_int) == 0 {
                ubuffree(zfile);
                return flocal_rec_fail(
                    0 as *mut libc::c_void as *mut stransfer,
                    qcmd,
                    qsys,
                    b"cannot create directories\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    qinfo = xmalloc(::std::mem::size_of::<srecinfo>() as libc::c_ulong) as *mut srecinfo;
    if (strchr((*qcmd).zoptions, 'm' as i32)).is_null() {
        (*qinfo).zmail = 0 as *mut libc::c_char;
    } else {
        (*qinfo).zmail = zbufcpy((*qcmd).zuser);
    }
    (*qinfo).zfile = zfile;
    (*qinfo).ztemp = 0 as *mut libc::c_char;
    (*qinfo).fspool = fspool;
    (*qinfo).flocal = 1 as libc::c_int;
    (*qinfo).freceived = 0 as libc::c_int;
    (*qinfo).freplied = 1 as libc::c_int;
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        flocal_rec_send_request
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans).pinfo = qinfo as pointer;
    return fqueue_local(qdaemon, qtrans);
}
unsafe extern "C" fn flocal_rec_fail(
    mut qtrans: *mut stransfer,
    mut qcmd: *mut scmd,
    mut qsys: *const uuconf_system,
    mut zwhy: *const libc::c_char,
) -> boolean {
    if !zwhy.is_null() {
        ulog(
            LOG_ERROR,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
            zwhy,
        );
        fmail_transfer(
            0 as libc::c_int,
            (*qcmd).zuser,
            0 as *mut libc::c_void as *const libc::c_char,
            zwhy,
            (*qcmd).zfrom,
            (*qsys).uuconf_zname,
            (*qcmd).zto,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_char,
        );
        fsysdep_did_work((*qcmd).pseq);
    }
    if !qtrans.is_null() {
        urrec_free(qtrans);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn flocal_rec_send_request(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    let mut cbytes: libc::c_long = 0;
    let mut cbytes2: libc::c_long = 0;
    let mut fquote: boolean = 0;
    let mut qcmd: *const scmd = 0 as *const scmd;
    let mut squoted: scmd = scmd {
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
    let mut clen: size_t = 0;
    let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    (*qinfo)
        .ztemp = zsysdep_receive_temp(
        (*qdaemon).qsys,
        (*qinfo).zfile,
        0 as *mut libc::c_void as *const libc::c_char,
        ((*(*qdaemon).qproto).frestart != 0
            && (*qdaemon).ifeatures & 0o2 as libc::c_int != 0 as libc::c_int)
            as libc::c_int,
    );
    if ((*qinfo).ztemp).is_null() {
        urrec_free(qtrans);
        return 0 as libc::c_int;
    }
    (*qtrans).fcmd = 1 as libc::c_int;
    (*qtrans)
        .precfn = Some(
        flocal_rec_await_reply
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    if fqueue_receive(qdaemon, qtrans) == 0 {
        return 0 as libc::c_int;
    }
    cbytes = csysdep_bytes_free((*qinfo).ztemp);
    cbytes2 = csysdep_bytes_free((*qinfo).zfile);
    if cbytes < cbytes2 {
        cbytes = cbytes2;
    }
    if cbytes != -(1 as libc::c_int) as libc::c_long {
        cbytes -= (*(*qdaemon).qsys).uuconf_cfree_space;
        if cbytes < 0 as libc::c_int as libc::c_long {
            cbytes = 0 as libc::c_int as libc::c_long;
        }
    }
    if (*qdaemon).clocal_size != -(1 as libc::c_int) as libc::c_long
        && (cbytes == -(1 as libc::c_int) as libc::c_long
            || (*qdaemon).clocal_size < cbytes)
    {
        cbytes = (*qdaemon).clocal_size;
    }
    fquote = fcmd_needs_quotes(&mut (*qtrans).s);
    if fquote == 0 {
        qcmd = &mut (*qtrans).s;
    } else {
        if (*qdaemon).ifeatures & 0o40 as libc::c_int == 0 as libc::c_int {
            return flocal_rec_fail(
                qtrans,
                &mut (*qtrans).s,
                (*qdaemon).qsys,
                b"remote system does not support required quoting\0" as *const u8
                    as *const libc::c_char,
            );
        }
        uquote_cmd(&mut (*qtrans).s, &mut squoted);
        qcmd = &mut squoted;
    }
    clen = (strlen((*qcmd).zfrom))
        .wrapping_add(strlen((*qcmd).zto))
        .wrapping_add(strlen((*qcmd).zuser))
        .wrapping_add(strlen((*qcmd).zoptions))
        .wrapping_add(30 as libc::c_int as libc::c_ulong);
    zsend = zbufalc(clen);
    if (*qdaemon).ifeatures & 0o1 as libc::c_int == 0 as libc::c_int {
        sprintf(
            zsend,
            b"R %s %s %s -%s\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
            (*qcmd).zto,
            (*qcmd).zuser,
            (*qcmd).zoptions,
        );
    } else if (*qdaemon).ifeatures & 0o10 as libc::c_int == 0 as libc::c_int {
        sprintf(
            zsend,
            b"R %s %s %s -%s 0x%lx\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
            (*qcmd).zto,
            (*qcmd).zuser,
            (*qcmd).zoptions,
            cbytes as libc::c_ulong,
        );
    } else {
        sprintf(
            zsend,
            b"R %s %s %s -%s %ld\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
            (*qcmd).zto,
            (*qcmd).zuser,
            (*qcmd).zoptions,
            cbytes,
        );
    }
    fret = (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, zsend, (*qtrans).ilocal, (*qtrans).iremote);
    ubuffree(zsend);
    if fquote != 0 {
        ufree_quoted_cmd(&mut squoted);
    }
    return fret;
}
unsafe extern "C" fn flocal_rec_await_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    let mut zlog: *const libc::c_char = 0 as *const libc::c_char;
    let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
    if *zdata.offset(0 as libc::c_int as isize) as libc::c_int != 'R' as i32
        || *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'Y' as i32
            && *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'N' as i32
    {
        ulog(
            LOG_ERROR,
            b"%s: bad response to receive request: \"%s\"\0" as *const u8
                as *const libc::c_char,
            (*qtrans).s.zfrom,
            zdata,
        );
        urrec_free(qtrans);
        return 0 as libc::c_int;
    }
    if *zdata.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
        let mut fnever: boolean = 0;
        let mut zerr: *const libc::c_char = 0 as *const libc::c_char;
        fnever = 1 as libc::c_int;
        if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '2' as i32 {
            zerr = b"no such file\0" as *const u8 as *const libc::c_char;
        } else if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '6' as i32 {
            zerr = b"too large to receive now\0" as *const u8 as *const libc::c_char;
            fnever = 0 as libc::c_int;
        } else if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '9' as i32 {
            zerr = b"too many channels for remote\0" as *const u8 as *const libc::c_char;
            fnever = 0 as libc::c_int;
            if (*qdaemon).cchans > 2 as libc::c_int {
                (*qdaemon).cchans -= 1;
                (*qdaemon).cchans;
            }
        } else {
            zerr = b"unknown reason\0" as *const u8 as *const libc::c_char;
        }
        if fnever != 0 {
            return flocal_rec_fail(qtrans, &mut (*qtrans).s, (*qdaemon).qsys, zerr);
        }
        ulog(
            LOG_ERROR,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            (*qtrans).s.zfrom,
            zerr,
        );
        urrec_free(qtrans);
        return 1 as libc::c_int;
    }
    (*qtrans)
        .s
        .imode = strtol(
        zdata.offset(2 as libc::c_int as isize) as *mut libc::c_char,
        &mut zend,
        8 as libc::c_int,
    ) as libc::c_uint;
    if (*qtrans).s.imode == 0 as libc::c_int as libc::c_uint {
        (*qtrans).s.imode = 0o666 as libc::c_int as libc::c_uint;
    }
    if *zend as libc::c_int == 'M' as i32 && (*qdaemon).fmaster != 0 {
        if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"flocal_rec_await_reply: Remote has requested transfer of control\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        (*qdaemon).fhangup_requested = 1 as libc::c_int;
    }
    (*qtrans)
        .e = esysdep_open_receive(
        (*qdaemon).qsys,
        (*qinfo).zfile,
        0 as *mut libc::c_void as *const libc::c_char,
        (*qinfo).ztemp,
        0 as *mut libc::c_void as *mut libc::c_long,
    );
    if ((*qtrans).e).is_null() {
        return flocal_rec_fail(
            qtrans,
            &mut (*qtrans).s,
            (*qdaemon).qsys,
            b"cannot open file\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*qinfo).fspool != 0 {
        zlog = (*qtrans).s.zto;
    } else {
        zlog = (*qinfo).zfile;
    }
    (*qtrans)
        .zlog = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_add(strlen(zlog)),
    );
    sprintf((*qtrans).zlog, b"Receiving %s\0" as *const u8 as *const libc::c_char, zlog);
    if ((*(*qdaemon).qproto).pffile).is_some() {
        let mut fhandled: boolean = 0;
        if (Some(((*(*qdaemon).qproto).pffile).unwrap()))
            .unwrap()(
            qdaemon,
            qtrans,
            1 as libc::c_int,
            0 as libc::c_int,
            -(1 as libc::c_int) as libc::c_long,
            &mut fhandled,
        ) == 0
        {
            return flocal_rec_fail(
                qtrans,
                &mut (*qtrans).s,
                (*qdaemon).qsys,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        if fhandled != 0 {
            return 1 as libc::c_int;
        }
    }
    (*qtrans).frecfile = 1 as libc::c_int;
    (*qtrans)
        .psendfn = Some(
        frec_file_send_confirm
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans)
        .precfn = Some(
        frec_file_end
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    return fqueue_receive(qdaemon, qtrans);
}
pub unsafe extern "C" fn frec_check_free(
    mut qtrans: *mut stransfer,
    mut cfree_space: libc::c_long,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    let mut cfree1: libc::c_long = 0;
    let mut cfree2: libc::c_long = 0;
    cfree1 = csysdep_bytes_free((*qinfo).ztemp);
    cfree2 = csysdep_bytes_free((*qinfo).zfile);
    if cfree1 < cfree2 {
        cfree1 = cfree2;
    }
    if cfree1 != -(1 as libc::c_int) as libc::c_long && cfree1 < cfree_space {
        ulog(
            LOG_ERROR,
            b"%s: too big to receive now\0" as *const u8 as *const libc::c_char,
            (*qinfo).zfile,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fremote_send_file_init(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
    mut iremote: libc::c_int,
) -> boolean {
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut fspool: boolean = 0;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: openfile_t = 0 as *mut FILE;
    let mut ztemp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cbytes: libc::c_long = 0;
    let mut cbytes2: libc::c_long = 0;
    let mut crestart: libc::c_long = 0;
    let mut qinfo: *mut srecinfo = 0 as *mut srecinfo;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    let mut zlog: *const libc::c_char = 0 as *const libc::c_char;
    qsys = (*qdaemon).qsys;
    if (*qsys).uuconf_frec_request == 0 {
        ulog(
            LOG_ERROR,
            b"%s: not permitted to receive files from remote\0" as *const u8
                as *const libc::c_char,
            (*qcmd).zfrom,
        );
        return fremote_send_fail(qdaemon, qcmd, FAILURE_PERM, iremote);
    }
    fspool = fspool_file((*qcmd).zto);
    if fspool != 0
        && *((*qcmd).zto).offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32
        || (*qcmd).bcmd as libc::c_int == 'E' as i32
            && (fspool == 0
                || *((*qcmd).zto).offset(0 as libc::c_int as isize) as libc::c_int
                    != 'D' as i32)
    {
        ulog(
            LOG_ERROR,
            b"%s: not permitted to receive\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
        );
        return fremote_send_fail(qdaemon, qcmd, FAILURE_PERM, iremote);
    }
    if fsysdep_already_received(qsys, (*qcmd).zto, (*qcmd).ztemp) != 0 {
        return fremote_send_fail(qdaemon, qcmd, FAILURE_RECEIVED, iremote);
    }
    if fspool != 0 {
        zfile = zsysdep_spool_file_name(qsys, (*qcmd).zto, 0 as *mut libc::c_void);
        if zfile.is_null() {
            return 0 as libc::c_int;
        }
    } else {
        let mut fbadname: boolean = 0;
        zfile = zsysdep_local_file((*qcmd).zto, (*qsys).uuconf_zpubdir, &mut fbadname);
        if zfile.is_null() && fbadname != 0 {
            ulog(
                LOG_ERROR,
                b"%s: bad local file name\0" as *const u8 as *const libc::c_char,
                (*qcmd).zto,
            );
            return fremote_send_fail(qdaemon, qcmd, FAILURE_PERM, iremote);
        }
        if !zfile.is_null() {
            let mut zadd: *mut libc::c_char = 0 as *mut libc::c_char;
            zadd = zsysdep_add_base(zfile, (*qcmd).zfrom);
            ubuffree(zfile);
            zfile = zadd;
        }
        if zfile.is_null() {
            return 0 as libc::c_int;
        }
        if fin_directory_list(
            zfile,
            (*qsys).uuconf_pzremote_receive,
            (*qsys).uuconf_zpubdir,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void as *const libc::c_char,
        ) == 0
        {
            ulog(
                LOG_ERROR,
                b"%s: not permitted to receive\0" as *const u8 as *const libc::c_char,
                zfile,
            );
            ubuffree(zfile);
            return fremote_send_fail(qdaemon, qcmd, FAILURE_PERM, iremote);
        }
        if (strchr((*qcmd).zoptions, 'f' as i32)).is_null() {
            if fsysdep_make_dirs(zfile, 1 as libc::c_int) == 0 {
                ubuffree(zfile);
                return fremote_send_fail(qdaemon, qcmd, FAILURE_OPEN, iremote);
            }
        }
    }
    ztemp = zsysdep_receive_temp(
        qsys,
        zfile,
        (*qcmd).ztemp,
        ((*(*qdaemon).qproto).frestart != 0
            && (*qdaemon).ifeatures & 0o2 as libc::c_int != 0 as libc::c_int)
            as libc::c_int,
    );
    cbytes = csysdep_bytes_free(ztemp);
    cbytes2 = csysdep_bytes_free(zfile);
    if cbytes < cbytes2 {
        cbytes = cbytes2;
    }
    if cbytes != -(1 as libc::c_int) as libc::c_long {
        cbytes -= (*qsys).uuconf_cfree_space;
        if cbytes < 0 as libc::c_int as libc::c_long {
            cbytes = 0 as libc::c_int as libc::c_long;
        }
    }
    if (*qdaemon).cremote_size != -(1 as libc::c_int) as libc::c_long
        && (cbytes == -(1 as libc::c_int) as libc::c_long
            || (*qdaemon).cremote_size < cbytes)
    {
        cbytes = (*qdaemon).cremote_size;
    }
    if cbytes != -(1 as libc::c_int) as libc::c_long {
        let mut csize: libc::c_long = 0;
        csize = (*qcmd).cbytes;
        if csize == -(1 as libc::c_int) as libc::c_long {
            csize = 10240 as libc::c_int as libc::c_long;
        }
        if cbytes < csize {
            ulog(
                LOG_ERROR,
                b"%s: too big to receive\0" as *const u8 as *const libc::c_char,
                zfile,
            );
            ubuffree(ztemp);
            ubuffree(zfile);
            return fremote_send_fail(qdaemon, qcmd, FAILURE_SIZE, iremote);
        }
    }
    crestart = -(1 as libc::c_int) as libc::c_long;
    e = esysdep_open_receive(
        qsys,
        zfile,
        (*qcmd).ztemp,
        ztemp,
        if (*(*qdaemon).qproto).frestart != 0
            && (*qdaemon).ifeatures & 0o2 as libc::c_int != 0 as libc::c_int
        {
            &mut crestart
        } else {
            0 as *mut libc::c_void as *mut libc::c_long
        },
    );
    if e.is_null() {
        ubuffree(ztemp);
        ubuffree(zfile);
        return fremote_send_fail(qdaemon, qcmd, FAILURE_OPEN, iremote);
    }
    if crestart > 0 as libc::c_int as libc::c_long {
        if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fremote_send_file_init: Restarting receive from %ld\0" as *const u8
                    as *const libc::c_char,
                crestart,
            );
        }
        if !(fseek(e, crestart, 0 as libc::c_int) == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"seek: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            fclose(e);
            ubuffree(ztemp);
            ubuffree(zfile);
            return 0 as libc::c_int;
        }
    }
    qinfo = xmalloc(::std::mem::size_of::<srecinfo>() as libc::c_ulong) as *mut srecinfo;
    if (strchr((*qcmd).zoptions, 'n' as i32)).is_null() {
        (*qinfo).zmail = 0 as *mut libc::c_char;
    } else {
        (*qinfo).zmail = zbufcpy((*qcmd).znotify);
    }
    (*qinfo).zfile = zfile;
    (*qinfo).ztemp = ztemp;
    (*qinfo).fspool = fspool;
    (*qinfo).flocal = 0 as libc::c_int;
    (*qinfo).freceived = 0 as libc::c_int;
    (*qinfo).freplied = 0 as libc::c_int;
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        fremote_send_reply
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans)
        .precfn = Some(
        frec_file_end
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    (*qtrans).iremote = iremote;
    (*qtrans).pinfo = qinfo as pointer;
    (*qtrans).frecfile = 1 as libc::c_int;
    (*qtrans).e = e;
    if crestart > 0 as libc::c_int as libc::c_long {
        (*qtrans).ipos = crestart;
    }
    if (*qcmd).bcmd as libc::c_int == 'E' as i32 {
        zlog = (*qcmd).zcmd;
    } else if (*qinfo).fspool != 0 {
        zlog = (*qcmd).zto;
    } else {
        zlog = (*qinfo).zfile;
    }
    (*qtrans)
        .zlog = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_add(strlen(zlog))
            .wrapping_add(50 as libc::c_int as libc::c_ulong),
    );
    sprintf((*qtrans).zlog, b"Receiving %s\0" as *const u8 as *const libc::c_char, zlog);
    if crestart > 0 as libc::c_int as libc::c_long
        || (*qcmd).cbytes > 0 as libc::c_int as libc::c_long
    {
        strcat((*qtrans).zlog, b" (\0" as *const u8 as *const libc::c_char);
        if (*qcmd).cbytes > 0 as libc::c_int as libc::c_long {
            sprintf(
                ((*qtrans).zlog).offset(strlen((*qtrans).zlog) as isize),
                b"%ld bytes\0" as *const u8 as *const libc::c_char,
                (*qcmd).cbytes,
            );
            if crestart > 0 as libc::c_int as libc::c_long {
                strcat((*qtrans).zlog, b" \0" as *const u8 as *const libc::c_char);
            }
        }
        if crestart > 0 as libc::c_int as libc::c_long {
            sprintf(
                ((*qtrans).zlog).offset(strlen((*qtrans).zlog) as isize),
                b"resume at %ld\0" as *const u8 as *const libc::c_char,
                crestart,
            );
        }
        strcat((*qtrans).zlog, b")\0" as *const u8 as *const libc::c_char);
    }
    return fqueue_remote(qdaemon, qtrans);
}
unsafe extern "C" fn fremote_send_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    let mut fret: boolean = 0;
    let mut ab: [libc::c_char; 50] = [0; 50];
    (*qtrans)
        .psendfn = Some(
        frec_file_send_confirm
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    if (*qinfo).freceived != 0 {
        fret = fqueue_send(qdaemon, qtrans);
    } else {
        fret = fqueue_receive(qdaemon, qtrans);
    }
    if fret == 0 {
        return 0 as libc::c_int;
    }
    ab[0 as libc::c_int as usize] = (*qtrans).s.bcmd;
    ab[1 as libc::c_int as usize] = 'Y' as i32 as libc::c_char;
    if (*qtrans).ipos <= 0 as libc::c_int as libc::c_long {
        ab[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        sprintf(
            ab.as_mut_ptr().offset(2 as libc::c_int as isize),
            b" 0x%lx\0" as *const u8 as *const libc::c_char,
            (*qtrans).ipos as libc::c_ulong,
        );
    }
    (*qinfo).freplied = 1 as libc::c_int;
    if (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, ab.as_mut_ptr(), (*qtrans).ilocal, (*qtrans).iremote) == 0
    {
        fclose((*qtrans).e);
        (*qtrans).e = 0 as *mut libc::c_void as *mut FILE;
        remove((*qinfo).ztemp);
        return 0 as libc::c_int;
    }
    if ((*(*qdaemon).qproto).pffile).is_some() {
        let mut fhandled: boolean = 0;
        if (Some(((*(*qdaemon).qproto).pffile).unwrap()))
            .unwrap()(
            qdaemon,
            qtrans,
            1 as libc::c_int,
            0 as libc::c_int,
            -(1 as libc::c_int) as libc::c_long,
            &mut fhandled,
        ) == 0
        {
            remove((*qinfo).ztemp);
            urrec_free(qtrans);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fremote_send_fail(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
    mut twhy: tfailure,
    mut iremote: libc::c_int,
) -> boolean {
    let mut qinfo: *mut srecfailinfo = 0 as *mut srecfailinfo;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    qinfo = xmalloc(::std::mem::size_of::<srecfailinfo>() as libc::c_ulong)
        as *mut srecfailinfo;
    (*qinfo).twhy = twhy;
    (*qinfo).fsent = 0 as libc::c_int;
    (*qinfo).freceived = ((*qdaemon).cchans <= 1 as libc::c_int) as libc::c_int;
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        fremote_send_fail_send
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans)
        .precfn = Some(
        fremote_discard
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    (*qtrans).iremote = iremote;
    (*qtrans).pinfo = qinfo as pointer;
    return fqueue_remote(qdaemon, qtrans);
}
unsafe extern "C" fn fremote_send_fail_send(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut srecfailinfo = (*qtrans).pinfo as *mut srecfailinfo;
    let mut ab: [libc::c_char; 4] = [0; 4];
    let mut ilocal: libc::c_int = 0;
    let mut iremote: libc::c_int = 0;
    ab[0 as libc::c_int as usize] = (*qtrans).s.bcmd;
    ab[1 as libc::c_int as usize] = 'N' as i32 as libc::c_char;
    match (*qinfo).twhy as libc::c_uint {
        1 => {
            ab[2 as libc::c_int as usize] = '2' as i32 as libc::c_char;
        }
        2 => {
            ab[2 as libc::c_int as usize] = '4' as i32 as libc::c_char;
        }
        3 => {
            ab[2 as libc::c_int as usize] = '6' as i32 as libc::c_char;
        }
        4 => {
            usent_receive_ack(qdaemon, qtrans);
            ab[2 as libc::c_int as usize] = '8' as i32 as libc::c_char;
        }
        _ => {
            ab[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
    }
    ab[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    ilocal = (*qtrans).ilocal;
    iremote = (*qtrans).iremote;
    if (*qinfo).freceived == 0 {
        (*qinfo).fsent = 1 as libc::c_int;
        if fqueue_receive(qdaemon, qtrans) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        xfree((*qtrans).pinfo);
        utransfree(qtrans);
    }
    return (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, ab.as_mut_ptr(), ilocal, iremote);
}
unsafe extern "C" fn fremote_discard(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut srecfailinfo = (*qtrans).pinfo as *mut srecfailinfo;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fremote_discard: Discarding %lu bytes\0" as *const u8
                as *const libc::c_char,
            cdata,
        );
    }
    if cdata != 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    (*qinfo).freceived = 1 as libc::c_int;
    if (*qinfo).fsent != 0 {
        xfree((*qtrans).pinfo);
        utransfree(qtrans);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn frec_file_end(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zerr: *const libc::c_char = 0 as *const libc::c_char;
    let mut fnever: boolean = 0;
    if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"frec_file_end: %s to %s (freplied %s)\0" as *const u8
                as *const libc::c_char,
            (*qtrans).s.zfrom,
            (*qtrans).s.zto,
            if (*qinfo).freplied != 0 {
                b"TRUE\0" as *const u8 as *const libc::c_char
            } else {
                b"FALSE\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if ((*(*qdaemon).qproto).pffile).is_some() {
        let mut fhandled: boolean = 0;
        if (Some(((*(*qdaemon).qproto).pffile).unwrap()))
            .unwrap()(
            qdaemon,
            qtrans,
            0 as libc::c_int,
            0 as libc::c_int,
            -(1 as libc::c_int) as libc::c_long,
            &mut fhandled,
        ) == 0
        {
            remove((*qinfo).ztemp);
            urrec_free(qtrans);
            return 0 as libc::c_int;
        }
        if fhandled != 0 {
            return 1 as libc::c_int;
        }
    }
    (*qinfo).freceived = 1 as libc::c_int;
    fnever = 0 as libc::c_int;
    zalc = 0 as *mut libc::c_char;
    if fsysdep_sync((*qtrans).e, (*qtrans).s.zto) == 0 {
        zerr = strerror(*__errno_location());
        fclose((*qtrans).e);
        (*qtrans).e = 0 as *mut libc::c_void as *mut FILE;
        remove((*qinfo).ztemp);
    } else if !(fclose((*qtrans).e) == 0 as libc::c_int) {
        zerr = strerror(*__errno_location());
        ulog(
            LOG_ERROR,
            b"%s: close: %s\0" as *const u8 as *const libc::c_char,
            (*qtrans).s.zto,
            zerr,
        );
        remove((*qinfo).ztemp);
        (*qtrans).e = 0 as *mut libc::c_void as *mut FILE;
    } else {
        (*qtrans).e = 0 as *mut libc::c_void as *mut FILE;
        if fsysdep_move_file(
            (*qinfo).ztemp,
            (*qinfo).zfile,
            (*qinfo).fspool,
            0 as libc::c_int,
            ((*qinfo).fspool == 0) as libc::c_int,
            if (*qinfo).flocal != 0 {
                (*qtrans).s.zuser
            } else {
                0 as *mut libc::c_void as *const libc::c_char
            },
        ) == 0
        {
            let mut cspace: libc::c_long = 0;
            cspace = csysdep_bytes_free((*qinfo).ztemp);
            if cspace == -(1 as libc::c_int) as libc::c_long {
                cspace = 10240 as libc::c_int as libc::c_long;
            }
            cspace
                -= (*(*qdaemon).qsys).uuconf_cfree_space
                    + (*(*qdaemon).qsys).uuconf_cfree_space
                        / 2 as libc::c_int as libc::c_long;
            if cspace < 0 as libc::c_int as libc::c_long {
                remove((*qinfo).ztemp);
                zerr = b"could not move to final location\0" as *const u8
                    as *const libc::c_char;
            } else {
                let mut az: [*const libc::c_char; 20] = [0 as *const libc::c_char; 20];
                let mut i: libc::c_int = 0;
                zalc = zbufalc(
                    (::std::mem::size_of::<[libc::c_char; 44]>() as libc::c_ulong)
                        .wrapping_add(strlen((*qinfo).ztemp)),
                );
                sprintf(
                    zalc,
                    b"could not move to final location (left as %s)\0" as *const u8
                        as *const libc::c_char,
                    (*qinfo).ztemp,
                );
                zerr = zalc;
                i = 0 as libc::c_int;
                let fresh0 = i;
                i = i + 1;
                az[fresh0
                    as usize] = b"The file\n\t\0" as *const u8 as *const libc::c_char;
                let fresh1 = i;
                i = i + 1;
                az[fresh1 as usize] = (*qinfo).ztemp;
                let fresh2 = i;
                i = i + 1;
                az[fresh2
                    as usize] = b"\nwas saved because the move to the final location failed.\n\0"
                    as *const u8 as *const libc::c_char;
                let fresh3 = i;
                i = i + 1;
                az[fresh3
                    as usize] = b"See the UUCP logs for more details.\n\0" as *const u8
                    as *const libc::c_char;
                let fresh4 = i;
                i = i + 1;
                az[fresh4
                    as usize] = b"The file transfer was from\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh5 = i;
                i = i + 1;
                az[fresh5 as usize] = (*(*qdaemon).qsys).uuconf_zname;
                let fresh6 = i;
                i = i + 1;
                az[fresh6 as usize] = b"!\0" as *const u8 as *const libc::c_char;
                let fresh7 = i;
                i = i + 1;
                az[fresh7 as usize] = (*qtrans).s.zfrom;
                let fresh8 = i;
                i = i + 1;
                az[fresh8 as usize] = b"\nto\n\t\0" as *const u8 as *const libc::c_char;
                let fresh9 = i;
                i = i + 1;
                az[fresh9 as usize] = (*qtrans).s.zto;
                let fresh10 = i;
                i = i + 1;
                az[fresh10
                    as usize] = b"\nand was requested by\n\t\0" as *const u8
                    as *const libc::c_char;
                let fresh11 = i;
                i = i + 1;
                az[fresh11 as usize] = (*qtrans).s.zuser;
                let fresh12 = i;
                i = i + 1;
                az[fresh12 as usize] = b"\n\0" as *const u8 as *const libc::c_char;
                fsysdep_mail(
                    b"uucp\0" as *const u8 as *const libc::c_char,
                    b"UUCP temporary file saved\0" as *const u8 as *const libc::c_char,
                    i,
                    az.as_mut_ptr(),
                );
            }
            ulog(
                LOG_ERROR,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                (*qinfo).zfile,
                zerr,
            );
            fnever = 1 as libc::c_int;
        } else {
            if (*qinfo).fspool == 0 {
                let mut imode: libc::c_uint = 0;
                if (*qtrans).s.imode & 0o111 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    imode = 0o777 as libc::c_int as libc::c_uint;
                } else {
                    imode = 0o666 as libc::c_int as libc::c_uint;
                }
                fsysdep_change_mode((*qinfo).zfile, imode);
            }
            zerr = 0 as *const libc::c_char;
        }
    }
    ustats(
        (zerr == 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int,
        (*qtrans).s.zuser,
        (*(*qdaemon).qsys).uuconf_zname,
        0 as libc::c_int,
        (*qtrans).cbytes,
        (*qtrans).isecs,
        (*qtrans).imicros,
        (*qdaemon).fcaller,
    );
    (*qdaemon).creceived += (*qtrans).cbytes;
    if zerr.is_null() {
        if !((*qinfo).zmail).is_null() && *(*qinfo).zmail as libc::c_int != '\0' as i32 {
            fmail_transfer(
                1 as libc::c_int,
                (*qtrans).s.zuser,
                (*qinfo).zmail,
                0 as *mut libc::c_void as *const libc::c_char,
                (*qtrans).s.zfrom,
                (*(*qdaemon).qsys).uuconf_zname,
                (*qtrans).s.zto,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        if !((*qtrans).s.pseq).is_null() {
            fsysdep_did_work((*qtrans).s.pseq);
        }
        if (*qinfo).flocal == 0 {
            fsysdep_remember_reception(
                (*qdaemon).qsys,
                (*qtrans).s.zto,
                (*qtrans).s.ztemp,
            );
        }
    } else if (*qinfo).flocal != 0 && fnever != 0 {
        fmail_transfer(
            0 as libc::c_int,
            (*qtrans).s.zuser,
            (*qinfo).zmail,
            zerr,
            (*qtrans).s.zfrom,
            (*(*qdaemon).qsys).uuconf_zname,
            (*qtrans).s.zto,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_char,
        );
        fsysdep_did_work((*qtrans).s.pseq);
    }
    ubuffree(zalc);
    if (*qtrans).s.bcmd as libc::c_int == 'E' as i32 && zerr.is_null() {
        let mut zxqt: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zxqtfile: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ztemp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut e: *mut FILE = 0 as *mut FILE;
        let mut fbad: boolean = 0;
        zxqt = zbufcpy((*qtrans).s.zto);
        *zxqt.offset(0 as libc::c_int as isize) = 'X' as i32 as libc::c_char;
        zxqtfile = zsysdep_spool_file_name(
            (*qdaemon).qsys,
            zxqt,
            0 as *mut libc::c_void,
        );
        ubuffree(zxqt);
        if zxqtfile.is_null() {
            urrec_free(qtrans);
            return 0 as libc::c_int;
        }
        e = 0 as *mut FILE;
        ztemp = zsysdep_receive_temp(
            (*qdaemon).qsys,
            zxqtfile,
            b"D.0\0" as *const u8 as *const libc::c_char,
            ((*(*qdaemon).qproto).frestart != 0
                && (*qdaemon).ifeatures & 0o2 as libc::c_int != 0 as libc::c_int)
                as libc::c_int,
        );
        if !ztemp.is_null() {
            e = esysdep_fopen(
                ztemp,
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        }
        if e.is_null() {
            ubuffree(zxqtfile);
            ubuffree(ztemp);
            urrec_free(qtrans);
            return 0 as libc::c_int;
        }
        if fcmd_needs_quotes(&mut (*qtrans).s) == 0 {
            fprintf(
                e,
                b"U %s %s\n\0" as *const u8 as *const libc::c_char,
                (*qtrans).s.zuser,
                (*(*qdaemon).qsys).uuconf_zname,
            );
            fprintf(e, b"F %s\n\0" as *const u8 as *const libc::c_char, (*qtrans).s.zto);
            fprintf(e, b"I %s\n\0" as *const u8 as *const libc::c_char, (*qtrans).s.zto);
            if !(strchr((*qtrans).s.zoptions, 'R' as i32)).is_null() {
                fprintf(
                    e,
                    b"R %s\n\0" as *const u8 as *const libc::c_char,
                    (*qtrans).s.znotify,
                );
            }
            fprintf(
                e,
                b"C %s\n\0" as *const u8 as *const libc::c_char,
                (*qtrans).s.zcmd,
            );
        } else {
            let mut z1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut z2: *mut libc::c_char = 0 as *mut libc::c_char;
            fprintf(e, b"Q\n\0" as *const u8 as *const libc::c_char);
            z1 = zquote_cmd_string((*qtrans).s.zuser, 0 as libc::c_int);
            z2 = zquote_cmd_string((*(*qdaemon).qsys).uuconf_zname, 0 as libc::c_int);
            fprintf(e, b"U %s %s\n\0" as *const u8 as *const libc::c_char, z1, z2);
            ubuffree(z1);
            ubuffree(z2);
            z1 = zquote_cmd_string((*qtrans).s.zto, 0 as libc::c_int);
            fprintf(e, b"F %s\n\0" as *const u8 as *const libc::c_char, z1);
            fprintf(e, b"I %s\n\0" as *const u8 as *const libc::c_char, z1);
            ubuffree(z1);
            if !(strchr((*qtrans).s.zoptions, 'R' as i32)).is_null() {
                z1 = zquote_cmd_string((*qtrans).s.znotify, 0 as libc::c_int);
                fprintf(e, b"R %s\n\0" as *const u8 as *const libc::c_char, z1);
                ubuffree(z1);
            }
            z1 = zquote_cmd_string((*qtrans).s.zcmd, 1 as libc::c_int);
            fprintf(e, b"C %s\n\0" as *const u8 as *const libc::c_char, z1);
            ubuffree(z1);
        }
        if !(strchr((*qtrans).s.zoptions, 'N' as i32)).is_null() {
            fprintf(e, b"N\n\0" as *const u8 as *const libc::c_char);
        }
        if !(strchr((*qtrans).s.zoptions, 'Z' as i32)).is_null() {
            fprintf(e, b"Z\n\0" as *const u8 as *const libc::c_char);
        }
        if !(strchr((*qtrans).s.zoptions, 'e' as i32)).is_null() {
            fprintf(e, b"e\n\0" as *const u8 as *const libc::c_char);
        }
        fbad = 0 as libc::c_int;
        if fsysdep_sync(e, ztemp) == 0 {
            fclose(e);
            remove(ztemp);
            fbad = 1 as libc::c_int;
        }
        if fbad == 0 {
            if fclose(e) == -(1 as libc::c_int) {
                ulog(
                    LOG_ERROR,
                    b"fclose: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                remove(ztemp);
                fbad = 1 as libc::c_int;
            }
        }
        if fbad == 0 {
            if fsysdep_move_file(
                ztemp,
                zxqtfile,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void as *const libc::c_char,
            ) == 0
            {
                remove(ztemp);
                fbad = 1 as libc::c_int;
            }
        }
        ubuffree(zxqtfile);
        ubuffree(ztemp);
        if fbad != 0 {
            urrec_free(qtrans);
            return 0 as libc::c_int;
        }
    }
    if zerr.is_null()
        && ((*qtrans).s.bcmd as libc::c_int == 'E' as i32
            || (*qinfo).fspool != 0
                && *((*qtrans).s.zto).offset(0 as libc::c_int as isize) as libc::c_int
                    == 'X' as i32)
    {
        (*qdaemon).cxfiles_received += 1;
        (*qdaemon).cxfiles_received;
        if (*qdaemon).irunuuxqt > 0 as libc::c_int
            && (*qdaemon).cxfiles_received >= (*qdaemon).irunuuxqt as libc::c_long
        {
            if fspawn_uuxqt(
                1 as libc::c_int,
                (*(*qdaemon).qsys).uuconf_zname,
                (*qdaemon).zconfig,
            ) != 0
            {
                (*qdaemon).cxfiles_received = 0 as libc::c_int as libc::c_long;
            }
        }
    }
    (*qinfo)
        .fmoved = (zerr == 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int;
    if (*qinfo).freplied != 0 {
        return fqueue_send(qdaemon, qtrans);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn frec_file_send_confirm(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    let mut zsend: *const libc::c_char = 0 as *const libc::c_char;
    let mut ilocal: libc::c_int = 0;
    let mut iremote: libc::c_int = 0;
    if (*qinfo).fmoved == 0 {
        zsend = b"CN5\0" as *const u8 as *const libc::c_char;
    } else if (*qdaemon).frequest_hangup == 0 {
        zsend = b"CY\0" as *const u8 as *const libc::c_char;
    } else {
        if (*qdaemon).fmaster != 0 {
            ulog(
                LOG_FATAL,
                b"frec_file_send_confirm: Can't happen\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"frec_send_file_confirm: Requesting remote to transfer control\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        zsend = b"CYM\0" as *const u8 as *const libc::c_char;
    }
    if (*qinfo).flocal == 0 && (*qinfo).fmoved != 0 {
        usent_receive_ack(qdaemon, qtrans);
    }
    ilocal = (*qtrans).ilocal;
    iremote = (*qtrans).iremote;
    urrec_free(qtrans);
    return (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, zsend, ilocal, iremote);
}
pub unsafe extern "C" fn frec_discard_temp(
    mut qdaemon: *mut sdaemon,
    mut qtrans: *mut stransfer,
) -> boolean {
    let mut qinfo: *mut srecinfo = (*qtrans).pinfo as *mut srecinfo;
    if (*qdaemon).ifeatures & 0o2 as libc::c_int == 0 as libc::c_int
        || ((*qtrans).s.ztemp).is_null()
        || *((*qtrans).s.ztemp).offset(0 as libc::c_int as isize) as libc::c_int
            != 'D' as i32
        || strcmp((*qtrans).s.ztemp, b"D.0\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        remove((*qinfo).ztemp);
    }
    return 1 as libc::c_int;
}
