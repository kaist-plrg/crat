use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sconnection;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn funknown_system(
        puuconf: pointer,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> boolean;
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
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn fcopy_file(
        zfrom: *const libc::c_char,
        zto: *const libc::c_char,
        fpublic: boolean,
        fmkdirs: boolean,
        fsignals: boolean,
    ) -> boolean;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn uuconf_system_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zsystem: *const libc::c_char,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn zsysdep_local_file(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_file_exists(zfile: *const libc::c_char) -> boolean;
    fn fsysdep_did_work(pseq: pointer) -> boolean;
    fn zsysdep_add_base(
        zfile: *const libc::c_char,
        zname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fsysdep_wildcard_start(zfile: *const libc::c_char) -> boolean;
    fn zsysdep_wildcard(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn fsysdep_wildcard_end() -> boolean;
    fn zsysdep_spool_commands(
        qsys: *const uuconf_system,
        bgrade: libc::c_int,
        ccmds: libc::c_int,
        pascmds: *const scmd,
        pftemp: *mut boolean,
    ) -> *mut libc::c_char;
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
    fn qtransalc(qcmd: *mut scmd) -> *mut stransfer;
    fn utransfree(qtrans: *mut stransfer);
    fn fqueue_local(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
    fn fqueue_remote(qdaemon: *mut sdaemon, qtrans: *mut stransfer) -> boolean;
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
pub static mut xcmd_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: xcmd.c,v 1.24 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn flocal_xcmd_init(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
) -> boolean {
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        flocal_xcmd_request
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    return fqueue_local(qdaemon, qtrans);
}
unsafe extern "C" fn flocal_xcmd_request(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
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
    ulog(
        LOG_NORMAL,
        b"Requesting work: %s to %s\0" as *const u8 as *const libc::c_char,
        (*qtrans).s.zfrom,
        (*qtrans).s.zto,
    );
    (*qtrans).fcmd = 1 as libc::c_int;
    (*qtrans)
        .precfn = Some(
        flocal_xcmd_await_reply
            as unsafe extern "C" fn(
                *mut stransfer,
                *mut sdaemon,
                *const libc::c_char,
                size_t,
            ) -> boolean,
    );
    fquote = fcmd_needs_quotes(&mut (*qtrans).s);
    if fquote == 0 {
        qcmd = &mut (*qtrans).s;
    } else {
        if (*qdaemon).ifeatures & 0o40 as libc::c_int == 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"%s: remote system does not support required quoting\0" as *const u8
                    as *const libc::c_char,
                (*qtrans).s.zfrom,
            );
            fmail_transfer(
                0 as libc::c_int,
                (*qtrans).s.zuser,
                0 as *mut libc::c_void as *const libc::c_char,
                b"remote system does not support required quoting\0" as *const u8
                    as *const libc::c_char,
                (*qtrans).s.zfrom,
                (*(*qdaemon).qsys).uuconf_zname,
                (*qtrans).s.zto,
                0 as *mut libc::c_void as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            fsysdep_did_work((*qtrans).s.pseq);
            utransfree(qtrans);
            return 1 as libc::c_int;
        }
        uquote_cmd(&mut (*qtrans).s, &mut squoted);
        qcmd = &mut squoted;
    }
    if fqueue_receive(qdaemon, qtrans) == 0 {
        return 0 as libc::c_int;
    }
    clen = (strlen((*qcmd).zfrom))
        .wrapping_add(strlen((*qcmd).zto))
        .wrapping_add(strlen((*qcmd).zuser))
        .wrapping_add(strlen((*qcmd).zoptions))
        .wrapping_add(7 as libc::c_int as libc::c_ulong);
    zsend = zbufalc(clen);
    sprintf(
        zsend,
        b"X %s %s %s -%s\0" as *const u8 as *const libc::c_char,
        (*qcmd).zfrom,
        (*qcmd).zto,
        (*qcmd).zuser,
        (*qcmd).zoptions,
    );
    fret = (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(qdaemon, zsend, (*qtrans).ilocal, (*qtrans).iremote);
    ubuffree(zsend);
    if fquote != 0 {
        ufree_quoted_cmd(&mut squoted);
    }
    return fret;
}
unsafe extern "C" fn flocal_xcmd_await_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
    mut zdata: *const libc::c_char,
    mut cdata: size_t,
) -> boolean {
    (*qtrans).precfn = None;
    if *zdata.offset(0 as libc::c_int as isize) as libc::c_int != 'X' as i32
        || *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'Y' as i32
            && *zdata.offset(1 as libc::c_int as isize) as libc::c_int != 'N' as i32
    {
        ulog(
            LOG_ERROR,
            b"Bad response to work request\0" as *const u8 as *const libc::c_char,
        );
        utransfree(qtrans);
        return 0 as libc::c_int;
    }
    if *zdata.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
        ulog(
            LOG_ERROR,
            b"%s: work request denied\0" as *const u8 as *const libc::c_char,
            (*qtrans).s.zfrom,
        );
        fmail_transfer(
            0 as libc::c_int,
            (*qtrans).s.zuser,
            0 as *mut libc::c_void as *const libc::c_char,
            b"work request denied\0" as *const u8 as *const libc::c_char,
            (*qtrans).s.zfrom,
            (*(*qdaemon).qsys).uuconf_zname,
            (*qtrans).s.zto,
            0 as *mut libc::c_void as *const libc::c_char,
            0 as *mut libc::c_void as *const libc::c_char,
        );
    }
    fsysdep_did_work((*qtrans).s.pseq);
    utransfree(qtrans);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fremote_xcmd_init(
    mut qdaemon: *mut sdaemon,
    mut qcmd: *mut scmd,
    mut iremote: libc::c_int,
) -> boolean {
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut zexclam: *const libc::c_char = 0 as *const libc::c_char;
    let mut qdestsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut sdestsys: uuconf_system = uuconf_system {
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
    let mut zdestfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmkdirs: boolean = 0;
    let mut qtrans: *mut stransfer = 0 as *mut stransfer;
    let mut zuser: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aboptions: [libc::c_char; 5] = [0; 5];
    let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    ulog(
        LOG_NORMAL,
        b"Work requested: %s to %s\0" as *const u8 as *const libc::c_char,
        (*qcmd).zfrom,
        (*qcmd).zto,
    );
    qsys = (*qdaemon).qsys;
    zexclam = strchr((*qcmd).zto, '!' as i32);
    if zexclam.is_null() || zexclam == (*qcmd).zto
        || strncmp(
            (*qdaemon).zlocalname,
            (*qcmd).zto,
            zexclam.offset_from((*qcmd).zto) as libc::c_long as size_t,
        ) == 0 as libc::c_int
    {
        let mut zconst: *const libc::c_char = 0 as *const libc::c_char;
        qdestsys = 0 as *const uuconf_system;
        if zexclam.is_null() {
            zconst = (*qcmd).zto;
        } else {
            zconst = zexclam.offset(1 as libc::c_int as isize);
        }
        zdestfile = zsysdep_local_file(
            zconst,
            (*qsys).uuconf_zpubdir,
            0 as *mut libc::c_void as *mut boolean,
        );
        if zdestfile.is_null() {
            return 0 as libc::c_int;
        }
        zuser = 0 as *mut libc::c_char;
        fmkdirs = (strchr((*qcmd).zoptions, 'f' as i32)
            != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    } else {
        let mut clen: size_t = 0;
        let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut iuuconf: libc::c_int = 0;
        let mut zoptions: *mut libc::c_char = 0 as *mut libc::c_char;
        clen = zexclam.offset_from((*qcmd).zto) as libc::c_long as size_t;
        zcopy = zbufalc(clen.wrapping_add(1 as libc::c_int as libc::c_ulong));
        memcpy(zcopy as *mut libc::c_void, (*qcmd).zto as *const libc::c_void, clen);
        *zcopy.offset(clen as isize) = '\0' as i32 as libc::c_char;
        iuuconf = uuconf_system_info((*qdaemon).puuconf, zcopy, &mut sdestsys);
        if iuuconf == 1 as libc::c_int {
            if funknown_system((*qdaemon).puuconf, zcopy, &mut sdestsys) == 0 {
                ulog(
                    LOG_ERROR,
                    b"%s: System not found\0" as *const u8 as *const libc::c_char,
                    zcopy,
                );
                ubuffree(zcopy);
                qtrans = qtransalc(qcmd);
                (*qtrans)
                    .psendfn = Some(
                    fremote_xcmd_reply
                        as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
                );
                (*qtrans).pinfo = b"XN\0" as *const u8 as *const libc::c_char as pointer;
                (*qtrans).iremote = iremote;
                return fqueue_remote(qdaemon, qtrans);
            }
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, (*qdaemon).puuconf, iuuconf);
            ubuffree(zcopy);
            return 0 as libc::c_int;
        }
        ubuffree(zcopy);
        qdestsys = &mut sdestsys;
        zdestfile = zbufcpy(zexclam.offset(1 as libc::c_int as isize));
        zuser = zbufalc(
            (strlen((*qdestsys).uuconf_zname))
                .wrapping_add(strlen((*qcmd).zuser))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                ),
        );
        sprintf(
            zuser,
            b"%s!%s\0" as *const u8 as *const libc::c_char,
            (*qdestsys).uuconf_zname,
            (*qcmd).zuser,
        );
        zoptions = aboptions.as_mut_ptr();
        let fresh0 = zoptions;
        zoptions = zoptions.offset(1);
        *fresh0 = 'C' as i32 as libc::c_char;
        if !(strchr((*qcmd).zoptions, 'd' as i32)).is_null() {
            let fresh1 = zoptions;
            zoptions = zoptions.offset(1);
            *fresh1 = 'd' as i32 as libc::c_char;
        }
        if !(strchr((*qcmd).zoptions, 'm' as i32)).is_null() {
            let fresh2 = zoptions;
            zoptions = zoptions.offset(1);
            *fresh2 = 'm' as i32 as libc::c_char;
        }
        *zoptions = '\0' as i32 as libc::c_char;
        fmkdirs = 1 as libc::c_int;
    }
    qtrans = qtransalc(qcmd);
    (*qtrans)
        .psendfn = Some(
        fremote_xcmd_reply
            as unsafe extern "C" fn(*mut stransfer, *mut sdaemon) -> boolean,
    );
    (*qtrans).pinfo = b"XY\0" as *const u8 as *const libc::c_char as pointer;
    (*qtrans).iremote = iremote;
    if fqueue_remote(qdaemon, qtrans) == 0 {
        ubuffree(zdestfile);
        ubuffree(zuser);
        return 0 as libc::c_int;
    }
    zfrom = zsysdep_local_file(
        (*qcmd).zfrom,
        (*qsys).uuconf_zpubdir,
        0 as *mut libc::c_void as *mut boolean,
    );
    if zfrom.is_null() {
        ubuffree(zdestfile);
        ubuffree(zuser);
        return 0 as libc::c_int;
    }
    if fsysdep_wildcard_start(zfrom) == 0 {
        ubuffree(zfrom);
        ubuffree(zdestfile);
        ubuffree(zuser);
        return 0 as libc::c_int;
    }
    fret = 1 as libc::c_int;
    loop {
        zfile = zsysdep_wildcard(zfrom);
        if zfile.is_null() {
            break;
        }
        let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut abtname: [libc::c_char; 15] = [0; 15];
        if fsysdep_file_exists(zfile) == 0 {
            ulog(
                LOG_ERROR,
                b"%s: no such file\0" as *const u8 as *const libc::c_char,
                zfile,
            );
        } else if fin_directory_list(
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
            break;
        } else {
            if !qdestsys.is_null() {
                zto = zsysdep_data_file_name(
                    qdestsys,
                    (*qdaemon).zlocalname,
                    'N' as i32,
                    0 as libc::c_int,
                    abtname.as_mut_ptr(),
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                if zto.is_null() {
                    fret = 0 as libc::c_int;
                    break;
                }
            } else {
                zto = zsysdep_add_base(zdestfile, zfile);
                if zto.is_null() {
                    fret = 0 as libc::c_int;
                    break;
                } else if fin_directory_list(
                    zto,
                    (*qsys).uuconf_pzremote_receive,
                    (*qsys).uuconf_zpubdir,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void as *const libc::c_char,
                ) == 0
                {
                    ulog(
                        LOG_ERROR,
                        b"%s: not permitted to receive\0" as *const u8
                            as *const libc::c_char,
                        zto,
                    );
                    ubuffree(zto);
                    break;
                }
            }
            if fcopy_file(
                zfile,
                zto,
                (qdestsys == 0 as *mut libc::c_void as *const uuconf_system)
                    as libc::c_int,
                fmkdirs,
                0 as libc::c_int,
            ) == 0
            {
                ubuffree(zto);
                break;
            } else {
                ubuffree(zto);
                if !qdestsys.is_null() {
                    let mut ssend: scmd = scmd {
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
                    let mut zjobid: *mut libc::c_char = 0 as *mut libc::c_char;
                    ssend.bcmd = 'S' as i32 as libc::c_char;
                    ssend.bgrade = 'N' as i32 as libc::c_char;
                    ssend.pseq = 0 as *mut libc::c_void;
                    ssend.zfrom = zfile;
                    ssend.zto = zdestfile;
                    ssend.zuser = zuser;
                    ssend.zoptions = aboptions.as_mut_ptr();
                    ssend.ztemp = abtname.as_mut_ptr();
                    ssend.imode = ixsysdep_file_mode(zfile);
                    ssend.znotify = b"\0" as *const u8 as *const libc::c_char;
                    ssend.cbytes = -(1 as libc::c_int) as libc::c_long;
                    ssend.zcmd = 0 as *const libc::c_char;
                    ssend.ipos = 0 as libc::c_int as libc::c_long;
                    zjobid = zsysdep_spool_commands(
                        qdestsys,
                        'N' as i32,
                        1 as libc::c_int,
                        &mut ssend,
                        0 as *mut libc::c_void as *mut boolean,
                    );
                    if zjobid.is_null() {
                        break;
                    }
                    ubuffree(zjobid);
                }
                ubuffree(zfile);
            }
        }
    }
    if !zfile.is_null() {
        ubuffree(zfile);
    }
    fsysdep_wildcard_end();
    ubuffree(zdestfile);
    if !qdestsys.is_null() {
        uuconf_free_block(sdestsys.uuconf_palloc);
    }
    ubuffree(zfrom);
    ubuffree(zuser);
    return fret;
}
unsafe extern "C" fn fremote_xcmd_reply(
    mut qtrans: *mut stransfer,
    mut qdaemon: *mut sdaemon,
) -> boolean {
    let mut fret: boolean = 0;
    fret = (Some(((*(*qdaemon).qproto).pfsendcmd).unwrap()))
        .unwrap()(
        qdaemon,
        (*qtrans).pinfo as *const libc::c_char,
        (*qtrans).ilocal,
        (*qtrans).iremote,
    );
    utransfree(qtrans);
    return fret;
}
