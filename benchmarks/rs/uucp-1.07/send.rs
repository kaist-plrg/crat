use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sconnection;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn cmax_size_ever(qtimesize: *const uuconf_timespan) -> libc::c_long;
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
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_file_exists(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_did_work(pseq: pointer) -> boolean;
    fn zsysdep_save_temp_file(pseq: pointer) -> *const libc::c_char;
    fn zsysdep_add_base(
        zfile: *const libc::c_char,
        zname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn zsysdep_spool_file_name(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        pseq: pointer,
    ) -> *mut libc::c_char;
    fn esysdep_open_send(
        qsys: *const uuconf_system,
        zname: *const libc::c_char,
        fcheck: boolean,
        zuser: *const libc::c_char,
    ) -> openfile_t;
    fn zsysdep_data_file_name(
        qsys: *const uuconf_system,
        zlocalname: *const libc::c_char,
        bgrade: libc::c_int,
        fxqt: boolean,
        ztname: *mut libc::c_char,
        zdname: *mut libc::c_char,
        zxname: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn ixsysdep_file_mode(zfile: *const libc::c_char) -> libc::c_uint;
    fn csysdep_size(zfile: *const libc::c_char) -> libc::c_long;
    fn qtransalc(qcmd: *mut scmd) -> *mut stransfer;
    fn utransfree(qtrans: *mut stransfer);
    fn fqueue_local(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_remote(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_send(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_receive(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
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
pub struct ssendinfo {
    pub zmail: *mut libc::c_char,
    pub zfile: *mut libc::c_char,
    pub cbytes: libc::c_long,
    pub flocal: boolean,
    pub fspool: boolean,
    pub fsent: boolean,
    pub fnever: boolean,
    pub zexec: *mut libc::c_char,
    pub zconfirm: *mut libc::c_char,
}
pub static mut send_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: send.c,v 1.57 2002/03/05 19:10:41 ian Rel $\0")
};
unsafe extern "C" fn usfree_send(mut qtrans: *mut stransfer) {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    if !qinfo.is_null() {
        ubuffree((*qinfo).zmail);
        ubuffree((*qinfo).zfile);
        ubuffree((*qinfo).zexec);
        ubuffree((*qinfo).zconfirm);
        xfree((*qtrans).pinfo);
    }
    utransfree(qtrans);
}
pub unsafe extern "C" fn flocal_send_file_init(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
) -> boolean {
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut fspool: boolean = 0;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cbytes: libc::c_long = 0;
    let mut qinfo: *mut ssendinfo = 0 as *mut ssendinfo;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    qsys = (*qdaemon).qsys;
    if if (*qdaemon).fcaller != 0 {
        ((*qsys).uuconf_fcall_transfer == 0) as libc::c_int
    } else {
        ((*qsys).uuconf_fcalled_transfer == 0) as libc::c_int
    } != 0
    {
        if (*qsys).uuconf_fcall_transfer == 0 && (*qsys).uuconf_fcalled_transfer == 0 {
            return flocal_send_fail(
                qcmd,
                qdaemon,
                b"not permitted to transfer files\0" as *const u8 as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    if (strchr((*qcmd).zoptions, 'C' as i32)).is_null()
        && fspool_file((*qcmd).zfrom) == 0
    {
        fspool = 0 as libc::c_int;
        if fin_directory_list(
            (*qcmd).zfrom,
            (*qsys).uuconf_pzlocal_send,
            (*qsys).uuconf_zpubdir,
            1 as libc::c_int,
            1 as libc::c_int,
            (*qcmd).zuser,
        ) == 0
        {
            return flocal_send_fail(
                qcmd,
                qdaemon,
                b"not permitted to send\0" as *const u8 as *const libc::c_char,
            );
        }
        zfile = zbufcpy((*qcmd).zfrom);
    } else {
        fspool = 1 as libc::c_int;
        zfile = zsysdep_spool_file_name(qsys, (*qcmd).ztemp, (*qcmd).pseq);
        if zfile.is_null() {
            return 0 as libc::c_int;
        }
    }
    cbytes = csysdep_size(zfile);
    if cbytes < 0 as libc::c_int as libc::c_long {
        ubuffree(zfile);
        if cbytes != -(1 as libc::c_int) as libc::c_long {
            return flocal_send_fail(
                qcmd,
                qdaemon,
                b"can not get size\0" as *const u8 as *const libc::c_char,
            );
        }
        if fspool == 0 {
            return flocal_send_fail(
                qcmd,
                qdaemon,
                b"does not exist\0" as *const u8 as *const libc::c_char,
            );
        }
        fsysdep_did_work((*qcmd).pseq);
        return 1 as libc::c_int;
    }
    if (*qdaemon).clocal_size != -(1 as libc::c_int) as libc::c_long
        && (*qdaemon).clocal_size < cbytes
    {
        ubuffree(zfile);
        if (*qdaemon).cmax_ever == -(2 as libc::c_int) as libc::c_long {
            let mut c1: libc::c_long = 0;
            let mut c2: libc::c_long = 0;
            c1 = cmax_size_ever((*qsys).uuconf_qcall_local_size);
            c2 = cmax_size_ever((*qsys).uuconf_qcalled_local_size);
            if c1 > c2 {
                (*qdaemon).cmax_ever = c1;
            } else {
                (*qdaemon).cmax_ever = c2;
            }
        }
        if (*qdaemon).cmax_ever != -(1 as libc::c_int) as libc::c_long
            && (*qdaemon).cmax_ever < (*qcmd).cbytes
        {
            return flocal_send_fail(
                qcmd,
                qdaemon,
                b"too large to send\0" as *const u8 as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    qinfo = xmalloc(::std::mem::size_of::<ssendinfo>() as libc::c_ulong)
        as *mut ssendinfo;
    if (strchr((*qcmd).zoptions, 'm' as i32)).is_null() {
        (*qinfo).zmail = 0 as *mut libc::c_char;
    } else {
        (*qinfo).zmail = zbufcpy((*qcmd).zuser);
    }
    (*qinfo).zfile = zfile;
    (*qinfo).cbytes = cbytes;
    (*qinfo)
        .flocal = (strchr((*qcmd).zuser, '!' as i32)
        == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    (*qinfo).fspool = fspool;
    (*qinfo).fsent = 0 as libc::c_int;
    (*qinfo).zexec = 0 as *mut libc::c_char;
    (*qinfo).zconfirm = 0 as *mut libc::c_char;
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        flocal_send_request
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans).pinfo = qinfo as pointer;
    return fqueue_local(qdaemon, qtrans);
}
unsafe extern "C" fn flocal_send_fail(
    mut qcmd: *mut scmd,
    mut qdaemon: *mut sdaemon,
    mut zwhy: *const libc::c_char,
) -> boolean {
    if !zwhy.is_null() {
        let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
        let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ztemp: *const libc::c_char = 0 as *const libc::c_char;
        if (*qcmd).bcmd as libc::c_int != 'E' as i32 {
            zfrom = (*qcmd).zfrom;
            zfree = 0 as *mut libc::c_char;
        } else {
            zfree = zbufalc(
                (strlen((*qcmd).zfrom))
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong,
                    )
                    .wrapping_add(strlen((*qcmd).zcmd)),
            );
            sprintf(
                zfree,
                b"%s (execution of \"%s\")\0" as *const u8 as *const libc::c_char,
                (*qcmd).zfrom,
                (*qcmd).zcmd,
            );
            zfrom = zfree;
        }
        ulog(LOG_ERROR, b"%s: %s\0" as *const u8 as *const libc::c_char, zfrom, zwhy);
        if (strchr((*qcmd).zuser, '!' as i32)).is_null() {
            ztemp = zsysdep_save_temp_file((*qcmd).pseq);
        } else {
            ztemp = 0 as *const libc::c_char;
        }
        fmail_transfer(
            0 as libc::c_int,
            (*qcmd).zuser,
            0 as *mut libc::c_void as *const libc::c_char,
            zwhy,
            zfrom,
            0 as *mut libc::c_void as *const libc::c_char,
            (*qcmd).zto,
            (*(*qdaemon).qsys).uuconf_zname,
            ztemp,
        );
        ubuffree(zfree);
    }
    fsysdep_did_work((*qcmd).pseq);
    return 1 as libc::c_int;
}
unsafe extern "C" fn flocal_send_request(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
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
    let mut znotify: *const libc::c_char = 0 as *const libc::c_char;
    let mut absize: [libc::c_char; 20] = [0; 20];
    let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    if (*qdaemon).cmax_receive != -(1 as libc::c_int) as libc::c_long
        && (*qdaemon).cmax_receive < (*qinfo).cbytes
    {
        fret = flocal_send_fail(
            &mut (*qtrans).s,
            qdaemon,
            b"too large for receiver\0" as *const u8 as *const libc::c_char,
        );
        usfree_send(qtrans);
        return fret;
    }
    if fsysdep_file_exists((*qinfo).zfile) == 0 {
        fsysdep_did_work((*qtrans).s.pseq);
        usfree_send(qtrans);
        return 1 as libc::c_int;
    }
    (*qtrans).fcmd = 1 as libc::c_int;
    (*qtrans)
        .psendfn = Some(
        flocal_send_open_file
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans)
        .precfn = Some(
        flocal_send_await_reply
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    if (*qdaemon).cchans > 1 as libc::c_int {
        fret = fqueue_send(qdaemon, qtrans);
    } else {
        fret = fqueue_receive(qdaemon, qtrans);
    }
    if fret == 0 {
        return 0 as libc::c_int;
    }
    fquote = fcmd_needs_quotes(&mut (*qtrans).s);
    if fquote == 0 {
        qcmd = &mut (*qtrans).s;
    } else {
        if (*qdaemon).ifeatures & 0o40 as libc::c_int == 0 as libc::c_int {
            fret = flocal_send_fail(
                &mut (*qtrans).s,
                qdaemon,
                b"remote system does not support required quoting\0" as *const u8
                    as *const libc::c_char,
            );
            usfree_send(qtrans);
            return fret;
        }
        uquote_cmd(&mut (*qtrans).s, &mut squoted);
        qcmd = &mut squoted;
    }
    znotify = (*qcmd).znotify;
    if znotify.is_null() {
        znotify = b"\0" as *const u8 as *const libc::c_char;
    }
    if (*qdaemon).ifeatures & 0o1 as libc::c_int != 0 as libc::c_int
        || (*qcmd).bcmd as libc::c_int == 'E' as i32
            && (*qdaemon).ifeatures & 0o4 as libc::c_int != 0 as libc::c_int
    {
        if *znotify as libc::c_int == '\0' as i32 {
            znotify = b"\"\"\0" as *const u8 as *const libc::c_char;
        }
    } else if strcmp(znotify, b"\"\"\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        znotify = b"\0" as *const u8 as *const libc::c_char;
    }
    if (*qdaemon).ifeatures & 0o1 as libc::c_int == 0 as libc::c_int
        && ((*qcmd).bcmd as libc::c_int != 'E' as i32
            || (*qdaemon).ifeatures & 0o4 as libc::c_int == 0 as libc::c_int)
    {
        absize[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else if (*qdaemon).ifeatures & 0o10 as libc::c_int == 0 as libc::c_int {
        sprintf(
            absize.as_mut_ptr(),
            b"0x%lx\0" as *const u8 as *const libc::c_char,
            (*qinfo).cbytes as libc::c_ulong,
        );
    } else {
        sprintf(
            absize.as_mut_ptr(),
            b"%ld\0" as *const u8 as *const libc::c_char,
            (*qinfo).cbytes,
        );
    }
    zsend = zbufalc(
        (strlen((*qcmd).zfrom))
            .wrapping_add(strlen((*qcmd).zto))
            .wrapping_add(strlen((*qcmd).zuser))
            .wrapping_add(strlen((*qcmd).zoptions))
            .wrapping_add(strlen((*qcmd).ztemp))
            .wrapping_add(strlen(znotify))
            .wrapping_add(strlen(absize.as_mut_ptr()))
            .wrapping_add(
                (if !((*qcmd).zcmd).is_null() {
                    strlen((*qcmd).zcmd)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }),
            )
            .wrapping_add(50 as libc::c_int as libc::c_ulong),
    );
    if (*qcmd).bcmd as libc::c_int == 'E' as i32
        && (*qdaemon).ifeatures & 0o4 as libc::c_int != 0 as libc::c_int
    {
        sprintf(
            zsend,
            b"E %s %s %s -%s %s 0%o %s %s %s\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
            (*qcmd).zto,
            (*qcmd).zuser,
            (*qcmd).zoptions,
            (*qcmd).ztemp,
            (*qcmd).imode,
            znotify,
            absize.as_mut_ptr(),
            (*qcmd).zcmd,
        );
    } else {
        let mut zoptions: *const libc::c_char = 0 as *const libc::c_char;
        let mut zdummy: *const libc::c_char = 0 as *const libc::c_char;
        if (*qcmd).bcmd as libc::c_int != 'E' as i32 {
            zoptions = (*qcmd).zoptions;
        } else if !(strchr((*qcmd).zoptions, 'C' as i32)).is_null() {
            zoptions = b"\0" as *const u8 as *const libc::c_char;
        } else {
            zoptions = b"c\0" as *const u8 as *const libc::c_char;
        }
        if (*qdaemon).ifeatures & 0o20 as libc::c_int != 0 as libc::c_int {
            zdummy = b" dummy \0" as *const u8 as *const libc::c_char;
        } else {
            zdummy = b" \0" as *const u8 as *const libc::c_char;
        }
        sprintf(
            zsend,
            b"S %s %s %s -%s %s 0%o %s%s%s\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
            (*qcmd).zto,
            (*qcmd).zuser,
            zoptions,
            (*qcmd).ztemp,
            (*qcmd).imode,
            znotify,
            zdummy,
            absize.as_mut_ptr(),
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
unsafe extern "C" fn flocal_send_await_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    let mut bcmd: libc::c_char = 0;
    if (*qtrans).s.bcmd as libc::c_int == 'E' as i32
        && (*qdaemon).ifeatures & 0o4 as libc::c_int != 0 as libc::c_int
    {
        bcmd = 'E' as i32 as libc::c_char;
    } else {
        bcmd = 'S' as i32 as libc::c_char;
    }
    if *zdata.offset(0 as libc::c_int as isize) as libc::c_int != bcmd as libc::c_int
        || *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'Y' as i32
            && *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'N' as i32
    {
        ulog(
            LOG_ERROR,
            b"%s: Bad response to %c request: \"%s\"\0" as *const u8
                as *const libc::c_char,
            (*qtrans).s.zfrom,
            bcmd as libc::c_int,
            zdata,
        );
        usfree_send(qtrans);
        return 0 as libc::c_int;
    }
    if *zdata.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
        let mut zerr: *const libc::c_char = 0 as *const libc::c_char;
        let mut fnever: boolean = 0;
        fnever = 1 as libc::c_int;
        if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '2' as i32 {
            zerr = b"permission denied by remote\0" as *const u8 as *const libc::c_char;
        } else if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '4' as i32 {
            zerr = b"remote cannot create work files\0" as *const u8
                as *const libc::c_char;
            fnever = 0 as libc::c_int;
        } else if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '6' as i32 {
            zerr = b"too large for remote now\0" as *const u8 as *const libc::c_char;
            fnever = 0 as libc::c_int;
        } else if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '7' as i32 {
            zerr = b"too large for remote\0" as *const u8 as *const libc::c_char;
        } else if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '8' as i32 {
            zerr = 0 as *const libc::c_char;
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
        if fnever == 0
            || (*qtrans).s.bcmd as libc::c_int == 'E' as i32
                && (*qdaemon).ifeatures & 0o4 as libc::c_int == 0 as libc::c_int
                && ((*qinfo).zexec).is_null()
        {
            if (*qtrans).s.bcmd as libc::c_int == 'E' as i32 {
                ulog(
                    LOG_ERROR,
                    b"%s (execution of \"%s\"): %s\0" as *const u8
                        as *const libc::c_char,
                    (*qtrans).s.zfrom,
                    (*qtrans).s.zcmd,
                    zerr,
                );
            } else {
                ulog(
                    LOG_ERROR,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    (*qtrans).s.zfrom,
                    zerr,
                );
            }
        } else if flocal_send_fail(&mut (*qtrans).s, qdaemon, zerr) == 0 {
            return 0 as libc::c_int
        }
        if (*qdaemon).cchans == 1 as libc::c_int || (*qinfo).fsent != 0 {
            if fnever != 0 && (*qtrans).s.bcmd as libc::c_int == 'E' as i32
                && (*qdaemon).ifeatures & 0o4 as libc::c_int == 0 as libc::c_int
                && ((*qinfo).zexec).is_null()
            {
                return fsend_exec_file_init(qtrans, qdaemon);
            }
            usfree_send(qtrans);
            return 1 as libc::c_int;
        } else {
            if (*qtrans).fsendfile != 0
                && !(fseek(
                    (*qtrans).e,
                    0 as libc::c_int as libc::c_long,
                    2 as libc::c_int,
                ) == 0 as libc::c_int)
            {
                ulog(
                    LOG_ERROR,
                    b"seek to end: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                usfree_send(qtrans);
                return 0 as libc::c_int;
            }
            (*qtrans)
                .psendfn = Some(
                flocal_send_cancelled
                    as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
            );
            (*qtrans).precfn = None;
            (*qinfo).fnever = fnever;
            return fqueue_send(qdaemon, qtrans);
        }
    }
    if *zdata.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        let mut cskip: libc::c_long = 0;
        cskip = strtol(
            zdata.offset(2 as libc::c_int as isize) as *mut libc::c_char,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            0 as libc::c_int,
        );
        if cskip > 0 as libc::c_int as libc::c_long && (*qtrans).ipos < cskip {
            if (*qtrans).fsendfile != 0 && (*qinfo).fsent == 0 {
                if !(fseek((*qtrans).e, cskip, 0 as libc::c_int) == 0 as libc::c_int) {
                    ulog(
                        LOG_ERROR,
                        b"seek: %s\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                    usfree_send(qtrans);
                    return 0 as libc::c_int;
                }
            }
            (*qtrans).ipos = cskip;
        }
    }
    (*qtrans).fcmd = 1 as libc::c_int;
    (*qtrans)
        .precfn = Some(
        fsend_await_confirm
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    if (*qinfo).fsent != 0 {
        return fqueue_receive(qdaemon, qtrans)
    } else if (*qdaemon).cchans <= 1 as libc::c_int {
        return fqueue_send(qdaemon, qtrans)
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn flocal_send_open_file(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    let mut zuser: *const libc::c_char = 0 as *const libc::c_char;
    if ((*qinfo).zexec).is_null() {
        zuser = (*qtrans).s.zuser;
        if !(strchr(zuser, '!' as i32)).is_null() {
            zuser = 0 as *const libc::c_char;
        }
        (*qtrans)
            .e = esysdep_open_send(
            (*qdaemon).qsys,
            (*qinfo).zfile,
            ((*qinfo).fspool == 0) as libc::c_int,
            zuser,
        );
        if ((*qtrans).e).is_null() {
            fmail_transfer(
                0 as libc::c_int,
                (*qtrans).s.zuser,
                0 as *mut libc::c_void as *const libc::c_char,
                b"cannot open file\0" as *const u8 as *const libc::c_char,
                (*qtrans).s.zfrom,
                0 as *mut libc::c_void as *const libc::c_char,
                (*qtrans).s.zto,
                (*(*qdaemon).qsys).uuconf_zname,
                if (*qinfo).flocal != 0 {
                    zsysdep_save_temp_file((*qtrans).s.pseq)
                } else {
                    0 as *mut libc::c_void as *const libc::c_char
                },
            );
            fsysdep_did_work((*qtrans).s.pseq);
            usfree_send(qtrans);
            return 0 as libc::c_int;
        }
    }
    if (*qtrans).ipos > 0 as libc::c_int as libc::c_long {
        if !((*qinfo).zexec).is_null() {
            if (*qtrans).ipos > (*qtrans).cbytes {
                (*qtrans).ipos = (*qtrans).cbytes;
            }
        } else if !(fseek((*qtrans).e, (*qtrans).ipos, 0 as libc::c_int)
            == 0 as libc::c_int)
        {
            ulog(
                LOG_ERROR,
                b"seek: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            usfree_send(qtrans);
            return 0 as libc::c_int;
        }
    }
    if ((*qinfo).zexec).is_null() {
        let mut zsend: *const libc::c_char = 0 as *const libc::c_char;
        let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*qtrans).s.bcmd as libc::c_int != 'E' as i32 {
            zsend = (*qtrans).s.zfrom;
            zalc = 0 as *mut libc::c_char;
        } else {
            zalc = zbufalc(
                (strlen((*qtrans).s.zcmd))
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
                    )
                    .wrapping_add(strlen((*qtrans).s.zfrom)),
            );
            sprintf(
                zalc,
                b"%s (%s)\0" as *const u8 as *const libc::c_char,
                (*qtrans).s.zcmd,
                (*qtrans).s.zfrom,
            );
            zsend = zalc;
        }
        (*qtrans)
            .zlog = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                .wrapping_add(strlen(zsend))
                .wrapping_add(50 as libc::c_int as libc::c_ulong),
        );
        sprintf(
            (*qtrans).zlog,
            b"Sending %s (%ld bytes\0" as *const u8 as *const libc::c_char,
            zsend,
            (*qinfo).cbytes,
        );
        if (*qtrans).ipos > 0 as libc::c_int as libc::c_long {
            sprintf(
                ((*qtrans).zlog).offset(strlen((*qtrans).zlog) as isize),
                b" resume at %ld\0" as *const u8 as *const libc::c_char,
                (*qtrans).ipos,
            );
        }
        strcat((*qtrans).zlog, b")\0" as *const u8 as *const libc::c_char);
        ubuffree(zalc);
    }
    if ((*(*qdaemon).qproto).pffile).is_some() {
        let mut fhandled: boolean = 0;
        if (Some(((*(*qdaemon).qproto).pffile).unwrap()))
            .unwrap()(
            qdaemon,
            qtrans,
            1 as libc::c_int,
            1 as libc::c_int,
            (*qinfo).cbytes - (*qtrans).ipos,
            &mut fhandled,
        ) == 0
        {
            usfree_send(qtrans);
            return 0 as libc::c_int;
        }
        if fhandled != 0 {
            return 1 as libc::c_int;
        }
    }
    if !((*qinfo).zexec).is_null() {
        (*qtrans)
            .psendfn = Some(
            fsend_exec_file
                as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
        );
    } else {
        (*qtrans).fsendfile = 1 as libc::c_int;
        (*qtrans)
            .psendfn = Some(
            fsend_file_end
                as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
        );
    }
    return fqueue_send(qdaemon, qtrans);
}
unsafe extern "C" fn flocal_send_cancelled(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    if (*qinfo).fnever != 0 && (*qtrans).s.bcmd as libc::c_int == 'E' as i32
        && (*qdaemon).ifeatures & 0o4 as libc::c_int == 0 as libc::c_int
        && ((*qinfo).zexec).is_null()
    {
        return fsend_exec_file_init(qtrans, qdaemon);
    }
    usfree_send(qtrans);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fremote_rec_file_init(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
    mut iremote: libc::c_int,
) -> boolean {
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fbadname: boolean = 0;
    let mut cbytes: libc::c_long = 0;
    let mut imode: libc::c_uint = 0;
    let mut e: openfile_t = 0 as *mut FILE;
    let mut qinfo: *mut ssendinfo = 0 as *mut ssendinfo;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    qsys = (*qdaemon).qsys;
    if (*qsys).uuconf_fsend_request == 0 {
        ulog(
            LOG_ERROR,
            b"%s: not permitted to send files to remote\0" as *const u8
                as *const libc::c_char,
            (*qcmd).zfrom,
        );
        return fremote_rec_fail(qdaemon, FAILURE_PERM, iremote);
    }
    if fspool_file((*qcmd).zfrom) != 0 {
        ulog(
            LOG_ERROR,
            b"%s: not permitted to send\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
        );
        return fremote_rec_fail(qdaemon, FAILURE_PERM, iremote);
    }
    zfile = zsysdep_local_file((*qcmd).zfrom, (*qsys).uuconf_zpubdir, &mut fbadname);
    if zfile.is_null() && fbadname != 0 {
        ulog(
            LOG_ERROR,
            b"%s: bad local file name\0" as *const u8 as *const libc::c_char,
            (*qcmd).zfrom,
        );
        return fremote_rec_fail(qdaemon, FAILURE_PERM, iremote);
    }
    if !zfile.is_null() {
        let mut zbased: *mut libc::c_char = 0 as *mut libc::c_char;
        zbased = zsysdep_add_base(zfile, (*qcmd).zto);
        ubuffree(zfile);
        zfile = zbased;
    }
    if zfile.is_null() {
        return fremote_rec_fail(qdaemon, FAILURE_PERM, iremote);
    }
    if fin_directory_list(
        zfile,
        (*qsys).uuconf_pzremote_send,
        (*qsys).uuconf_zpubdir,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
    ) == 0
    {
        ulog(
            LOG_ERROR,
            b"%s: not permitted to send\0" as *const u8 as *const libc::c_char,
            zfile,
        );
        ubuffree(zfile);
        return fremote_rec_fail(qdaemon, FAILURE_PERM, iremote);
    }
    cbytes = csysdep_size(zfile);
    if cbytes != -(1 as libc::c_int) as libc::c_long
        && ((*qcmd).cbytes != -(1 as libc::c_int) as libc::c_long
            && (*qcmd).cbytes < cbytes
            || (*qdaemon).cremote_size != -(1 as libc::c_int) as libc::c_long
                && (*qdaemon).cremote_size < cbytes
            || (*qdaemon).cmax_receive != -(1 as libc::c_int) as libc::c_long
                && (*qdaemon).cmax_receive < cbytes)
    {
        ulog(
            LOG_ERROR,
            b"%s: too large to send\0" as *const u8 as *const libc::c_char,
            zfile,
        );
        ubuffree(zfile);
        return fremote_rec_fail(qdaemon, FAILURE_SIZE, iremote);
    }
    imode = ixsysdep_file_mode(zfile);
    e = esysdep_open_send(
        qsys,
        zfile,
        1 as libc::c_int,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if e.is_null() {
        ubuffree(zfile);
        return fremote_rec_fail(qdaemon, FAILURE_OPEN, iremote);
    }
    if (*qcmd).ipos > 0 as libc::c_int as libc::c_long {
        if !(fseek(e, (*qcmd).ipos, 0 as libc::c_int) == 0 as libc::c_int) {
            ulog(
                LOG_ERROR,
                b"seek: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            ubuffree(zfile);
            return 0 as libc::c_int;
        }
    }
    qinfo = xmalloc(::std::mem::size_of::<ssendinfo>() as libc::c_ulong)
        as *mut ssendinfo;
    (*qinfo).zmail = 0 as *mut libc::c_char;
    (*qinfo).zfile = zfile;
    (*qinfo).cbytes = cbytes;
    (*qinfo).flocal = 0 as libc::c_int;
    (*qinfo).fspool = 0 as libc::c_int;
    (*qinfo).fsent = 0 as libc::c_int;
    (*qinfo).zexec = 0 as *mut libc::c_char;
    (*qinfo).zconfirm = 0 as *mut libc::c_char;
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        fremote_rec_reply
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans).iremote = iremote;
    (*qtrans).pinfo = qinfo as pointer;
    (*qtrans).e = e;
    (*qtrans).ipos = (*qcmd).ipos;
    (*qtrans).s.imode = imode;
    return fqueue_remote(qdaemon, qtrans);
}
unsafe extern "C" fn fremote_rec_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    let mut absend: [libc::c_char; 50] = [0; 50];
    (*qtrans).fsendfile = 1 as libc::c_int;
    (*qtrans)
        .psendfn = Some(
        fsend_file_end as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans).fcmd = 1 as libc::c_int;
    (*qtrans)
        .precfn = Some(
        fsend_await_confirm
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    if fqueue_send(qdaemon, qtrans) == 0 {
        return 0 as libc::c_int;
    }
    (*qtrans)
        .zlog = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_add(strlen((*qtrans).s.zfrom))
            .wrapping_add(25 as libc::c_int as libc::c_ulong),
    );
    sprintf(
        (*qtrans).zlog,
        b"Sending %s (%ld bytes)\0" as *const u8 as *const libc::c_char,
        (*qtrans).s.zfrom,
        (*qinfo).cbytes,
    );
    if (*qdaemon).frequest_hangup != 0 {
        if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fremote_rec_reply: Requesting remote to transfer control\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    sprintf(
        absend.as_mut_ptr(),
        b"RY 0%o%s 0x%lx%s\0" as *const u8 as *const libc::c_char,
        (*qtrans).s.imode,
        if (*qdaemon).frequest_hangup != 0 {
            b"M\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (*qinfo).cbytes as libc::c_ulong,
        if (*qdaemon).frequest_hangup != 0 {
            b"M\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    if (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, absend.as_mut_ptr(), (*qtrans).ilocal, (*qtrans).iremote) == 0
    {
        fclose((*qtrans).e);
        (*qtrans).e = 0 as *mut libc::c_void as *mut FILE;
        return 0 as libc::c_int;
    }
    if ((*(*qdaemon).qproto).pffile).is_some() {
        let mut fhandled: boolean = 0;
        if (Some(((*(*qdaemon).qproto).pffile).unwrap()))
            .unwrap()(
            qdaemon,
            qtrans,
            1 as libc::c_int,
            1 as libc::c_int,
            (*qinfo).cbytes,
            &mut fhandled,
        ) == 0
        {
            usfree_send(qtrans);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fremote_rec_fail(
    mut qdaemon: *mut sdaemon,
    mut twhy: tfailure,
    mut iremote: libc::c_int,
) -> boolean {
    let mut ptinfo: *mut tfailure = 0 as *mut tfailure;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    ptinfo = xmalloc(::std::mem::size_of::<tfailure>() as libc::c_ulong)
        as *mut tfailure;
    *ptinfo = twhy;
    qtrans = qtransalc(0 as *mut libc::c_void as *mut scmd);
    (*qtrans)
        .psendfn = Some(
        fremote_rec_fail_send
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans).iremote = iremote;
    (*qtrans).pinfo = ptinfo as pointer;
    return fqueue_remote(qdaemon, qtrans);
}
unsafe extern "C" fn fremote_rec_fail_send(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut ptinfo: *mut tfailure = (*qtrans).pinfo as *mut tfailure;
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut ilocal: libc::c_int = 0;
    let mut iremote: libc::c_int = 0;
    match *ptinfo as libc::c_uint {
        1 | 2 => {
            z = b"RN2\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            z = b"RN6\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            z = b"RN\0" as *const u8 as *const libc::c_char;
        }
    }
    ilocal = (*qtrans).ilocal;
    iremote = (*qtrans).iremote;
    xfree((*qtrans).pinfo);
    utransfree(qtrans);
    return (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, z, ilocal, iremote);
}
unsafe extern "C" fn fsend_file_end(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    if ((*(*qdaemon).qproto).pffile).is_some() {
        let mut fhandled: boolean = 0;
        if (Some(((*(*qdaemon).qproto).pffile).unwrap()))
            .unwrap()(
            qdaemon,
            qtrans,
            0 as libc::c_int,
            1 as libc::c_int,
            -(1 as libc::c_int) as libc::c_long,
            &mut fhandled,
        ) == 0
        {
            usfree_send(qtrans);
            return 0 as libc::c_int;
        }
        if fhandled != 0 {
            return 1 as libc::c_int;
        }
    }
    (*qinfo).fsent = 1 as libc::c_int;
    if !((*qinfo).zconfirm).is_null() {
        return fsend_await_confirm(
            qtrans,
            qdaemon,
            (*qinfo).zconfirm,
            (strlen((*qinfo).zconfirm)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    return fqueue_receive(qdaemon, qtrans);
}
unsafe extern "C" fn fsend_await_confirm(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    let mut fnever: boolean = 0;
    let mut zerr: *const libc::c_char = 0 as *const libc::c_char;
    if (*qinfo).fsent == 0 {
        (*qinfo).zconfirm = zbufcpy(zdata);
        return 1 as libc::c_int;
    }
    if ((*qinfo).zexec).is_null() {
        fclose((*qtrans).e);
        (*qtrans).e = 0 as *mut libc::c_void as *mut FILE;
    }
    fnever = 0 as libc::c_int;
    if *zdata.offset(0 as libc::c_int as isize) as libc::c_int != 'C' as i32
        || *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'Y' as i32
            && *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'N' as i32
    {
        zerr = b"bad confirmation from remote\0" as *const u8 as *const libc::c_char;
        ulog(
            LOG_ERROR,
            b"%s: %s \"%s\"\0" as *const u8 as *const libc::c_char,
            (*qtrans).s.zfrom,
            zerr,
            zdata,
        );
    } else if *zdata.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
        fnever = 1 as libc::c_int;
        if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == '5' as i32 {
            zerr = b"file could not be stored in final location\0" as *const u8
                as *const libc::c_char;
            ulog(
                LOG_ERROR,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                (*qtrans).s.zfrom,
                zerr,
            );
        } else {
            zerr = b"file send failed for unknown reason\0" as *const u8
                as *const libc::c_char;
            ulog(
                LOG_ERROR,
                b"%s: %s \"%s\"\0" as *const u8 as *const libc::c_char,
                (*qtrans).s.zfrom,
                zerr,
                zdata,
            );
        }
    } else {
        zerr = 0 as *const libc::c_char;
        if *zdata.offset(2 as libc::c_int as isize) as libc::c_int == 'M' as i32
            && (*qdaemon).fmaster != 0
        {
            if iDebug & 0o10 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG,
                    b"fsend_await_confirm: Remote has requested transfer of control\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            (*qdaemon).fhangup_requested = 1 as libc::c_int;
        }
    }
    ustats(
        (zerr == 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int,
        (*qtrans).s.zuser,
        (*(*qdaemon).qsys).uuconf_zname,
        1 as libc::c_int,
        (*qtrans).cbytes,
        (*qtrans).isecs,
        (*qtrans).imicros,
        (*qdaemon).fcaller,
    );
    (*qdaemon).csent += (*qtrans).cbytes;
    if zerr.is_null() {
        if (*qtrans).s.bcmd as libc::c_int == 'E' as i32
            && (*qdaemon).ifeatures & 0o4 as libc::c_int == 0 as libc::c_int
            && ((*qinfo).zexec).is_null()
        {
            return fsend_exec_file_init(qtrans, qdaemon);
        }
        if !((*qinfo).zmail).is_null() && *(*qinfo).zmail as libc::c_int != '\0' as i32 {
            fmail_transfer(
                1 as libc::c_int,
                (*qtrans).s.zuser,
                (*qinfo).zmail,
                0 as *mut libc::c_void as *const libc::c_char,
                (*qtrans).s.zfrom,
                0 as *mut libc::c_void as *const libc::c_char,
                (*qtrans).s.zto,
                (*(*qdaemon).qsys).uuconf_zname,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        }
        if !((*qtrans).s.pseq).is_null() {
            fsysdep_did_work((*qtrans).s.pseq);
        }
    } else if fnever != 0 && (*qinfo).flocal != 0 {
        fmail_transfer(
            0 as libc::c_int,
            (*qtrans).s.zuser,
            (*qinfo).zmail,
            zerr,
            (*qtrans).s.zfrom,
            0 as *mut libc::c_void as *const libc::c_char,
            (*qtrans).s.zto,
            (*(*qdaemon).qsys).uuconf_zname,
            zsysdep_save_temp_file((*qtrans).s.pseq),
        );
        fsysdep_did_work((*qtrans).s.pseq);
    }
    usfree_send(qtrans);
    return 1 as libc::c_int;
}
unsafe extern "C" fn fsend_exec_file_init(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    let mut zxqtfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abtname: [libc::c_char; 15] = [0; 15];
    let mut abxname: [libc::c_char; 15] = [0; 15];
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut calc: size_t = 0;
    let mut clen: size_t = 0;
    let mut fquote: boolean = 0;
    z = 0 as *mut libc::c_char;
    calc = 0 as libc::c_int as size_t;
    clen = 0 as libc::c_int as size_t;
    fquote = fcmd_needs_quotes(&mut (*qtrans).s);
    if fquote != 0 {
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'Q' as i32,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    usadd_exec_line(
        &mut z,
        &mut calc,
        &mut clen,
        'U' as i32,
        (*qtrans).s.zuser,
        (*qdaemon).zlocalname,
        fquote,
    );
    usadd_exec_line(
        &mut z,
        &mut calc,
        &mut clen,
        'F' as i32,
        (*qtrans).s.zto,
        b"\0" as *const u8 as *const libc::c_char,
        fquote,
    );
    usadd_exec_line(
        &mut z,
        &mut calc,
        &mut clen,
        'I' as i32,
        (*qtrans).s.zto,
        b"\0" as *const u8 as *const libc::c_char,
        fquote,
    );
    if !(strchr((*qtrans).s.zoptions, 'N' as i32)).is_null() {
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'N' as i32,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            fquote,
        );
    }
    if !(strchr((*qtrans).s.zoptions, 'Z' as i32)).is_null() {
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'Z' as i32,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            fquote,
        );
    }
    if !(strchr((*qtrans).s.zoptions, 'R' as i32)).is_null() {
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'R' as i32,
            (*qtrans).s.znotify,
            b"\0" as *const u8 as *const libc::c_char,
            fquote,
        );
    }
    if !(strchr((*qtrans).s.zoptions, 'e' as i32)).is_null() {
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'e' as i32,
            b"\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            fquote,
        );
    }
    if fquote == 0 {
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'C' as i32,
            (*qtrans).s.zcmd,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        let mut zquoted: *mut libc::c_char = 0 as *mut libc::c_char;
        zquoted = zquote_cmd_string((*qtrans).s.zcmd, 1 as libc::c_int);
        usadd_exec_line(
            &mut z,
            &mut calc,
            &mut clen,
            'C' as i32,
            zquoted,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        ubuffree(zquoted);
    }
    (*qinfo).zexec = z;
    (*qinfo).cbytes = clen as libc::c_long;
    zxqtfile = zsysdep_data_file_name(
        (*qdaemon).qsys,
        (*qdaemon).zlocalname,
        'N' as i32,
        1 as libc::c_int,
        abtname.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        abxname.as_mut_ptr(),
    );
    if zxqtfile.is_null() {
        usfree_send(qtrans);
        return 0 as libc::c_int;
    }
    ubuffree(zxqtfile);
    ubuffree((*qtrans).s.zfrom as *mut libc::c_char);
    (*qtrans).s.zfrom = zbufcpy(abtname.as_mut_ptr());
    ubuffree((*qtrans).s.zto as *mut libc::c_char);
    (*qtrans).s.zto = zbufcpy(abxname.as_mut_ptr());
    ubuffree((*qtrans).s.zoptions as *mut libc::c_char);
    (*qtrans).s.zoptions = zbufcpy(b"C\0" as *const u8 as *const libc::c_char);
    ubuffree((*qtrans).s.ztemp as *mut libc::c_char);
    (*qtrans).s.ztemp = zbufcpy(abtname.as_mut_ptr());
    (*qtrans)
        .psendfn = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> boolean>,
        Option::<unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean>,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
                unsafe extern "C" fn() -> boolean,
            >(flocal_send_request),
        ),
    );
    (*qtrans).precfn = None;
    (*qtrans).ipos = 0 as libc::c_int as libc::c_long;
    (*qtrans).cbytes = 0 as libc::c_int as libc::c_long;
    (*qtrans).isecs = 0 as libc::c_int as libc::c_long;
    (*qtrans).imicros = 0 as libc::c_int as libc::c_long;
    (*qinfo).fsent = 0 as libc::c_int;
    ubuffree((*qinfo).zconfirm);
    (*qinfo).zconfirm = 0 as *mut libc::c_char;
    return fqueue_send(qdaemon, qtrans);
}
unsafe extern "C" fn usadd_exec_line(
    mut pz: *mut *mut libc::c_char,
    mut pcalc: *mut size_t,
    mut pclen: *mut size_t,
    mut bcmd: libc::c_int,
    mut z1: *const libc::c_char,
    mut z2: *const libc::c_char,
    mut fquote: boolean,
) {
    let mut z1q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z2q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c1: size_t = 0;
    let mut c2: size_t = 0;
    let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
    z1q = 0 as *mut libc::c_char;
    z2q = 0 as *mut libc::c_char;
    if fquote != 0 {
        if *z1 as libc::c_int != '\0' as i32 {
            z1q = zquote_cmd_string(z1, 0 as libc::c_int);
            z1 = z1q;
        }
        if *z2 as libc::c_int != '\0' as i32 {
            z2q = zquote_cmd_string(z2, 0 as libc::c_int);
            z2 = z2q;
        }
    }
    c1 = strlen(z1);
    c2 = strlen(z2);
    if (*pclen)
        .wrapping_add(c1)
        .wrapping_add(c2)
        .wrapping_add(4 as libc::c_int as libc::c_ulong) >= *pcalc
    {
        *pcalc = (*pcalc as libc::c_ulong)
            .wrapping_add(
                c1.wrapping_add(c2).wrapping_add(100 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        znew = zbufalc(*pcalc);
        if *pclen > 0 as libc::c_int as libc::c_ulong {
            memcpy(znew as *mut libc::c_void, *pz as *const libc::c_void, *pclen);
            ubuffree(*pz);
        }
        *pz = znew;
    }
    znew = (*pz).offset(*pclen as isize);
    let fresh0 = znew;
    znew = znew.offset(1);
    *fresh0 = bcmd as libc::c_char;
    if *z1 as libc::c_int != '\0' as i32 {
        let fresh1 = znew;
        znew = znew.offset(1);
        *fresh1 = ' ' as i32 as libc::c_char;
        memcpy(znew as *mut libc::c_void, z1 as *const libc::c_void, c1);
        znew = znew.offset(c1 as isize);
        if *z2 as libc::c_int != '\0' as i32 {
            let fresh2 = znew;
            znew = znew.offset(1);
            *fresh2 = ' ' as i32 as libc::c_char;
            memcpy(znew as *mut libc::c_void, z2 as *const libc::c_void, c2);
            znew = znew.offset(c2 as isize);
        }
    }
    if fquote != 0 {
        ubuffree(z1q);
        ubuffree(z2q);
    }
    let fresh3 = znew;
    znew = znew.offset(1);
    *fresh3 = '\n' as i32 as libc::c_char;
    *pclen = znew.offset_from(*pz) as libc::c_long as size_t;
}
unsafe extern "C" fn fsend_exec_file(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut qinfo: *mut ssendinfo = (*qtrans).pinfo as *mut ssendinfo;
    let mut zdata: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cdata: size_t = 0;
    let mut csend: size_t = 0;
    zdata = (Some(((*(*qdaemon).qproto).pzgetspace).unwrap()))
        .unwrap()(qdaemon, &mut cdata);
    if zdata.is_null() {
        usfree_send(qtrans);
        return 0 as libc::c_int;
    }
    csend = ((*qinfo).cbytes - (*qtrans).ipos) as size_t;
    if csend > cdata {
        csend = cdata;
    }
    memcpy(
        zdata as *mut libc::c_void,
        ((*qinfo).zexec).offset((*qtrans).ipos as isize) as *const libc::c_void,
        csend,
    );
    if (Some(((*(*qdaemon).qproto).pfsenddata).unwrap()))
        .unwrap()(
        qdaemon,
        zdata,
        csend,
        (*qtrans).ilocal,
        (*qtrans).iremote,
        (*qtrans).ipos,
    ) == 0
    {
        usfree_send(qtrans);
        return 0 as libc::c_int;
    }
    (*qtrans)
        .cbytes = ((*qtrans).cbytes as libc::c_ulong).wrapping_add(csend) as libc::c_long
        as libc::c_long;
    (*qtrans)
        .ipos = ((*qtrans).ipos as libc::c_ulong).wrapping_add(csend) as libc::c_long
        as libc::c_long;
    if csend == 0 as libc::c_int as libc::c_ulong {
        return fsend_file_end(qtrans, qdaemon);
    }
    return 1 as libc::c_int;
}
