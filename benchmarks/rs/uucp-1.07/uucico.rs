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
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut gnu_optarg: *mut libc::c_char;
    static mut gnu_optind: libc::c_int;
    static mut gnu_opterr: libc::c_int;
    fn gnu_getopt(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut azStatus: [*const libc::c_char; 0];
    static mut iDebug: libc::c_int;
    fn funknown_system(
        puuconf: pointer,
        zsystem: *const libc::c_char,
        qsys: *mut uuconf_system,
    ) -> boolean;
    fn ftimespan_match(
        qspan: *const uuconf_timespan,
        pival: *mut libc::c_long,
        pcretry: *mut libc::c_int,
    ) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_fatal_fn(pfn: Option::<unsafe extern "C" fn() -> ()>);
    fn ulog_to_file(puuconf: pointer, ffile: boolean);
    fn ulog_system(zsystem: *const libc::c_char);
    fn ulog_user(zuser: *const libc::c_char);
    fn ulog_close();
    fn ustats_close();
    fn cdebug_char(z: *mut libc::c_char, ichar: libc::c_int) -> size_t;
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn cescape(zbuf: *mut libc::c_char) -> size_t;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    static mut zProgram: *const libc::c_char;
    static mut afSignal: [sig_atomic_t; 5];
    static mut zLdevice: *mut libc::c_char;
    fn uuconf_init(
        uuconf_ppglobal: *mut *mut libc::c_void,
        uuconf_zprogram: *const libc::c_char,
        uuconf_zname: *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_system_names(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_ppzsystems: *mut *mut *mut libc::c_char,
        uuconf_falias: libc::c_int,
    ) -> libc::c_int;
    fn uuconf_system_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zsystem: *const libc::c_char,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_find_port(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zname: *const libc::c_char,
        uuconf_ibaud: libc::c_long,
        uuconf_ihighbaud: libc::c_long,
        uuconf_pifn: Option::<
            unsafe extern "C" fn(*mut uuconf_port, *mut libc::c_void) -> libc::c_int,
        >,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_qport: *mut uuconf_port,
    ) -> libc::c_int;
    fn uuconf_dialer_info(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zdialer: *const libc::c_char,
        uuconf_qdialer: *mut uuconf_dialer,
    ) -> libc::c_int;
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_login_localname(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_zlogin: *const libc::c_char,
        pzname: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debuglevel(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_strip(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pistrip: *mut libc::c_int,
    ) -> libc::c_int;
    fn uuconf_runuuxqt(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pirunuuxqt: *mut libc::c_int,
    ) -> libc::c_int;
    fn uuconf_callin(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_cmp: Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *mut libc::c_void,
                *const libc::c_char,
            ) -> libc::c_int,
        >,
        uuconf_pinfo: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_validate(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *const uuconf_system,
        uuconf_zlogin: *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_remote_unknown(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn uuconf_cmd_args(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_cargs: libc::c_int,
        uuconf_pzargs: *mut *mut libc::c_char,
        uuconf_qtab: *const uuconf_cmdtab,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_pfiunknownfn: uuconf_cmdtabfn,
        uuconf_iflags: libc::c_int,
        pblock: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn fconn_init(
        qport: *mut uuconf_port,
        qconn: *mut sconnection,
        ttype: uuconf_porttype,
    ) -> boolean;
    fn uconn_free(qconn: *mut sconnection);
    fn fconn_lock(qconn: *mut sconnection, fin: boolean, fuser: boolean) -> boolean;
    fn fconn_unlock(qconn: *mut sconnection) -> boolean;
    fn fconn_open(
        qconn: *mut sconnection,
        ibaud: libc::c_long,
        ihighbaud: libc::c_long,
        fwait: boolean,
        fuser: boolean,
    ) -> boolean;
    fn fconn_close(
        qconn: *mut sconnection,
        puuconf: pointer,
        qdialer: *mut uuconf_dialer,
        fsuccess: boolean,
    ) -> boolean;
    fn fconn_dial(
        q: *mut sconnection,
        puuconf: pointer,
        qsys: *const uuconf_system,
        zphone: *const libc::c_char,
        qdialer: *mut uuconf_dialer,
        ptdialerfound: *mut tdialerfound,
    ) -> boolean;
    fn fconn_write(
        qconn: *mut sconnection,
        zbuf: *const libc::c_char,
        cbytes: size_t,
    ) -> boolean;
    fn iconn_baud(qconn: *mut sconnection) -> libc::c_long;
    fn fchat(
        qconn: *mut sconnection,
        puuconf: pointer,
        qchat: *const uuconf_chat,
        qsys: *const uuconf_system,
        qdialer: *const uuconf_dialer,
        zphone: *const libc::c_char,
        ftranslate: boolean,
        zport: *const libc::c_char,
        ibaud: libc::c_long,
    ) -> boolean;
    fn breceive_char(
        qconn: *mut sconnection,
        ctimeout: libc::c_int,
        freport: boolean,
    ) -> libc::c_int;
    static mut asGproto_params: [uuconf_cmdtab; 0];
    fn fgstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fbiggstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fvstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fgshutdown(qdaemon: *mut sdaemon) -> boolean;
    fn fgsendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn zggetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn fgsenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn fgwait(qdaemon: *mut sdaemon) -> boolean;
    static mut asFproto_params: [uuconf_cmdtab; 0];
    fn ffstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn ffshutdown(qdaemon: *mut sdaemon) -> boolean;
    fn ffsendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn zfgetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn ffsenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn ffwait(qdaemon: *mut sdaemon) -> boolean;
    fn fffile(
        qdaemon: *mut sdaemon,
        qtrans: *mut stransfer,
        fstart: boolean,
        fsend: boolean,
        cbytes: libc::c_long,
        pfhandled: *mut boolean,
    ) -> boolean;
    static mut asTproto_params: [uuconf_cmdtab; 0];
    fn ftstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn ftshutdown(qdaemon: *mut sdaemon) -> boolean;
    fn ftsendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn ztgetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn ftsenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn ftwait(qdaemon: *mut sdaemon) -> boolean;
    fn ftfile(
        qdaemon: *mut sdaemon,
        qtrans: *mut stransfer,
        fstart: boolean,
        fsend: boolean,
        cbytes: libc::c_long,
        pfhandled: *mut boolean,
    ) -> boolean;
    static mut asEproto_params: [uuconf_cmdtab; 0];
    fn festart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn feshutdown(qdaemon: *mut sdaemon) -> boolean;
    fn fesendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn zegetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn fesenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn fewait(qdaemon: *mut sdaemon) -> boolean;
    fn fefile(
        qdaemon: *mut sdaemon,
        qtrans: *mut stransfer,
        fstart: boolean,
        fsend: boolean,
        cbytes: libc::c_long,
        pfhandled: *mut boolean,
    ) -> boolean;
    static mut asIproto_params: [uuconf_cmdtab; 0];
    fn fistart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fishutdown(qdaemon: *mut sdaemon) -> boolean;
    fn fisendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn zigetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn fisenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn fiwait(qdaemon: *mut sdaemon) -> boolean;
    fn fjstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fjshutdown(qdaemon: *mut sdaemon) -> boolean;
    static mut asZproto_params: [uuconf_cmdtab; 0];
    fn fzstart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fzshutdown(qdaemon: *mut sdaemon) -> boolean;
    fn fzsendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn zzgetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn fzsenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn fzwait(qdaemon: *mut sdaemon) -> boolean;
    fn fzfile(
        qdaemon: *mut sdaemon,
        qtrans: *mut stransfer,
        fstart: boolean,
        fsend: boolean,
        cbytes: libc::c_long,
        pfhandled: *mut boolean,
    ) -> boolean;
    static mut asYproto_params: [uuconf_cmdtab; 0];
    fn fystart(qdaemon: *mut sdaemon, pzlog: *mut *mut libc::c_char) -> boolean;
    fn fyshutdown(qdaemon: *mut sdaemon) -> boolean;
    fn fysendcmd(
        qdaemon: *mut sdaemon,
        z: *const libc::c_char,
        ilocal: libc::c_int,
        iremote: libc::c_int,
    ) -> boolean;
    fn zygetspace(qdaemon: *mut sdaemon, pcdata: *mut size_t) -> *mut libc::c_char;
    fn fysenddata(
        qdaemon: *mut sdaemon,
        z: *mut libc::c_char,
        c: size_t,
        ilocal: libc::c_int,
        iremote: libc::c_int,
        ipos: libc::c_long,
    ) -> boolean;
    fn fywait(qdaemon: *mut sdaemon) -> boolean;
    fn fyfile(
        qdaemon: *mut sdaemon,
        qtrans: *mut stransfer,
        fstart: boolean,
        fsend: boolean,
        cbytes: libc::c_long,
        pfhandled: *mut boolean,
    ) -> boolean;
    fn floop(qdaemon: *mut sdaemon) -> boolean;
    fn fqueue(qdaemon: *mut sdaemon, pfany: *mut boolean) -> boolean;
    fn uclear_queue(qdaemon: *mut sdaemon);
    fn ufailed(qdaemon: *mut sdaemon);
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn usysdep_detach();
    fn zsysdep_localname() -> *const libc::c_char;
    fn zsysdep_login_name() -> *const libc::c_char;
    fn usysdep_signal(isig: libc::c_int);
    fn zsysdep_port_name(pftcp_port: *mut boolean) -> *const libc::c_char;
    fn fsysdep_run(
        ffork: boolean,
        zprogram: *const libc::c_char,
        zarg1: *const libc::c_char,
        zarg2: *const libc::c_char,
    ) -> boolean;
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn fsysdep_lock_system(qsys: *const uuconf_system) -> boolean;
    fn fsysdep_unlock_system(qsys: *const uuconf_system) -> boolean;
    fn ixsysdep_get_sequence(qsys: *const uuconf_system) -> libc::c_long;
    fn fsysdep_get_status(
        qsys: *const uuconf_system,
        qret: *mut sstatus,
        pfnone: *mut boolean,
    ) -> boolean;
    fn fsysdep_set_status(qsys: *const uuconf_system, qset: *const sstatus) -> boolean;
    fn fsysdep_unknown_caller(
        zscript: *const libc::c_char,
        zsystem: *const libc::c_char,
    ) -> boolean;
    fn fsysdep_has_work(qsys: *const uuconf_system) -> boolean;
    fn zsysdep_spool_commands(
        qsys: *const uuconf_system,
        bgrade: libc::c_int,
        ccmds: libc::c_int,
        pascmds: *const scmd,
        pftemp: *mut boolean,
    ) -> *mut libc::c_char;
    fn fsysdep_privileged() -> boolean;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type openfile_t = *mut FILE;
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
pub struct sconnection {
    pub qcmds: *const sconncmds,
    pub psysdep: pointer,
    pub qport: *mut uuconf_port,
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
pub type UUCONF_POINTER = *mut libc::c_void;
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
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_cmdtab {
    pub uuconf_zcmd: *const libc::c_char,
    pub uuconf_itype: libc::c_int,
    pub uuconf_pvar: UUCONF_POINTER,
    pub uuconf_pifn: uuconf_cmdtabfn,
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
pub struct sstatus {
    pub ttype: tstatus_type,
    pub cretries: libc::c_int,
    pub ilast: libc::c_long,
    pub cwait: libc::c_int,
    pub zstring: *mut libc::c_char,
}
pub type tstatus_type = libc::c_uint;
pub const STATUS_VALUES: tstatus_type = 8;
pub const STATUS_WRONG_TIME: tstatus_type = 7;
pub const STATUS_TALKING: tstatus_type = 6;
pub const STATUS_FAILED: tstatus_type = 5;
pub const STATUS_HANDSHAKE_FAILED: tstatus_type = 4;
pub const STATUS_LOGIN_FAILED: tstatus_type = 3;
pub const STATUS_DIAL_FAILED: tstatus_type = 2;
pub const STATUS_PORT_FAILED: tstatus_type = 1;
pub const STATUS_COMPLETE: tstatus_type = 0;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISspace: C2RustUnnamed_0 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scallin_info {
    pub zuser: *const libc::c_char,
    pub zpass: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spass {
    pub fmatched: boolean,
    pub flocked: boolean,
    pub qconn: *mut sconnection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const no_argument: _argtype = 0;
pub const required_argument: _argtype = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type _argtype = libc::c_uint;
pub const optional_argument: _argtype = 2;
pub static mut uucico_rcsid: [libc::c_char; 52] = unsafe {
    *::std::mem::transmute::<
        &[u8; 52],
        &[libc::c_char; 52],
    >(b"$Id: uucico.c,v 1.204 2003/05/29 06:00:49 ian Rel $\0")
};
static mut asProtocols: [sprotocol; 10] = unsafe {
    [
        {
            let mut init = sprotocol {
                bname: 't' as i32 as libc::c_char,
                ireliable: 0o10 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asTproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    ftstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    ftshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    ftsendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    ztgetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    ftsenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(ftwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: Some(
                    ftfile
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut stransfer,
                            boolean,
                            boolean,
                            libc::c_long,
                            *mut boolean,
                        ) -> boolean,
                ),
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'e' as i32 as libc::c_char,
                ireliable: 0o10 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asEproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    festart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    feshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fesendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zegetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fesenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fewait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: Some(
                    fefile
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut stransfer,
                            boolean,
                            boolean,
                            libc::c_long,
                            *mut boolean,
                        ) -> boolean,
                ),
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'i' as i32 as libc::c_char,
                ireliable: 0o2 as libc::c_int,
                cchans: 7 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asIproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fistart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fishutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fisendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zigetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fisenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fiwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: None,
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'a' as i32 as libc::c_char,
                ireliable: 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asZproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fzstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fzshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fzsendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zzgetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fzsenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fzwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: Some(
                    fzfile
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut stransfer,
                            boolean,
                            boolean,
                            libc::c_long,
                            *mut boolean,
                        ) -> boolean,
                ),
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'g' as i32 as libc::c_char,
                ireliable: 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asGproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fgstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fgshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fgsendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zggetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fgsenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fgwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: None,
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'G' as i32 as libc::c_char,
                ireliable: 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asGproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fbiggstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fgshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fgsendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zggetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fgsenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fgwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: None,
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'j' as i32 as libc::c_char,
                ireliable: 0o2 as libc::c_int,
                cchans: 7 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asIproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fjstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fjshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fisendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zigetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fisenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fiwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: None,
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'f' as i32 as libc::c_char,
                ireliable: 0o4 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 0 as libc::c_int,
                qcmds: asFproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    ffstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    ffshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    ffsendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zfgetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    ffsenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(ffwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: Some(
                    fffile
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut stransfer,
                            boolean,
                            boolean,
                            libc::c_long,
                            *mut boolean,
                        ) -> boolean,
                ),
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'v' as i32 as libc::c_char,
                ireliable: 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asGproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fvstart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fgshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fgsendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zggetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fgsenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fgwait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: None,
            };
            init
        },
        {
            let mut init = sprotocol {
                bname: 'y' as i32 as libc::c_char,
                ireliable: 0o4 as libc::c_int | 0o2 as libc::c_int,
                cchans: 1 as libc::c_int,
                frestart: 1 as libc::c_int,
                qcmds: asYproto_params.as_ptr() as *mut _,
                pfstart: Some(
                    fystart
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut *mut libc::c_char,
                        ) -> boolean,
                ),
                pfshutdown: Some(
                    fyshutdown as unsafe extern "C" fn(*mut sdaemon) -> boolean,
                ),
                pfsendcmd: Some(
                    fysendcmd
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *const libc::c_char,
                            libc::c_int,
                            libc::c_int,
                        ) -> boolean,
                ),
                pzgetspace: Some(
                    zygetspace
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut size_t,
                        ) -> *mut libc::c_char,
                ),
                pfsenddata: Some(
                    fysenddata
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut libc::c_char,
                            size_t,
                            libc::c_int,
                            libc::c_int,
                            libc::c_long,
                        ) -> boolean,
                ),
                pfwait: Some(fywait as unsafe extern "C" fn(*mut sdaemon) -> boolean),
                pffile: Some(
                    fyfile
                        as unsafe extern "C" fn(
                            *mut sdaemon,
                            *mut stransfer,
                            boolean,
                            boolean,
                            libc::c_long,
                            *mut boolean,
                        ) -> boolean,
                ),
            };
            init
        },
    ]
};
static mut fLocked_system: boolean = 0;
static mut sLocked_system: uuconf_system = uuconf_system {
    uuconf_zname: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_pzalias: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_qalternate: 0 as *const uuconf_system as *mut uuconf_system,
    uuconf_zalternate: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_fcall: 0,
    uuconf_fcalled: 0,
    uuconf_qtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalltimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalledtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_cmax_retries: 0,
    uuconf_csuccess_wait: 0,
    uuconf_qcall_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcall_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalled_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_qcalled_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
    uuconf_ibaud: 0,
    uuconf_ihighbaud: 0,
    uuconf_zport: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_qport: 0 as *const uuconf_port as *mut uuconf_port,
    uuconf_zphone: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_schat: uuconf_chat {
        uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_ctimeout: 0,
        uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_fstrip: 0,
    },
    uuconf_zcall_login: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_zcall_password: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_zcalled_login: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_fcallback: 0,
    uuconf_fsequence: 0,
    uuconf_zprotocols: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_qproto_params: 0 as *const uuconf_proto_param as *mut uuconf_proto_param,
    uuconf_scalled_chat: uuconf_chat {
        uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_ctimeout: 0,
        uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_fstrip: 0,
    },
    uuconf_zdebug: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_zmax_remote_debug: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_fsend_request: 0,
    uuconf_frec_request: 0,
    uuconf_fcall_transfer: 0,
    uuconf_fcalled_transfer: 0,
    uuconf_pzlocal_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzremote_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzlocal_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzremote_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzpath: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzcmds: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_cfree_space: 0,
    uuconf_pzforward_from: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_pzforward_to: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    uuconf_zpubdir: 0 as *const libc::c_char,
    uuconf_zlocalname: 0 as *const libc::c_char as *mut libc::c_char,
    uuconf_cmax_file_time: 0,
    uuconf_palloc: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut sDaemon: sdaemon = sdaemon {
    puuconf: 0 as *const libc::c_void as *mut libc::c_void,
    zconfig: 0 as *const libc::c_char,
    irunuuxqt: 0,
    qsys: 0 as *const uuconf_system,
    zlocalname: 0 as *const libc::c_char,
    qconn: 0 as *const sconnection as *mut sconnection,
    qproto: 0 as *const sprotocol,
    cchans: 0,
    clocal_size: 0,
    cremote_size: 0,
    cmax_ever: 0,
    cmax_receive: 0,
    csent: 0,
    creceived: 0,
    cxfiles_received: 0,
    ifeatures: 0,
    frequest_hangup: 0,
    fhangup_requested: 0,
    fhangup: 0,
    fmaster: 0,
    fcaller: 0,
    ireliable: 0,
    bgrade: 0,
};
static mut qConn: *mut sconnection = 0 as *const sconnection as *mut sconnection;
static mut pUuconf: pointer = 0 as *const libc::c_void as *mut libc::c_void;
static mut asLongopts: [option; 20] = [
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ifwork\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nodetach\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"loop\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdin\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"port\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nouuxqt\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"master\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slave\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"system\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"login\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wait\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"try-next\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"config\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut fquiet: boolean = 0 as libc::c_int;
    let mut fifwork: boolean = 0 as libc::c_int;
    let mut fdetach: boolean = 1 as libc::c_int;
    let mut fendless: boolean = 0 as libc::c_int;
    let mut fforce: boolean = 0 as libc::c_int;
    let mut tstdintype: uuconf_porttype = UUCONF_PORTTYPE_STDIN;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut flogin: boolean = 0 as libc::c_int;
    let mut zport: *const libc::c_char = 0 as *const libc::c_char;
    let mut fuuxqt: boolean = 1 as libc::c_int;
    let mut fmaster: boolean = 0 as libc::c_int;
    let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut zlogin: *const libc::c_char = 0 as *const libc::c_char;
    let mut fwait: boolean = 0 as libc::c_int;
    let mut ftrynext: boolean = 0 as libc::c_int;
    let mut zopts: *const libc::c_char = 0 as *const libc::c_char;
    let mut iopt: libc::c_int = 0;
    let mut qport: *mut uuconf_port = 0 as *mut uuconf_port;
    let mut sport: uuconf_port = uuconf_port {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_ttype: UUCONF_PORTTYPE_UNKNOWN,
        uuconf_zprotocols: 0 as *mut libc::c_char,
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_ireliable: 0,
        uuconf_zlockname: 0 as *mut libc::c_char,
        uuconf_palloc: 0 as *mut libc::c_void,
        uuconf_u: C2RustUnnamed {
            uuconf_sstdin: uuconf_stdin_port {
                uuconf_idummy: 0,
            },
        },
    };
    let mut fret: boolean = 1 as libc::c_int;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut iholddebug: libc::c_int = 0;
    zProgram = *argv.offset(0 as libc::c_int as isize);
    if *zProgram as libc::c_int == '-' as i32 {
        zProgram = zProgram.offset(1);
        zProgram;
    }
    zopts = b"cCDefi:I:lp:qr:s:S:u:x:X:vwz\0" as *const u8 as *const libc::c_char;
    loop {
        iopt = getopt_long(
            argc,
            argv,
            zopts,
            asLongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            2 | 99 => {
                fquiet = 1 as libc::c_int;
            }
            67 => {
                fifwork = 1 as libc::c_int;
            }
            68 => {
                fdetach = 0 as libc::c_int;
            }
            101 => {
                fendless = 1 as libc::c_int;
            }
            102 => {
                fforce = 1 as libc::c_int;
            }
            105 => {
                if strcasecmp(gnu_optarg, b"tli\0" as *const u8 as *const libc::c_char)
                    != 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"%s: unsupported port type \"%s\"\n\0" as *const u8
                            as *const libc::c_char,
                        zProgram,
                        gnu_optarg,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%s: not compiled with TLI support\n\0" as *const u8
                            as *const libc::c_char,
                        zProgram,
                    );
                }
            }
            108 => {
                flogin = 1 as libc::c_int;
            }
            112 => {
                zport = gnu_optarg;
            }
            113 => {
                fuuxqt = 0 as libc::c_int;
            }
            114 => {
                if strcmp(gnu_optarg, b"1\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    fmaster = 1 as libc::c_int;
                } else if strcmp(gnu_optarg, b"0\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    fmaster = 0 as libc::c_int;
                } else {
                    uusage();
                }
            }
            115 => {
                zsystem = gnu_optarg;
                fmaster = 1 as libc::c_int;
            }
            83 => {
                zsystem = gnu_optarg;
                fforce = 1 as libc::c_int;
                fmaster = 1 as libc::c_int;
            }
            117 => {
                if fsysdep_privileged() != 0 {
                    zlogin = gnu_optarg;
                } else {
                    fprintf(
                        stderr,
                        b"%s: ignoring command line login name: not a privileged user\n\0"
                            as *const u8 as *const libc::c_char,
                        zProgram,
                    );
                }
            }
            119 => {
                fwait = 1 as libc::c_int;
            }
            122 => {
                ftrynext = 1 as libc::c_int;
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            120 | 88 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            118 => {
                printf(
                    b"uucico (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
                    b"1.07\0" as *const u8 as *const libc::c_char,
                );
                printf(
                    b"Copyright (C) 1991, 92, 93, 94, 1995, 2002, 2003 Ian Lance Taylor\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"This program is free software; you may redistribute it under the terms of\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"the GNU General Public LIcense.  This program has ABSOLUTELY NO WARRANTY.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            4 => {
                fmaster = 0 as libc::c_int;
            }
            3 => {
                fmaster = 1 as libc::c_int;
            }
            1 => {
                uhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                uusage();
            }
        }
    }
    if gnu_optind != argc {
        uusage();
    }
    if fwait != 0 && zport.is_null() {
        fprintf(
            stderr,
            b"%s: -w requires -p\0" as *const u8 as *const libc::c_char,
            zProgram,
        );
        uusage();
    }
    iuuconf = uuconf_init(
        &mut puuconf,
        0 as *mut libc::c_void as *const libc::c_char,
        zconfig,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    pUuconf = puuconf;
    let mut zdebug: *const libc::c_char = 0 as *const libc::c_char;
    iuuconf = uuconf_debuglevel(puuconf, &mut zdebug);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    if !zdebug.is_null() {
        iDebug |= idebug_parse(zdebug);
    }
    if zport.is_null() {
        qport = 0 as *mut uuconf_port;
    } else {
        iuuconf = uuconf_find_port(
            puuconf,
            zport,
            0 as libc::c_int as libc::c_long,
            0 as libc::c_int as libc::c_long,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int>,
            >(0 as *mut libc::c_void),
            0 as *mut libc::c_void,
            &mut sport,
        );
        if iuuconf == 1 as libc::c_int {
            ulog(
                LOG_FATAL,
                b"%s: port not found\0" as *const u8 as *const libc::c_char,
                zport,
            );
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
        }
        qport = &mut sport;
    }
    usysdep_signal(2 as libc::c_int);
    usysdep_signal(1 as libc::c_int);
    usysdep_signal(3 as libc::c_int);
    usysdep_signal(15 as libc::c_int);
    usysdep_signal(13 as libc::c_int);
    usysdep_initialize(puuconf, 0o4 as libc::c_int);
    ulog_to_file(puuconf, 1 as libc::c_int);
    ulog_fatal_fn(Some(uabort as unsafe extern "C" fn() -> ()));
    if fmaster != 0 {
        if !zsystem.is_null() {
            iuuconf = uuconf_system_info(puuconf, zsystem, &mut sLocked_system);
            if iuuconf == 1 as libc::c_int {
                ulog(
                    LOG_FATAL,
                    b"%s: System not found\0" as *const u8 as *const libc::c_char,
                    zsystem,
                );
            } else if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            if fdetach != 0
                && (qport.is_null()
                    || (*qport).uuconf_ttype as libc::c_uint
                        != UUCONF_PORTTYPE_STDIN as libc::c_int as libc::c_uint)
            {
                usysdep_detach();
            }
            ulog_system(sLocked_system.uuconf_zname);
            iholddebug = iDebug;
            if !(sLocked_system.uuconf_zdebug).is_null() {
                iDebug |= idebug_parse(sLocked_system.uuconf_zdebug);
            }
            if fsysdep_lock_system(&mut sLocked_system) == 0 {
                ulog(
                    LOG_ERROR,
                    b"System already locked\0" as *const u8 as *const libc::c_char,
                );
                fret = 0 as libc::c_int;
            } else {
                fLocked_system = 1 as libc::c_int;
                fret = fcall(
                    puuconf,
                    zconfig,
                    fuuxqt,
                    &mut sLocked_system,
                    qport,
                    fifwork,
                    fforce,
                    fdetach,
                    fquiet,
                    ftrynext,
                );
                if fLocked_system != 0 {
                    fsysdep_unlock_system(&mut sLocked_system);
                    fLocked_system = 0 as libc::c_int;
                }
            }
            iDebug = iholddebug;
            ulog_system(0 as *mut libc::c_void as *const libc::c_char);
            uuconf_free_block(sLocked_system.uuconf_palloc);
        } else {
            let mut pznames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut c: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut fdidone: boolean = 0;
            fret = 1 as libc::c_int;
            fdidone = 0 as libc::c_int;
            iuuconf = uuconf_system_names(puuconf, &mut pznames, 0 as libc::c_int);
            if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            c = 0 as libc::c_int;
            pz = pznames;
            while !(*pz).is_null() {
                c += 1;
                c;
                pz = pz.offset(1);
                pz;
            }
            srand(
                ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
                    as libc::c_uint,
            );
            i = c - 1 as libc::c_int;
            while i > 0 as libc::c_int {
                let mut iuse: libc::c_int = 0;
                let mut zhold: *mut libc::c_char = 0 as *mut libc::c_char;
                iuse = rand() % (i + 1 as libc::c_int);
                zhold = *pznames.offset(i as isize);
                let ref mut fresh0 = *pznames.offset(i as isize);
                *fresh0 = *pznames.offset(iuse as isize);
                let ref mut fresh1 = *pznames.offset(iuse as isize);
                *fresh1 = zhold;
                i -= 1;
                i;
            }
            pz = pznames;
            while !(*pz).is_null()
                && !(afSignal[0 as libc::c_int as usize] != 0
                    || afSignal[1 as libc::c_int as usize] != 0
                    || afSignal[2 as libc::c_int as usize] != 0
                    || afSignal[3 as libc::c_int as usize] != 0
                    || afSignal[4 as libc::c_int as usize] != 0)
            {
                iuuconf = uuconf_system_info(puuconf, *pz, &mut sLocked_system);
                if iuuconf != 0 as libc::c_int {
                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                    xfree(*pz as pointer);
                } else {
                    if fsysdep_has_work(&mut sLocked_system) != 0 {
                        fdidone = 1 as libc::c_int;
                        if fdetach != 0
                            && (qport.is_null()
                                || (*qport).uuconf_ttype as libc::c_uint
                                    != UUCONF_PORTTYPE_STDIN as libc::c_int as libc::c_uint)
                        {
                            usysdep_detach();
                        }
                        ulog_system(sLocked_system.uuconf_zname);
                        iholddebug = iDebug;
                        if !(sLocked_system.uuconf_zdebug).is_null() {
                            iDebug |= idebug_parse(sLocked_system.uuconf_zdebug);
                        }
                        if fsysdep_lock_system(&mut sLocked_system) == 0 {
                            ulog(
                                LOG_ERROR,
                                b"System already locked\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fret = 0 as libc::c_int;
                        } else {
                            fLocked_system = 1 as libc::c_int;
                            if fcall(
                                puuconf,
                                zconfig,
                                fuuxqt,
                                &mut sLocked_system,
                                qport,
                                1 as libc::c_int,
                                fforce,
                                fdetach,
                                fquiet,
                                ftrynext,
                            ) == 0
                            {
                                fret = 0 as libc::c_int;
                            }
                            ::std::ptr::write_volatile(
                                &mut afSignal[0 as libc::c_int as usize]
                                    as *mut sig_atomic_t,
                                0 as libc::c_int,
                            );
                            if fLocked_system != 0 {
                                fsysdep_unlock_system(&mut sLocked_system);
                                fLocked_system = 0 as libc::c_int;
                            }
                        }
                        iDebug = iholddebug;
                        ulog_system(0 as *mut libc::c_void as *const libc::c_char);
                    }
                    uuconf_free_block(sLocked_system.uuconf_palloc);
                    xfree(*pz as pointer);
                }
                pz = pz.offset(1);
                pz;
            }
            xfree(pznames as pointer);
            if fdidone == 0 && fquiet == 0 {
                ulog(LOG_NORMAL, b"No work\0" as *const u8 as *const libc::c_char);
            }
        }
        if fwait != 0 {
            fendless = 1 as libc::c_int;
            fmaster = 0 as libc::c_int;
        }
    }
    if fmaster == 0 {
        let mut sconn: sconnection = sconnection {
            qcmds: 0 as *const sconncmds,
            psysdep: 0 as *mut libc::c_void,
            qport: 0 as *mut uuconf_port,
        };
        let mut flocked: boolean = 0;
        fret = 1 as libc::c_int;
        zsystem = 0 as *const libc::c_char;
        if fconn_init(qport, &mut sconn, tstdintype) == 0 {
            fret = 0 as libc::c_int;
        }
        if !qport.is_null() {
            if fdetach != 0
                && (*qport).uuconf_ttype as libc::c_uint
                    != UUCONF_PORTTYPE_STDIN as libc::c_int as libc::c_uint
            {
                usysdep_detach();
            }
        }
        if fconn_lock(&mut sconn, 1 as libc::c_int, 0 as libc::c_int) != 0 {
            flocked = 1 as libc::c_int;
        } else {
            flocked = 0 as libc::c_int;
            ulog(
                LOG_ERROR,
                b"%s: Port already locked\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zname,
            );
            fret = 0 as libc::c_int;
        }
        if fret != 0 {
            if fconn_open(
                &mut sconn,
                0 as libc::c_int as libc::c_long,
                0 as libc::c_int as libc::c_long,
                1 as libc::c_int,
                0 as libc::c_int,
            ) == 0
            {
                fret = 0 as libc::c_int;
            }
            qConn = &mut sconn;
        }
        if fret != 0 {
            if fendless != 0 {
                while !(afSignal[0 as libc::c_int as usize] != 0
                    || afSignal[1 as libc::c_int as usize] != 0
                    || afSignal[2 as libc::c_int as usize] != 0
                    || afSignal[3 as libc::c_int as usize] != 0
                    || afSignal[4 as libc::c_int as usize] != 0)
                    && flogin_prompt(
                        puuconf,
                        zconfig,
                        fuuxqt,
                        &mut sconn,
                        0 as *mut libc::c_void as *const libc::c_char,
                        0 as *mut libc::c_void as *mut *const libc::c_char,
                    ) != 0
                {
                    if fconn_close(
                        &mut sconn,
                        puuconf,
                        0 as *mut libc::c_void as *mut uuconf_dialer,
                        1 as libc::c_int,
                    ) == 0
                        || fconn_open(
                            &mut sconn,
                            0 as libc::c_int as libc::c_long,
                            0 as libc::c_int as libc::c_long,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        ) == 0
                    {
                        break;
                    }
                }
                fret = 0 as libc::c_int;
            } else if flogin != 0 {
                fret = flogin_prompt(
                    puuconf,
                    zconfig,
                    fuuxqt,
                    &mut sconn,
                    zlogin,
                    &mut zsystem,
                );
            } else {
                iholddebug = iDebug;
                if zlogin.is_null() {
                    zlogin = zsysdep_login_name();
                }
                fret = faccept_call(
                    puuconf,
                    zconfig,
                    fuuxqt,
                    zlogin,
                    &mut sconn,
                    &mut zsystem,
                );
                iDebug = iholddebug;
            }
        }
        if !qConn.is_null() {
            if fconn_close(
                &mut sconn,
                puuconf,
                0 as *mut libc::c_void as *mut uuconf_dialer,
                fret,
            ) == 0
            {
                fret = 0 as libc::c_int;
            }
            qConn = 0 as *mut sconnection;
        }
        if flocked != 0 {
            fconn_unlock(&mut sconn);
        }
        uconn_free(&mut sconn);
    }
    ulog_close();
    ustats_close();
    if afSignal[3 as libc::c_int as usize] != 0 {
        fuuxqt = 0 as libc::c_int;
    }
    if fuuxqt != 0 {
        let mut irunuuxqt: libc::c_int = 0;
        iuuconf = uuconf_runuuxqt(puuconf, &mut irunuuxqt);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        } else if irunuuxqt == -(1 as libc::c_int) {
            if fdetach != 0 {
                usysdep_detach();
            }
            if fspawn_uuxqt(0 as libc::c_int, zsystem, zconfig) == 0 {
                fret = 0 as libc::c_int;
            }
        }
    }
    usysdep_exit(fret);
    return 0 as libc::c_int;
}
unsafe extern "C" fn uusage() {
    fprintf(
        stderr,
        b"Usage: %s [options]\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    fprintf(
        stderr,
        b"Use %s --help for help\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn uhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Usage: %s [options]\n\0" as *const u8 as *const libc::c_char, zProgram);
    printf(
        b" -s,-S,--system system: Call system (-S implies -f)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -f,--force: Force call despite system status\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -r state: 1 for master, 0 for slave (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b" --master: Act as master\n\0" as *const u8 as *const libc::c_char);
    printf(b" --slave: Act as slave (default)\n\0" as *const u8 as *const libc::c_char);
    printf(b" -p,--port port: Specify port\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -l,--prompt: Prompt for login name and password\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -e,--loop: Endless loop of login prompts and daemon execution\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -w,--wait: After calling out, wait for incoming calls\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -q,--nouuxqt: Don't start uuxqt when done\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -c,--quiet: Don't log bad time or no work warnings\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -C,--ifwork: Only call named system if there is work\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -D,--nodetach: Don't detach from controlling terminal\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -u,--login: Set login name (privileged users only)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -i,--stdin type: Type of standard input (only TLI supported)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -z,--try-next: If a call fails, try the next alternate\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -x,-X,--debug debug: Set debugging level\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -I,--config file: Set configuration file to use\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -v,--version: Print version and exit\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b" --help: Print help and exit\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Report bugs to taylor-uucp@gnu.org\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn uabort() {
    if fLocked_system != 0 {
        ufailed(&mut sDaemon);
    }
    ulog_user(0 as *mut libc::c_void as *const libc::c_char);
    if !qConn.is_null() {
        fconn_close(
            qConn,
            pUuconf,
            0 as *mut libc::c_void as *mut uuconf_dialer,
            0 as libc::c_int,
        );
        fconn_unlock(qConn);
        uconn_free(qConn);
    }
    if fLocked_system != 0 {
        fsysdep_unlock_system(&mut sLocked_system);
        fLocked_system = 0 as libc::c_int;
    }
    ulog_system(0 as *mut libc::c_void as *const libc::c_char);
    ulog_close();
    ustats_close();
    usysdep_exit(0 as libc::c_int);
}
unsafe extern "C" fn fcall(
    mut puuconf: pointer,
    mut zconfig: *const libc::c_char,
    mut fuuxqt: boolean,
    mut qorigsys: *const uuconf_system,
    mut qport: *mut uuconf_port,
    mut fifwork: boolean,
    mut fforce: boolean,
    mut fdetach: boolean,
    mut fquiet: boolean,
    mut ftrynext: boolean,
) -> boolean {
    let mut sstat: sstatus = sstatus {
        ttype: STATUS_COMPLETE,
        cretries: 0,
        ilast: 0,
        cwait: 0,
        zstring: 0 as *mut libc::c_char,
    };
    let mut inow: libc::c_long = 0;
    let mut fbadtime: boolean = 0;
    let mut fnevertime: boolean = 0;
    let mut ffoundwork: boolean = 0;
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    if fsysdep_get_status(qorigsys, &mut sstat, 0 as *mut libc::c_void as *mut boolean)
        == 0
    {
        return 0 as libc::c_int;
    }
    ubuffree(sstat.zstring);
    inow = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    if fforce == 0 {
        if (*qorigsys).uuconf_cmax_retries > 0 as libc::c_int
            && sstat.cretries >= (*qorigsys).uuconf_cmax_retries && sstat.ilast <= inow
            && sstat.ilast
                + 24 as libc::c_int as libc::c_long * 60 as libc::c_int as libc::c_long
                    * 60 as libc::c_int as libc::c_long > inow
        {
            ulog(LOG_ERROR, b"Too many retries\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        if (if sstat.ttype as libc::c_uint
            == STATUS_COMPLETE as libc::c_int as libc::c_uint
        {
            (sstat.ilast + (*qorigsys).uuconf_csuccess_wait as libc::c_long > inow)
                as libc::c_int
        } else {
            (sstat.ilast + sstat.cwait as libc::c_long > inow) as libc::c_int
        }) != 0 && sstat.ilast <= inow
        {
            ulog(
                LOG_NORMAL,
                b"Retry time not reached\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    sDaemon.puuconf = puuconf;
    sDaemon.zconfig = zconfig;
    if fuuxqt == 0 {
        sDaemon.irunuuxqt = 0 as libc::c_int;
    } else {
        let mut iuuconf: libc::c_int = 0;
        iuuconf = uuconf_runuuxqt(puuconf, &mut sDaemon.irunuuxqt);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        }
    }
    fbadtime = 1 as libc::c_int;
    fnevertime = 1 as libc::c_int;
    ffoundwork = 0 as libc::c_int;
    let mut current_block_69: u64;
    qsys = qorigsys;
    while !qsys.is_null() {
        let mut cretry: libc::c_int = 0;
        let mut fany: boolean = 0;
        let mut fret: boolean = 0;
        let mut fcalled: boolean = 0;
        if afSignal[0 as libc::c_int as usize] != 0
            || afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0
        {
            return 0 as libc::c_int;
        }
        if !((*qsys).uuconf_fcall == 0 || ((*qsys).uuconf_qtimegrade).is_null()) {
            if !qport.is_null()
                && (!((*qsys).uuconf_qport).is_null()
                    || !((*qsys).uuconf_zport).is_null()
                        && strcmp((*qport).uuconf_zname, (*qsys).uuconf_zport)
                            != 0 as libc::c_int)
            {
                let mut ql: *const uuconf_system = 0 as *const uuconf_system;
                ql = (*qsys).uuconf_qalternate;
                while !ql.is_null() {
                    if ((*ql).uuconf_qport).is_null() && !((*ql).uuconf_zport).is_null()
                        && strcmp((*ql).uuconf_zport, (*qport).uuconf_zname)
                            == 0 as libc::c_int
                    {
                        break;
                    }
                    ql = (*ql).uuconf_qalternate;
                }
                if !ql.is_null() {
                    current_block_69 = 13797916685926291137;
                } else {
                    current_block_69 = 11459959175219260272;
                }
            } else {
                current_block_69 = 11459959175219260272;
            }
            match current_block_69 {
                13797916685926291137 => {}
                _ => {
                    fnevertime = 0 as libc::c_int;
                    if !(ftimespan_match(
                        (*qsys).uuconf_qtimegrade,
                        0 as *mut libc::c_void as *mut libc::c_long,
                        &mut cretry,
                    ) == 0)
                    {
                        fbadtime = 0 as libc::c_int;
                        sDaemon.qsys = qsys;
                        sDaemon.zlocalname = 0 as *const libc::c_char;
                        sDaemon.qconn = 0 as *mut sconnection;
                        sDaemon.qproto = 0 as *const sprotocol;
                        sDaemon.cchans = 1 as libc::c_int;
                        sDaemon.clocal_size = -(1 as libc::c_int) as libc::c_long;
                        sDaemon.cremote_size = -(1 as libc::c_int) as libc::c_long;
                        sDaemon.cmax_ever = -(2 as libc::c_int) as libc::c_long;
                        sDaemon.cmax_receive = -(1 as libc::c_int) as libc::c_long;
                        sDaemon.csent = 0 as libc::c_int as libc::c_long;
                        sDaemon.creceived = 0 as libc::c_int as libc::c_long;
                        sDaemon.cxfiles_received = 0 as libc::c_int as libc::c_long;
                        sDaemon.ifeatures = 0 as libc::c_int;
                        sDaemon.frequest_hangup = 0 as libc::c_int;
                        sDaemon.fhangup_requested = 0 as libc::c_int;
                        sDaemon.fhangup = 0 as libc::c_int;
                        sDaemon.fmaster = 1 as libc::c_int;
                        sDaemon.fcaller = 1 as libc::c_int;
                        sDaemon.ireliable = 0 as libc::c_int;
                        sDaemon.bgrade = '\0' as i32 as libc::c_char;
                        if fqueue(&mut sDaemon, &mut fany) == 0 {
                            return 0 as libc::c_int;
                        }
                        if fifwork != 0 && fany == 0 {
                            uclear_queue(&mut sDaemon);
                        } else {
                            ffoundwork = 1 as libc::c_int;
                            fret = fconn_call(
                                &mut sDaemon,
                                qport,
                                &mut sstat,
                                cretry,
                                &mut fcalled,
                            );
                            uclear_queue(&mut sDaemon);
                            if fret != 0 {
                                return 1 as libc::c_int;
                            }
                            if fcalled != 0 && ftrynext == 0 {
                                return 0 as libc::c_int;
                            }
                            if fdetach != 0 {
                                fsysdep_unlock_system(&mut sLocked_system);
                                fLocked_system = 0 as libc::c_int;
                                usysdep_detach();
                                if fsysdep_lock_system(&mut sLocked_system) == 0 {
                                    return 0 as libc::c_int;
                                }
                                fLocked_system = 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        qsys = (*qsys).uuconf_qalternate;
    }
    if fbadtime != 0 {
        if fquiet == 0 {
            ulog(
                LOG_NORMAL,
                b"Wrong time to call\0" as *const u8 as *const libc::c_char,
            );
        }
        if fnevertime == 0 {
            sstat.ttype = STATUS_WRONG_TIME;
            sstat.ilast = inow;
            sstat.cwait = 0 as libc::c_int;
            fsysdep_set_status(qorigsys, &mut sstat);
        }
    } else if ffoundwork == 0 {
        if fquiet == 0 {
            ulog(LOG_NORMAL, b"No work\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fconn_call(
    mut qdaemon: *mut sdaemon,
    mut qport: *mut uuconf_port,
    mut qstat: *mut sstatus,
    mut cretry: libc::c_int,
    mut pfcalled: *mut boolean,
) -> boolean {
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut sport: uuconf_port = uuconf_port {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_ttype: UUCONF_PORTTYPE_UNKNOWN,
        uuconf_zprotocols: 0 as *mut libc::c_char,
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_ireliable: 0,
        uuconf_zlockname: 0 as *mut libc::c_char,
        uuconf_palloc: 0 as *mut libc::c_void,
        uuconf_u: C2RustUnnamed {
            uuconf_sstdin: uuconf_stdin_port {
                uuconf_idummy: 0,
            },
        },
    };
    let mut sconn: sconnection = sconnection {
        qcmds: 0 as *const sconncmds,
        psysdep: 0 as *mut libc::c_void,
        qport: 0 as *mut uuconf_port,
    };
    let mut terr: tstatus_type = STATUS_COMPLETE;
    let mut fret: boolean = 0;
    puuconf = (*qdaemon).puuconf;
    qsys = (*qdaemon).qsys;
    *pfcalled = 0 as libc::c_int;
    ::std::ptr::write_volatile(
        &mut afSignal[0 as libc::c_int as usize] as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    if qport.is_null() {
        qport = (*qsys).uuconf_qport;
    }
    if !qport.is_null() {
        if fconn_init(qport, &mut sconn, UUCONF_PORTTYPE_UNKNOWN) == 0 {
            return 0 as libc::c_int;
        }
        if fconn_lock(&mut sconn, 0 as libc::c_int, 0 as libc::c_int) == 0 {
            ulog(
                LOG_ERROR,
                b"%s: Port already locked\0" as *const u8 as *const libc::c_char,
                (*qport).uuconf_zname,
            );
            return 0 as libc::c_int;
        }
    } else {
        let mut s: spass = spass {
            fmatched: 0,
            flocked: 0,
            qconn: 0 as *mut sconnection,
        };
        let mut iuuconf: libc::c_int = 0;
        s.fmatched = 0 as libc::c_int;
        s.flocked = 0 as libc::c_int;
        s.qconn = &mut sconn;
        iuuconf = uuconf_find_port(
            puuconf,
            (*qsys).uuconf_zport,
            (*qsys).uuconf_ibaud,
            (*qsys).uuconf_ihighbaud,
            Some(
                iuport_lock
                    as unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
            ),
            &mut s as *mut spass as pointer,
            &mut sport,
        );
        if iuuconf == 1 as libc::c_int {
            if s.fmatched == 0 {
                ulog(
                    LOG_ERROR,
                    b"No matching ports\0" as *const u8 as *const libc::c_char,
                );
            } else {
                ulog(
                    LOG_ERROR,
                    b"All matching ports in use\0" as *const u8 as *const libc::c_char,
                );
                (*qstat).ttype = STATUS_PORT_FAILED;
                (*qstat)
                    .ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
                if cretry == 0 as libc::c_int {
                    (*qstat)
                        .cwait = (*qstat).cretries * 10 as libc::c_int
                        * 60 as libc::c_int;
                } else {
                    (*qstat).cwait = cretry * 60 as libc::c_int;
                }
                fsysdep_set_status(qsys, qstat);
            }
            return 0 as libc::c_int;
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            if s.flocked != 0 {
                fconn_unlock(&mut sconn);
                uconn_free(&mut sconn);
            }
            return 0 as libc::c_int;
        }
    }
    if fconn_open(
        &mut sconn,
        (*qsys).uuconf_ibaud,
        (*qsys).uuconf_ihighbaud,
        0 as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        terr = STATUS_PORT_FAILED;
        fret = 0 as libc::c_int;
    } else {
        let mut qdialer: *mut uuconf_dialer = 0 as *mut uuconf_dialer;
        let mut sdialer: uuconf_dialer = uuconf_dialer {
            uuconf_zname: 0 as *mut libc::c_char,
            uuconf_schat: uuconf_chat {
                uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                uuconf_pzprogram: 0 as *const *mut libc::c_char
                    as *mut *mut libc::c_char,
                uuconf_ctimeout: 0,
                uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                uuconf_fstrip: 0,
            },
            uuconf_zdialtone: 0 as *mut libc::c_char,
            uuconf_zpause: 0 as *mut libc::c_char,
            uuconf_fcarrier: 0,
            uuconf_ccarrier_wait: 0,
            uuconf_fdtr_toggle: 0,
            uuconf_fdtr_toggle_wait: 0,
            uuconf_scomplete: uuconf_chat {
                uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                uuconf_pzprogram: 0 as *const *mut libc::c_char
                    as *mut *mut libc::c_char,
                uuconf_ctimeout: 0,
                uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                uuconf_fstrip: 0,
            },
            uuconf_sabort: uuconf_chat {
                uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                uuconf_pzprogram: 0 as *const *mut libc::c_char
                    as *mut *mut libc::c_char,
                uuconf_ctimeout: 0,
                uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                uuconf_fstrip: 0,
            },
            uuconf_qproto_params: 0 as *mut uuconf_proto_param,
            uuconf_ireliable: 0,
            uuconf_palloc: 0 as *mut libc::c_void,
        };
        let mut tdialer: tdialerfound = DIALERFOUND_FALSE;
        if ((*qsys).uuconf_zalternate).is_null() {
            ulog(
                LOG_NORMAL,
                b"Calling system %s (port %s)\0" as *const u8 as *const libc::c_char,
                (*qsys).uuconf_zname,
                if zLdevice.is_null() {
                    b"unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char
                } else {
                    zLdevice
                },
            );
        } else {
            ulog(
                LOG_NORMAL,
                b"Calling system %s (alternate %s, port %s)\0" as *const u8
                    as *const libc::c_char,
                (*qsys).uuconf_zname,
                (*qsys).uuconf_zalternate,
                if zLdevice.is_null() {
                    b"unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char
                } else {
                    zLdevice
                },
            );
        }
        qdialer = 0 as *mut uuconf_dialer;
        if fconn_dial(
            &mut sconn,
            puuconf,
            qsys,
            (*qsys).uuconf_zphone,
            &mut sdialer,
            &mut tdialer,
        ) == 0
        {
            tdialer = DIALERFOUND_FALSE;
            terr = STATUS_DIAL_FAILED;
            fret = 0 as libc::c_int;
        } else {
            (*qdaemon).qconn = &mut sconn;
            if tdialer as libc::c_uint
                == DIALERFOUND_FALSE as libc::c_int as libc::c_uint
            {
                qdialer = 0 as *mut uuconf_dialer;
            } else {
                qdialer = &mut sdialer;
            }
            fret = fdo_call(qdaemon, qstat, qdialer, pfcalled, &mut terr);
        }
        fconn_close(&mut sconn, puuconf, qdialer, fret);
        if tdialer as libc::c_uint == DIALERFOUND_FREE as libc::c_int as libc::c_uint {
            uuconf_free_block(sdialer.uuconf_palloc);
        }
    }
    if fret == 0 {
        if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"Call failed: %d (%s)\0" as *const u8 as *const libc::c_char,
                terr as libc::c_int,
                *azStatus.as_mut_ptr().offset(terr as libc::c_int as isize),
            );
        }
        (*qstat).ttype = terr;
        (*qstat).cretries += 1;
        (*qstat).cretries;
        (*qstat).ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
        if cretry == 0 as libc::c_int {
            (*qstat).cwait = (*qstat).cretries * 10 as libc::c_int * 60 as libc::c_int;
        } else {
            (*qstat).cwait = cretry * 60 as libc::c_int;
        }
        fsysdep_set_status(qsys, qstat);
    }
    fconn_unlock(&mut sconn);
    uconn_free(&mut sconn);
    if qport.is_null() {
        uuconf_free_block(sport.uuconf_palloc);
    }
    return fret;
}
unsafe extern "C" fn fdo_call(
    mut qdaemon: *mut sdaemon,
    mut qstat: *mut sstatus,
    mut qdialer: *const uuconf_dialer,
    mut pfcalled: *mut boolean,
    mut pterr: *mut tstatus_type,
) -> boolean {
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut qconn: *mut sconnection = 0 as *mut sconnection;
    let mut iuuconf: libc::c_int = 0;
    let mut istrip: libc::c_int = 0;
    let mut fstrip: boolean = 0;
    let mut zport: *const libc::c_char = 0 as *const libc::c_char;
    let mut zstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut istart_time: libc::c_long = 0;
    let mut zlog: *mut libc::c_char = 0 as *mut libc::c_char;
    puuconf = (*qdaemon).puuconf;
    qsys = (*qdaemon).qsys;
    qconn = (*qdaemon).qconn;
    iuuconf = uuconf_strip(puuconf, &mut istrip);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        return 0 as libc::c_int;
    }
    fstrip = (istrip & 0o2 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    *pterr = STATUS_LOGIN_FAILED;
    if ((*qconn).qport).is_null() {
        zport = b"unknown\0" as *const u8 as *const libc::c_char;
    } else {
        zport = (*(*qconn).qport).uuconf_zname;
    }
    if fchat(
        qconn,
        puuconf,
        &(*qsys).uuconf_schat,
        qsys,
        0 as *mut libc::c_void as *const uuconf_dialer,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        zport,
        iconn_baud(qconn),
    ) == 0
    {
        return 0 as libc::c_int;
    }
    *pfcalled = 1 as libc::c_int;
    istart_time = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    *pterr = STATUS_HANDSHAKE_FAILED;
    zstr = zget_uucp_cmd(qconn, 1 as libc::c_int, fstrip);
    if zstr.is_null() {
        return 0 as libc::c_int;
    }
    if strncmp(
        zstr,
        b"Shere\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"Bad startup string (expected \"Shere\" got \"%s\")\0" as *const u8
                as *const libc::c_char,
            zstr,
        );
        ubuffree(zstr);
        return 0 as libc::c_int;
    }
    ulog(LOG_NORMAL, b"Login successful\0" as *const u8 as *const libc::c_char);
    (*qstat).ttype = STATUS_TALKING;
    (*qstat).ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    (*qstat).cretries = 0 as libc::c_int;
    (*qstat).cwait = 0 as libc::c_int;
    if fsysdep_set_status(qsys, qstat) == 0 {
        return 0 as libc::c_int;
    }
    if *zstr.offset(5 as libc::c_int as isize) as libc::c_int == '=' as i32 {
        let mut zheresys: *const libc::c_char = 0 as *const libc::c_char;
        let mut clen: size_t = 0;
        let mut icmp: libc::c_int = 0;
        zheresys = zstr.offset(6 as libc::c_int as isize);
        clen = strlen(zheresys);
        if clen == 7 as libc::c_int as libc::c_ulong
            || clen == 14 as libc::c_int as libc::c_ulong
        {
            icmp = strncmp(zheresys, (*qsys).uuconf_zname, clen);
        } else {
            icmp = strcmp(zheresys, (*qsys).uuconf_zname);
        }
        if icmp != 0 as libc::c_int {
            if !((*qsys).uuconf_pzalias).is_null() {
                let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                pz = (*qsys).uuconf_pzalias;
                while !(*pz).is_null() {
                    if clen == 7 as libc::c_int as libc::c_ulong
                        || clen == 14 as libc::c_int as libc::c_ulong
                    {
                        icmp = strncmp(zheresys, *pz, clen);
                    } else {
                        icmp = strcmp(zheresys, *pz);
                    }
                    if icmp == 0 as libc::c_int {
                        break;
                    }
                    pz = pz.offset(1);
                    pz;
                }
            }
            if icmp != 0 as libc::c_int {
                ulog(
                    LOG_ERROR,
                    b"Called wrong system (%s)\0" as *const u8 as *const libc::c_char,
                    zheresys,
                );
                ubuffree(zstr);
                return 0 as libc::c_int;
            }
        }
    } else if *zstr.offset(5 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
            ulog(
                LOG_DEBUG,
                b"fdo_call: Strange Shere: %s\0" as *const u8 as *const libc::c_char,
                zstr,
            );
        }
    }
    ubuffree(zstr);
    let mut ival: libc::c_long = 0;
    let mut bgrade: libc::c_char = 0;
    let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    if ftimespan_match(
        (*qsys).uuconf_qcalltimegrade,
        &mut ival,
        0 as *mut libc::c_void as *mut libc::c_int,
    ) == 0
    {
        bgrade = '\0' as i32 as libc::c_char;
    } else {
        bgrade = ival as libc::c_char;
    }
    if !((*qsys).uuconf_zlocalname).is_null() {
        (*qdaemon).zlocalname = (*qsys).uuconf_zlocalname;
    } else {
        iuuconf = uuconf_localname(puuconf, &mut (*qdaemon).zlocalname);
        if iuuconf == 1 as libc::c_int {
            (*qdaemon).zlocalname = zsysdep_localname();
            if ((*qdaemon).zlocalname).is_null() {
                return 0 as libc::c_int;
            }
        } else if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            return 0 as libc::c_int;
        }
    }
    zsend = zbufalc(
        (strlen((*qdaemon).zlocalname)).wrapping_add(70 as libc::c_int as libc::c_ulong),
    );
    if (*qsys).uuconf_fsequence == 0 {
        if bgrade as libc::c_int == '\0' as i32 {
            sprintf(
                zsend,
                b"S%s -R -N0%o\0" as *const u8 as *const libc::c_char,
                (*qdaemon).zlocalname,
                (0o1 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int
                    | 0o40 as libc::c_int) as libc::c_uint,
            );
        } else {
            sprintf(
                zsend,
                b"S%s -p%c -vgrade=%c -R -N0%o\0" as *const u8 as *const libc::c_char,
                (*qdaemon).zlocalname,
                bgrade as libc::c_int,
                bgrade as libc::c_int,
                (0o1 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int
                    | 0o40 as libc::c_int) as libc::c_uint,
            );
        }
    } else {
        let mut iseq: libc::c_long = 0;
        iseq = ixsysdep_get_sequence(qsys);
        if iseq < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        if bgrade as libc::c_int == '\0' as i32 {
            sprintf(
                zsend,
                b"S%s -Q%ld -R -N0%o\0" as *const u8 as *const libc::c_char,
                (*qdaemon).zlocalname,
                iseq,
                (0o1 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int
                    | 0o40 as libc::c_int) as libc::c_uint,
            );
        } else {
            sprintf(
                zsend,
                b"S%s -Q%ld -p%c -vgrade=%c -R -N0%o\0" as *const u8
                    as *const libc::c_char,
                (*qdaemon).zlocalname,
                iseq,
                bgrade as libc::c_int,
                bgrade as libc::c_int,
                (0o1 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int
                    | 0o40 as libc::c_int) as libc::c_uint,
            );
        }
    }
    fret = fsend_uucp_cmd(qconn, zsend);
    ubuffree(zsend);
    if fret == 0 {
        return 0 as libc::c_int;
    }
    zstr = zget_uucp_cmd(qconn, 1 as libc::c_int, fstrip);
    if zstr.is_null() {
        return 0 as libc::c_int;
    }
    if *zstr.offset(0 as libc::c_int as isize) as libc::c_int != 'R' as i32 {
        ulog(
            LOG_ERROR,
            b"Bad response to handshake string (%s)\0" as *const u8
                as *const libc::c_char,
            zstr,
        );
        ubuffree(zstr);
        return 0 as libc::c_int;
    }
    if strncmp(
        zstr.offset(1 as libc::c_int as isize),
        b"OKN\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        if *zstr
            .offset(
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '\0' as i32
        {
            (*qdaemon).ifeatures |= 0o1 as libc::c_int | 0o10 as libc::c_int;
        } else {
            (*qdaemon).ifeatures
                |= strtol(
                    zstr
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                                as isize,
                        )
                        .offset(-(1 as libc::c_int as isize)),
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    0 as libc::c_int,
                ) as libc::c_int;
        }
    } else if strncmp(
        zstr.offset(1 as libc::c_int as isize),
        b"OK\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        if *zstr
            .offset(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int != '\0' as i32
        {
            let mut zopt: *mut libc::c_char = 0 as *mut libc::c_char;
            zopt = zstr
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize));
            while *zopt as libc::c_int != '\0' as i32 {
                let mut b: libc::c_char = 0;
                let mut c: libc::c_long = 0;
                let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
                let fresh2 = zopt;
                zopt = zopt.offset(1);
                b = *fresh2;
                if *(*__ctype_b_loc()).offset(b as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || b as libc::c_int != '-' as i32
                {
                    continue;
                }
                match *zopt as libc::c_int {
                    82 => {
                        (*qdaemon).ifeatures
                            |= 0o2 as libc::c_int | 0o20 as libc::c_int
                                | 0o1 as libc::c_int;
                    }
                    85 => {
                        c = strtol(zopt, &mut zend, 0 as libc::c_int);
                        if c > 0 as libc::c_int as libc::c_long
                            && c
                                <= 9223372036854775807 as libc::c_long
                                    / 512 as libc::c_int as libc::c_long
                        {
                            (*qdaemon)
                                .cmax_receive = c * 512 as libc::c_int as libc::c_long;
                        }
                        zopt = zend;
                    }
                    _ => {}
                }
                while *zopt as libc::c_int != '\0' as i32
                    && *(*__ctype_b_loc()).offset(*zopt as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    zopt = zopt.offset(1);
                    zopt;
                }
            }
        }
    } else if strcmp(
        zstr.offset(1 as libc::c_int as isize),
        b"CB\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        ulog(
            LOG_NORMAL,
            b"Remote system will call back\0" as *const u8 as *const libc::c_char,
        );
        (*qstat).ttype = STATUS_COMPLETE;
        fsysdep_set_status(qsys, qstat);
        ubuffree(zstr);
        return 1 as libc::c_int;
    } else {
        ulog(
            LOG_ERROR,
            b"Handshake failed (%s)\0" as *const u8 as *const libc::c_char,
            zstr.offset(1 as libc::c_int as isize),
        );
        ubuffree(zstr);
        return 0 as libc::c_int;
    }
    ubuffree(zstr);
    zstr = zget_uucp_cmd(qconn, 1 as libc::c_int, fstrip);
    if zstr.is_null() {
        return 0 as libc::c_int;
    }
    if *zstr.offset(0 as libc::c_int as isize) as libc::c_int != 'P' as i32 {
        ulog(
            LOG_ERROR,
            b"Bad protocol handshake (%s)\0" as *const u8 as *const libc::c_char,
            zstr,
        );
        ubuffree(zstr);
        return 0 as libc::c_int;
    }
    if !((*qconn).qport).is_null()
        && (*(*qconn).qport).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int
    {
        (*qdaemon).ireliable = (*(*qconn).qport).uuconf_ireliable;
    }
    if !qdialer.is_null()
        && (*qdialer).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int
    {
        if (*qdaemon).ireliable != 0 as libc::c_int {
            (*qdaemon).ireliable &= (*qdialer).uuconf_ireliable;
        } else {
            (*qdaemon).ireliable = (*qdialer).uuconf_ireliable;
        }
    }
    if (*qdaemon).ireliable == 0 as libc::c_int {
        (*qdaemon)
            .ireliable = 0o4 as libc::c_int | 0o2 as libc::c_int | 0o20 as libc::c_int
            | 0o1 as libc::c_int;
    }
    let mut i: size_t = 0;
    let mut ab: [libc::c_char; 5] = [0; 5];
    i = (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong);
    if !((*qsys).uuconf_zprotocols).is_null()
        || !((*qconn).qport).is_null()
            && !((*(*qconn).qport).uuconf_zprotocols).is_null()
    {
        let mut zproto: *const libc::c_char = 0 as *const libc::c_char;
        if !((*qsys).uuconf_zprotocols).is_null() {
            zproto = (*qsys).uuconf_zprotocols;
        } else {
            zproto = (*(*qconn).qport).uuconf_zprotocols;
        }
        while *zproto as libc::c_int != '\0' as i32 {
            if !(strchr(zstr.offset(1 as libc::c_int as isize), *zproto as libc::c_int))
                .is_null()
            {
                i = 0 as libc::c_int as size_t;
                while i
                    < (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<sprotocol>() as libc::c_ulong,
                        )
                {
                    if asProtocols[i as usize].bname as libc::c_int
                        == *zproto as libc::c_int
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if i
                    < (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<sprotocol>() as libc::c_ulong,
                        )
                {
                    break;
                }
            }
            zproto = zproto.offset(1);
            zproto;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong)
        {
            let mut ipr: libc::c_int = 0;
            ipr = asProtocols[i as usize].ireliable;
            if !(ipr & (*qdaemon).ireliable != ipr) {
                if !(strchr(
                    zstr.offset(1 as libc::c_int as isize),
                    asProtocols[i as usize].bname as libc::c_int,
                ))
                    .is_null()
                {
                    break;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    ubuffree(zstr);
    if i
        >= (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong)
    {
        fsend_uucp_cmd(qconn, b"UN\0" as *const u8 as *const libc::c_char);
        ulog(
            LOG_ERROR,
            b"No mutually supported protocols\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*qdaemon).qproto = &*asProtocols.as_ptr().offset(i as isize) as *const sprotocol;
    if (*qdaemon).ireliable & 0o20 as libc::c_int == 0 as libc::c_int {
        (*qdaemon).cchans = 1 as libc::c_int;
    } else {
        (*qdaemon).cchans = asProtocols[i as usize].cchans;
    }
    sprintf(
        ab.as_mut_ptr(),
        b"U%c\0" as *const u8 as *const libc::c_char,
        (*(*qdaemon).qproto).bname as libc::c_int,
    );
    if fsend_uucp_cmd(qconn, ab.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    if !((*(*qdaemon).qproto).qcmds).is_null() {
        if !((*qsys).uuconf_qproto_params).is_null() {
            uapply_proto_params(
                puuconf,
                (*(*qdaemon).qproto).bname as libc::c_int,
                (*(*qdaemon).qproto).qcmds,
                (*qsys).uuconf_qproto_params,
            );
        }
        if !((*qconn).qport).is_null()
            && !((*(*qconn).qport).uuconf_qproto_params).is_null()
        {
            uapply_proto_params(
                puuconf,
                (*(*qdaemon).qproto).bname as libc::c_int,
                (*(*qdaemon).qproto).qcmds,
                (*(*qconn).qport).uuconf_qproto_params,
            );
        }
        if !qdialer.is_null() && !((*qdialer).uuconf_qproto_params).is_null() {
            uapply_proto_params(
                puuconf,
                (*(*qdaemon).qproto).bname as libc::c_int,
                (*(*qdaemon).qproto).qcmds,
                (*qdialer).uuconf_qproto_params,
            );
        }
    }
    if (Some(((*(*qdaemon).qproto).pfstart).unwrap())).unwrap()(qdaemon, &mut zlog) == 0
    {
        return 0 as libc::c_int;
    }
    if zlog.is_null() {
        zlog = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        sprintf(
            zlog,
            b"protocol '%c'\0" as *const u8 as *const libc::c_char,
            (*(*qdaemon).qproto).bname as libc::c_int,
        );
    }
    ulog(
        LOG_NORMAL,
        b"Handshake successful (%s)\0" as *const u8 as *const libc::c_char,
        zlog,
    );
    ubuffree(zlog);
    *pterr = STATUS_FAILED;
    let mut fret_0: boolean = 0;
    let mut iend_time: libc::c_long = 0;
    fret_0 = floop(qdaemon);
    if fsend_uucp_cmd(qconn, b"OOOOOO\0" as *const u8 as *const libc::c_char) != 0
        && fsend_uucp_cmd(qconn, b"OOOOOO\0" as *const u8 as *const libc::c_char) != 0
    {
        let mut i_0: libc::c_int = 0;
        let mut fdone: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < 25 as libc::c_int {
            zstr = zget_uucp_cmd(qconn, 0 as libc::c_int, fstrip);
            if zstr.is_null() {
                break;
            }
            fdone = (strstr(zstr, b"OOOOOO\0" as *const u8 as *const libc::c_char)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
            ubuffree(zstr);
            if fdone != 0 {
                break;
            }
            i_0 += 1;
            i_0;
        }
    }
    iend_time = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    ulog(
        LOG_NORMAL,
        b"Call complete (%ld seconds %ld bytes %ld bps)\0" as *const u8
            as *const libc::c_char,
        iend_time - istart_time,
        (*qdaemon).csent + (*qdaemon).creceived,
        if iend_time != istart_time {
            ((*qdaemon).csent + (*qdaemon).creceived) / (iend_time - istart_time)
        } else {
            0 as libc::c_int as libc::c_long
        },
    );
    if fret_0 != 0 {
        (*qstat).ttype = STATUS_COMPLETE;
        (*qstat).ilast = iend_time;
        fsysdep_set_status(qsys, qstat);
    }
    if (*qdaemon).irunuuxqt == -(2 as libc::c_int)
        || (*qdaemon).irunuuxqt > 0 as libc::c_int
            && (*qdaemon).cxfiles_received > 0 as libc::c_int as libc::c_long
    {
        fspawn_uuxqt(
            1 as libc::c_int,
            (*(*qdaemon).qsys).uuconf_zname,
            (*qdaemon).zconfig,
        );
    }
    return fret_0;
}
unsafe extern "C" fn iuport_lock(
    mut qport: *mut uuconf_port,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut q: *mut spass = pinfo as *mut spass;
    (*q).fmatched = 1 as libc::c_int;
    if fconn_init(qport, (*q).qconn, UUCONF_PORTTYPE_UNKNOWN) == 0 {
        return 1 as libc::c_int
    } else if fconn_lock((*q).qconn, 0 as libc::c_int, 0 as libc::c_int) == 0 {
        uconn_free((*q).qconn);
        return 1 as libc::c_int;
    } else {
        (*q).flocked = 1 as libc::c_int;
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn flogin_prompt(
    mut puuconf: pointer,
    mut zconfig: *const libc::c_char,
    mut fuuxqt: boolean,
    mut qconn: *mut sconnection,
    mut zlogin: *const libc::c_char,
    mut pzsystem: *mut *const libc::c_char,
) -> boolean {
    let mut iuuconf: libc::c_int = 0;
    let mut istrip: libc::c_int = 0;
    let mut fstrip: boolean = 0;
    let mut zuser: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zpass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    let mut s: scallin_info = scallin_info {
        zuser: 0 as *const libc::c_char,
        zpass: 0 as *const libc::c_char,
    };
    if !pzsystem.is_null() {
        *pzsystem = 0 as *const libc::c_char;
    }
    if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"flogin_prompt: Waiting for login\0" as *const u8 as *const libc::c_char,
        );
    }
    iuuconf = uuconf_strip(puuconf, &mut istrip);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        return 0 as libc::c_int;
    }
    fstrip = (istrip & 0o1 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    zuser = 0 as *mut libc::c_char;
    if zlogin.is_null() {
        loop {
            ubuffree(zuser);
            if fconn_write(
                qconn,
                b"login: \0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                return 0 as libc::c_int;
            }
            zuser = zget_typed_line(qconn, fstrip);
            if !(!zuser.is_null() && *zuser as libc::c_int == '\0' as i32) {
                break;
            }
        }
        if zuser.is_null() {
            return 1 as libc::c_int;
        }
        zlogin = zuser;
    }
    if fconn_write(
        qconn,
        b"Password:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0
    {
        ubuffree(zuser);
        return 0 as libc::c_int;
    }
    zpass = zget_typed_line(qconn, fstrip);
    if zpass.is_null() {
        ubuffree(zuser);
        return 1 as libc::c_int;
    }
    fret = 1 as libc::c_int;
    s.zuser = zlogin;
    s.zpass = zpass;
    iuuconf = uuconf_callin(
        puuconf,
        Some(
            icallin_cmp
                as unsafe extern "C" fn(
                    libc::c_int,
                    pointer,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        &mut s as *mut scallin_info as *mut libc::c_void,
    );
    ubuffree(zpass);
    if iuuconf == 1 as libc::c_int {
        ulog(LOG_ERROR, b"Bad login\0" as *const u8 as *const libc::c_char);
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        fret = 0 as libc::c_int;
    } else {
        let mut iholddebug: libc::c_int = 0;
        iholddebug = iDebug;
        faccept_call(puuconf, zconfig, fuuxqt, zlogin, qconn, pzsystem);
        iDebug = iholddebug;
    }
    ubuffree(zuser);
    return fret;
}
unsafe extern "C" fn icallin_cmp(
    mut iwhich: libc::c_int,
    mut pinfo: pointer,
    mut zfile: *const libc::c_char,
) -> libc::c_int {
    let mut qinfo: *mut scallin_info = pinfo as *mut scallin_info;
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut icmp: libc::c_int = 0;
    zcopy = zbufcpy(zfile);
    cescape(zcopy);
    if iwhich == 0 as libc::c_int {
        icmp = strcmp((*qinfo).zuser, zcopy);
    } else {
        icmp = strcmp((*qinfo).zpass, zcopy);
    }
    ubuffree(zcopy);
    return (icmp == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn faccept_call(
    mut puuconf: pointer,
    mut zconfig: *const libc::c_char,
    mut fuuxqt: boolean,
    mut zlogin: *const libc::c_char,
    mut qconn: *mut sconnection,
    mut pzsystem: *mut *const libc::c_char,
) -> boolean {
    let mut istart_time: libc::c_long = 0;
    let mut iuuconf: libc::c_int = 0;
    let mut istrip: libc::c_int = 0;
    let mut fstrip: boolean = 0;
    let mut zport: *const libc::c_char = 0 as *const libc::c_char;
    let mut qport: *mut uuconf_port = 0 as *mut uuconf_port;
    let mut sport: uuconf_port = uuconf_port {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_ttype: UUCONF_PORTTYPE_UNKNOWN,
        uuconf_zprotocols: 0 as *mut libc::c_char,
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_ireliable: 0,
        uuconf_zlockname: 0 as *mut libc::c_char,
        uuconf_palloc: 0 as *mut libc::c_void,
        uuconf_u: C2RustUnnamed {
            uuconf_sstdin: uuconf_stdin_port {
                uuconf_idummy: 0,
            },
        },
    };
    let mut qdialer: *mut uuconf_dialer = 0 as *mut uuconf_dialer;
    let mut sdialer: uuconf_dialer = uuconf_dialer {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_schat: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zdialtone: 0 as *mut libc::c_char,
        uuconf_zpause: 0 as *mut libc::c_char,
        uuconf_fcarrier: 0,
        uuconf_ccarrier_wait: 0,
        uuconf_fdtr_toggle: 0,
        uuconf_fdtr_toggle_wait: 0,
        uuconf_scomplete: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_sabort: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_ireliable: 0,
        uuconf_palloc: 0 as *mut libc::c_void,
    };
    let mut ftcp_port: boolean = 0;
    let mut zsend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zspace: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    let mut zstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ssys: uuconf_system = uuconf_system {
        uuconf_zname: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_pzalias: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_qalternate: 0 as *const uuconf_system as *mut uuconf_system,
        uuconf_zalternate: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_fcall: 0,
        uuconf_fcalled: 0,
        uuconf_qtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalltimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalledtimegrade: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_cmax_retries: 0,
        uuconf_csuccess_wait: 0,
        uuconf_qcall_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcall_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalled_local_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_qcalled_remote_size: 0 as *const uuconf_timespan as *mut uuconf_timespan,
        uuconf_ibaud: 0,
        uuconf_ihighbaud: 0,
        uuconf_zport: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_qport: 0 as *const uuconf_port as *mut uuconf_port,
        uuconf_zphone: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_schat: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zcall_login: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_zcall_password: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_zcalled_login: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_fcallback: 0,
        uuconf_fsequence: 0,
        uuconf_zprotocols: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_qproto_params: 0 as *const uuconf_proto_param as *mut uuconf_proto_param,
        uuconf_scalled_chat: uuconf_chat {
            uuconf_pzchat: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zdebug: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_zmax_remote_debug: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_fsend_request: 0,
        uuconf_frec_request: 0,
        uuconf_fcall_transfer: 0,
        uuconf_fcalled_transfer: 0,
        uuconf_pzlocal_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzremote_send: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzlocal_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzremote_receive: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzpath: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzcmds: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_cfree_space: 0,
        uuconf_pzforward_from: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_pzforward_to: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        uuconf_zpubdir: 0 as *const libc::c_char,
        uuconf_zlocalname: 0 as *const libc::c_char as *mut libc::c_char,
        uuconf_cmax_file_time: 0,
        uuconf_palloc: 0 as *const libc::c_void as *mut libc::c_void,
    };
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut qany: *const uuconf_system = 0 as *const uuconf_system;
    let mut zloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sstat: sstatus = sstatus {
        ttype: STATUS_COMPLETE,
        cretries: 0,
        ilast: 0,
        cwait: 0,
        zstring: 0 as *mut libc::c_char,
    };
    let mut fgotseq: boolean = 0;
    let mut fgotn: boolean = 0;
    let mut i: size_t = 0;
    let mut zlog: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zgrade: *mut libc::c_char = 0 as *mut libc::c_char;
    if !pzsystem.is_null() {
        *pzsystem = 0 as *const libc::c_char;
    }
    ulog(
        LOG_NORMAL,
        b"Incoming call (login %s port %s)\0" as *const u8 as *const libc::c_char,
        zlogin,
        if zLdevice.is_null() {
            b"unknown\0" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            zLdevice
        },
    );
    istart_time = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    iuuconf = uuconf_strip(puuconf, &mut istrip);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        uaccept_call_cleanup(
            puuconf,
            0 as *mut libc::c_void as *mut uuconf_system,
            0 as *mut libc::c_void as *mut uuconf_port,
            &mut sport,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    fstrip = (istrip & 0o2 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    if !((*qconn).qport).is_null() {
        qport = (*qconn).qport;
        zport = (*qport).uuconf_zname;
        ftcp_port = 0 as libc::c_int;
    } else {
        zport = zsysdep_port_name(&mut ftcp_port);
        if zport.is_null() {
            qport = 0 as *mut uuconf_port;
            zport = b"unknown\0" as *const u8 as *const libc::c_char;
        } else {
            iuuconf = uuconf_find_port(
                puuconf,
                zport,
                0 as libc::c_int as libc::c_long,
                0 as libc::c_int as libc::c_long,
                ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<
                        unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
                    >,
                >(0 as *mut libc::c_void),
                0 as *mut libc::c_void,
                &mut sport,
            );
            if iuuconf == 1 as libc::c_int {
                qport = 0 as *mut uuconf_port;
            } else if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                uaccept_call_cleanup(
                    puuconf,
                    0 as *mut libc::c_void as *mut uuconf_system,
                    0 as *mut libc::c_void as *mut uuconf_port,
                    &mut sport,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                return 0 as libc::c_int;
            } else {
                qport = &mut sport;
            }
        }
    }
    qdialer = 0 as *mut uuconf_dialer;
    if !qport.is_null() {
        if (*qport).uuconf_ttype as libc::c_uint
            == UUCONF_PORTTYPE_MODEM as libc::c_int as libc::c_uint
        {
            if !((*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer).is_null() {
                let mut zdialer: *const libc::c_char = 0 as *const libc::c_char;
                zdialer = *((*qport).uuconf_u.uuconf_smodem.uuconf_pzdialer)
                    .offset(0 as libc::c_int as isize);
                iuuconf = uuconf_dialer_info(puuconf, zdialer, &mut sdialer);
                if iuuconf == 0 as libc::c_int {
                    qdialer = &mut sdialer;
                }
            } else {
                qdialer = (*qport).uuconf_u.uuconf_smodem.uuconf_qdialer;
            }
        } else if (*qport).uuconf_ttype as libc::c_uint
            == UUCONF_PORTTYPE_TCP as libc::c_int as libc::c_uint
            || (*qport).uuconf_ttype as libc::c_uint
                == UUCONF_PORTTYPE_TLI as libc::c_int as libc::c_uint
                && (*qport).uuconf_ireliable & 0o1 as libc::c_int == 0 as libc::c_int
        {
            ftcp_port = 1 as libc::c_int;
        }
    }
    sDaemon.puuconf = puuconf;
    sDaemon.zconfig = zconfig;
    if fuuxqt == 0 {
        sDaemon.irunuuxqt = 0 as libc::c_int;
    } else {
        iuuconf = uuconf_runuuxqt(puuconf, &mut sDaemon.irunuuxqt);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        }
    }
    sDaemon.qsys = 0 as *const uuconf_system;
    sDaemon.zlocalname = 0 as *const libc::c_char;
    sDaemon.qconn = qconn;
    sDaemon.qproto = 0 as *const sprotocol;
    sDaemon.cchans = 1 as libc::c_int;
    sDaemon.clocal_size = -(1 as libc::c_int) as libc::c_long;
    sDaemon.cremote_size = -(1 as libc::c_int) as libc::c_long;
    sDaemon.cmax_ever = -(2 as libc::c_int) as libc::c_long;
    sDaemon.cmax_receive = -(1 as libc::c_int) as libc::c_long;
    sDaemon.csent = 0 as libc::c_int as libc::c_long;
    sDaemon.creceived = 0 as libc::c_int as libc::c_long;
    sDaemon.cxfiles_received = 0 as libc::c_int as libc::c_long;
    sDaemon.ifeatures = 0 as libc::c_int;
    sDaemon.frequest_hangup = 0 as libc::c_int;
    sDaemon.fhangup_requested = 0 as libc::c_int;
    sDaemon.fhangup = 0 as libc::c_int;
    sDaemon.fmaster = 0 as libc::c_int;
    sDaemon.fcaller = 0 as libc::c_int;
    sDaemon.ireliable = 0 as libc::c_int;
    sDaemon.bgrade = 'z' as i32 as libc::c_char;
    iuuconf = uuconf_login_localname(puuconf, zlogin, &mut zloc);
    if iuuconf == 0 as libc::c_int {
        sDaemon.zlocalname = zloc;
    } else if iuuconf == 1 as libc::c_int {
        sDaemon.zlocalname = zsysdep_localname();
        if (sDaemon.zlocalname).is_null() {
            uaccept_call_cleanup(
                puuconf,
                0 as *mut libc::c_void as *mut uuconf_system,
                qport,
                &mut sport,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    } else {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        uaccept_call_cleanup(
            puuconf,
            0 as *mut libc::c_void as *mut uuconf_system,
            qport,
            &mut sport,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    zsend = zbufalc(
        (strlen(sDaemon.zlocalname))
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong),
    );
    sprintf(
        zsend,
        b"Shere=%s\0" as *const u8 as *const libc::c_char,
        sDaemon.zlocalname,
    );
    fret = fsend_uucp_cmd(qconn, zsend);
    ubuffree(zsend);
    if fret == 0 {
        uaccept_call_cleanup(
            puuconf,
            0 as *mut libc::c_void as *mut uuconf_system,
            qport,
            &mut sport,
            zloc,
        );
        return 0 as libc::c_int;
    }
    zstr = zget_uucp_cmd(qconn, 1 as libc::c_int, fstrip);
    if zstr.is_null() {
        uaccept_call_cleanup(
            puuconf,
            0 as *mut libc::c_void as *mut uuconf_system,
            qport,
            &mut sport,
            zloc,
        );
        return 0 as libc::c_int;
    }
    if *zstr.offset(0 as libc::c_int as isize) as libc::c_int != 'S' as i32 {
        ulog(
            LOG_ERROR,
            b"Bad introduction string\0" as *const u8 as *const libc::c_char,
        );
        ubuffree(zstr);
        uaccept_call_cleanup(
            puuconf,
            0 as *mut libc::c_void as *mut uuconf_system,
            qport,
            &mut sport,
            zloc,
        );
        return 0 as libc::c_int;
    }
    zspace = strchr(zstr, ' ' as i32);
    if !zspace.is_null() {
        *zspace = '\0' as i32 as libc::c_char;
    }
    iuuconf = uuconf_system_info(
        puuconf,
        zstr.offset(1 as libc::c_int as isize),
        &mut ssys,
    );
    if iuuconf == 1 as libc::c_int {
        let mut zscript: *mut libc::c_char = 0 as *mut libc::c_char;
        iuuconf = uuconf_remote_unknown(puuconf, &mut zscript);
        if iuuconf == 0 as libc::c_int {
            if fsysdep_unknown_caller(zscript, zstr.offset(1 as libc::c_int as isize))
                == 0
            {
                xfree(zscript as pointer);
                fsend_uucp_cmd(
                    qconn,
                    b"RYou are unknown to me\0" as *const u8 as *const libc::c_char,
                );
                ubuffree(zstr);
                uaccept_call_cleanup(
                    puuconf,
                    0 as *mut libc::c_void as *mut uuconf_system,
                    qport,
                    &mut sport,
                    zloc,
                );
                return 0 as libc::c_int;
            }
            xfree(zscript as pointer);
        } else if iuuconf != 1 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            ubuffree(zstr);
            uaccept_call_cleanup(
                puuconf,
                0 as *mut libc::c_void as *mut uuconf_system,
                qport,
                &mut sport,
                zloc,
            );
            return 0 as libc::c_int;
        }
        if funknown_system(puuconf, zstr.offset(1 as libc::c_int as isize), &mut ssys)
            == 0
        {
            fsend_uucp_cmd(
                qconn,
                b"RYou are unknown to me\0" as *const u8 as *const libc::c_char,
            );
            ulog(
                LOG_ERROR,
                b"Call from unknown system %s\0" as *const u8 as *const libc::c_char,
                zstr.offset(1 as libc::c_int as isize),
            );
            ubuffree(zstr);
            uaccept_call_cleanup(
                puuconf,
                0 as *mut libc::c_void as *mut uuconf_system,
                qport,
                &mut sport,
                zloc,
            );
            return 0 as libc::c_int;
        }
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        ubuffree(zstr);
        uaccept_call_cleanup(
            puuconf,
            0 as *mut libc::c_void as *mut uuconf_system,
            qport,
            &mut sport,
            zloc,
        );
        return 0 as libc::c_int;
    }
    qany = 0 as *const uuconf_system;
    qsys = &mut ssys;
    while !qsys.is_null() {
        if !((*qsys).uuconf_fcalled == 0) {
            if ((*qsys).uuconf_zcalled_login).is_null()
                || strcmp(
                    (*qsys).uuconf_zcalled_login,
                    b"ANY\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                if qany.is_null() {
                    qany = qsys;
                }
            } else if strcmp((*qsys).uuconf_zcalled_login, zlogin) == 0 as libc::c_int {
                break;
            }
        }
        qsys = (*qsys).uuconf_qalternate;
    }
    if qsys.is_null() && !qany.is_null() {
        iuuconf = uuconf_validate(puuconf, qany, zlogin);
        if iuuconf == 0 as libc::c_int {
            qsys = qany;
        } else if iuuconf != 1 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            ubuffree(zstr);
            uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
            return 0 as libc::c_int;
        }
    }
    if qsys.is_null() {
        fsend_uucp_cmd(qconn, b"RLOGIN\0" as *const u8 as *const libc::c_char);
        ulog(
            LOG_ERROR,
            b"System %s used wrong login name %s\0" as *const u8 as *const libc::c_char,
            zstr.offset(1 as libc::c_int as isize),
            zlogin,
        );
        ubuffree(zstr);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    sDaemon.qsys = qsys;
    if !pzsystem.is_null() {
        *pzsystem = zbufcpy((*qsys).uuconf_zname);
    }
    ulog_system((*qsys).uuconf_zname);
    if !((*qsys).uuconf_zdebug).is_null() {
        iDebug |= idebug_parse((*qsys).uuconf_zdebug);
    }
    if (*qsys).uuconf_fcallback != 0 {
        fsend_uucp_cmd(qconn, b"RCB\0" as *const u8 as *const libc::c_char);
        ulog(LOG_NORMAL, b"Will call back\0" as *const u8 as *const libc::c_char);
        sstat.ttype = STATUS_COMPLETE;
        sstat.cretries = 0 as libc::c_int;
        sstat.ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
        sstat.cwait = 0 as libc::c_int;
        fsysdep_set_status(qsys, &mut sstat);
        ubuffree(
            zsysdep_spool_commands(
                qsys,
                '0' as i32,
                0 as libc::c_int,
                0 as *mut libc::c_void as *const scmd,
                0 as *mut libc::c_void as *mut boolean,
            ),
        );
        ubuffree(zstr);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 1 as libc::c_int;
    }
    if fsysdep_lock_system(qsys) == 0 {
        if (*qsys).uuconf_fsequence != 0 {
            ixsysdep_get_sequence(qsys);
        }
        fsend_uucp_cmd(qconn, b"RLCK\0" as *const u8 as *const libc::c_char);
        ulog(LOG_ERROR, b"System already locked\0" as *const u8 as *const libc::c_char);
        ubuffree(zstr);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    sLocked_system = *qsys;
    fLocked_system = 1 as libc::c_int;
    sstat.ttype = STATUS_TALKING;
    sstat.cretries = 0 as libc::c_int;
    sstat.ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    sstat.cwait = 0 as libc::c_int;
    fsysdep_set_status(qsys, &mut sstat);
    fgotseq = 0 as libc::c_int;
    fgotn = 0 as libc::c_int;
    if !zspace.is_null() {
        let mut paz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut pzset: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        zspace = zspace.offset(1);
        zspace;
        paz = xmalloc(
            (strlen(zspace))
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        pzset = paz;
        let fresh3 = pzset;
        pzset = pzset.offset(1);
        *fresh3 = 0 as *mut libc::c_char;
        loop {
            while *zspace as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(*zspace as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                zspace = zspace.offset(1);
                zspace;
            }
            if *zspace as libc::c_int == '\0' as i32 {
                break;
            }
            let fresh4 = pzset;
            pzset = pzset.offset(1);
            *fresh4 = zspace;
            zspace = zspace.offset(1);
            zspace;
            while *zspace as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(*zspace as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                zspace = zspace.offset(1);
                zspace;
            }
            if *zspace as libc::c_int == '\0' as i32 {
                break;
            }
            let fresh5 = zspace;
            zspace = zspace.offset(1);
            *fresh5 = '\0' as i32 as libc::c_char;
        }
        if pzset != paz.offset(1 as libc::c_int as isize) {
            let mut iopt: libc::c_int = 0;
            *pzset = 0 as *mut libc::c_char;
            gnu_optind = 0 as libc::c_int;
            gnu_opterr = 0 as libc::c_int;
            loop {
                iopt = gnu_getopt(
                    pzset.offset_from(paz) as libc::c_long as libc::c_int,
                    paz,
                    b"N::p:Q:RU:v:x:\0" as *const u8 as *const libc::c_char,
                );
                if !(iopt != -(1 as libc::c_int)) {
                    break;
                }
                let mut iseq: libc::c_long = 0;
                let mut c: libc::c_long = 0;
                let mut b: libc::c_char = 0;
                let mut iwant: libc::c_int = 0;
                match iopt {
                    78 => {
                        fgotn = 1 as libc::c_int;
                        if gnu_optarg.is_null() {
                            sDaemon.ifeatures
                                |= 0o1 as libc::c_int | 0o10 as libc::c_int;
                        } else {
                            sDaemon.ifeatures
                                |= strtol(
                                    gnu_optarg,
                                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                                    0 as libc::c_int,
                                ) as libc::c_int;
                        }
                    }
                    112 => {
                        if *(*__ctype_b_loc())
                            .offset(
                                *gnu_optarg.offset(0 as libc::c_int as isize)
                                    as libc::c_uchar as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            sDaemon
                                .bgrade = *gnu_optarg.offset(0 as libc::c_int as isize);
                        }
                    }
                    81 => {
                        iseq = strtol(
                            gnu_optarg,
                            0 as *mut libc::c_void as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        );
                        if (*qsys).uuconf_fsequence != 0
                            && iseq != ixsysdep_get_sequence(qsys)
                        {
                            fsend_uucp_cmd(
                                qconn,
                                b"RBADSEQ\0" as *const u8 as *const libc::c_char,
                            );
                            ulog(
                                LOG_ERROR,
                                b"Out of sequence call rejected\0" as *const u8
                                    as *const libc::c_char,
                            );
                            sstat.ttype = STATUS_FAILED;
                            fsysdep_set_status(qsys, &mut sstat);
                            xfree(paz as pointer);
                            ubuffree(zstr);
                            uaccept_call_cleanup(
                                puuconf,
                                &mut ssys,
                                qport,
                                &mut sport,
                                zloc,
                            );
                            return 0 as libc::c_int;
                        }
                        fgotseq = 1 as libc::c_int;
                    }
                    82 => {
                        sDaemon.ifeatures |= 0o2 as libc::c_int;
                    }
                    85 => {
                        c = strtol(
                            gnu_optarg,
                            0 as *mut libc::c_void as *mut *mut libc::c_char,
                            0 as libc::c_int,
                        );
                        if c > 0 as libc::c_int as libc::c_long
                            && c
                                < 9223372036854775807 as libc::c_long
                                    / 512 as libc::c_int as libc::c_long
                        {
                            sDaemon
                                .cmax_receive = c * 512 as libc::c_int as libc::c_long;
                        }
                    }
                    118 => {
                        if strncmp(
                            gnu_optarg,
                            b"grade=\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0 as libc::c_int
                        {
                            b = *gnu_optarg
                                .offset(
                                    (::std::mem::size_of::<[libc::c_char; 7]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                );
                            if *(*__ctype_b_loc())
                                .offset(b as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                sDaemon.bgrade = b;
                            }
                        }
                    }
                    120 => {
                        iwant = strtol(
                            gnu_optarg,
                            0 as *mut libc::c_void as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        ) as libc::c_int;
                        if iwant <= 9 as libc::c_int {
                            iwant = ((1 as libc::c_int) << iwant) - 1 as libc::c_int;
                        }
                        if !((*qsys).uuconf_zmax_remote_debug).is_null() {
                            iwant &= idebug_parse((*qsys).uuconf_zmax_remote_debug);
                        } else {
                            iwant
                                &= 0o1 as libc::c_int | 0o2 as libc::c_int
                                    | 0o4 as libc::c_int;
                        }
                        if iDebug | iwant != iDebug {
                            iDebug |= iwant;
                            ulog(
                                LOG_NORMAL,
                                b"Setting debugging mode to 0%o\0" as *const u8
                                    as *const libc::c_char,
                                iDebug,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        xfree(paz as pointer);
    }
    ubuffree(zstr);
    if (*qsys).uuconf_fsequence != 0 && fgotseq == 0 {
        fsend_uucp_cmd(qconn, b"RBADSEQ\0" as *const u8 as *const libc::c_char);
        ulog(
            LOG_ERROR,
            b"No sequence number (call rejected)\0" as *const u8 as *const libc::c_char,
        );
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    let mut ab: [libc::c_char; 20] = [0; 20];
    let mut zreply: *const libc::c_char = 0 as *const libc::c_char;
    if fgotn == 0 {
        if sDaemon.ifeatures & 0o2 as libc::c_int == 0 as libc::c_int {
            zreply = b"ROK\0" as *const u8 as *const libc::c_char;
        } else {
            sDaemon.ifeatures |= 0o20 as libc::c_int | 0o1 as libc::c_int;
            zreply = b"ROK -R\0" as *const u8 as *const libc::c_char;
        }
    } else if sDaemon.ifeatures & 0o10 as libc::c_int != 0 as libc::c_int {
        zreply = b"ROKN\0" as *const u8 as *const libc::c_char;
    } else {
        sprintf(
            ab.as_mut_ptr(),
            b"ROKN0%o\0" as *const u8 as *const libc::c_char,
            (0o1 as libc::c_int | 0o4 as libc::c_int | 0o2 as libc::c_int
                | 0o40 as libc::c_int) as libc::c_uint,
        );
        zreply = ab.as_mut_ptr();
    }
    if fsend_uucp_cmd(qconn, zreply) == 0 {
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    if ftcp_port != 0 {
        sDaemon
            .ireliable = 0o1 as libc::c_int | 0o10 as libc::c_int | 0o4 as libc::c_int
            | 0o2 as libc::c_int | 0o20 as libc::c_int;
    } else {
        if !qport.is_null()
            && (*qport).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int
        {
            sDaemon.ireliable = (*qport).uuconf_ireliable;
        }
        if !qdialer.is_null()
            && (*qdialer).uuconf_ireliable & 0o1 as libc::c_int != 0 as libc::c_int
        {
            if sDaemon.ireliable != 0 as libc::c_int {
                sDaemon.ireliable &= (*qdialer).uuconf_ireliable;
            } else {
                sDaemon.ireliable = (*qdialer).uuconf_ireliable;
            }
        }
        if sDaemon.ireliable == 0 as libc::c_int {
            sDaemon
                .ireliable = 0o4 as libc::c_int | 0o2 as libc::c_int
                | 0o20 as libc::c_int | 0o1 as libc::c_int;
        }
    }
    if !((*qsys).uuconf_zprotocols).is_null()
        || !qport.is_null() && !((*qport).uuconf_zprotocols).is_null()
    {
        let mut zprotos: *const libc::c_char = 0 as *const libc::c_char;
        if !((*qsys).uuconf_zprotocols).is_null() {
            zprotos = (*qsys).uuconf_zprotocols;
        } else {
            zprotos = (*qport).uuconf_zprotocols;
        }
        zsend = zbufalc(
            (strlen(zprotos)).wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        sprintf(zsend, b"P%s\0" as *const u8 as *const libc::c_char, zprotos);
    } else {
        let mut zset: *mut libc::c_char = 0 as *mut libc::c_char;
        zsend = zbufalc(
            (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        zset = zsend;
        let fresh6 = zset;
        zset = zset.offset(1);
        *fresh6 = 'P' as i32 as libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong)
        {
            let mut ipr: libc::c_int = 0;
            ipr = asProtocols[i as usize].ireliable;
            if !(ipr & sDaemon.ireliable != ipr) {
                let fresh7 = zset;
                zset = zset.offset(1);
                *fresh7 = asProtocols[i as usize].bname;
            }
            i = i.wrapping_add(1);
            i;
        }
        *zset = '\0' as i32 as libc::c_char;
    }
    fret = fsend_uucp_cmd(qconn, zsend);
    ubuffree(zsend);
    if fret == 0 {
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    zstr = zget_uucp_cmd(qconn, 1 as libc::c_int, fstrip);
    if zstr.is_null() {
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    if *zstr.offset(0 as libc::c_int as isize) as libc::c_int != 'U' as i32 {
        ulog(
            LOG_ERROR,
            b"Bad protocol response string\0" as *const u8 as *const libc::c_char,
        );
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        ubuffree(zstr);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    if *zstr.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
        ulog(LOG_ERROR, b"No supported protocol\0" as *const u8 as *const libc::c_char);
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        ubuffree(zstr);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong)
    {
        if asProtocols[i as usize].bname as libc::c_int
            == *zstr.offset(1 as libc::c_int as isize) as libc::c_int
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    ubuffree(zstr);
    if i
        >= (::std::mem::size_of::<[sprotocol; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<sprotocol>() as libc::c_ulong)
    {
        ulog(LOG_ERROR, b"No supported protocol\0" as *const u8 as *const libc::c_char);
        sstat.ttype = STATUS_FAILED;
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    sDaemon.qproto = &*asProtocols.as_ptr().offset(i as isize) as *const sprotocol;
    if sDaemon.ireliable & 0o20 as libc::c_int == 0 as libc::c_int {
        sDaemon.cchans = 1 as libc::c_int;
    } else {
        sDaemon.cchans = asProtocols[i as usize].cchans;
    }
    if fchat(
        qconn,
        puuconf,
        &(*qsys).uuconf_scalled_chat,
        qsys,
        0 as *mut libc::c_void as *const uuconf_dialer,
        0 as *mut libc::c_void as *const libc::c_char,
        0 as libc::c_int,
        zport,
        iconn_baud(qconn),
    ) == 0
    {
        sstat.ttype = STATUS_FAILED;
        sstat.ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    if !((*sDaemon.qproto).qcmds).is_null() {
        if !((*qsys).uuconf_qproto_params).is_null() {
            uapply_proto_params(
                puuconf,
                (*sDaemon.qproto).bname as libc::c_int,
                (*sDaemon.qproto).qcmds,
                (*qsys).uuconf_qproto_params,
            );
        }
        if !qport.is_null() && !((*qport).uuconf_qproto_params).is_null() {
            uapply_proto_params(
                puuconf,
                (*sDaemon.qproto).bname as libc::c_int,
                (*sDaemon.qproto).qcmds,
                (*qport).uuconf_qproto_params,
            );
        }
        if !qdialer.is_null() && !((*qdialer).uuconf_qproto_params).is_null() {
            uapply_proto_params(
                puuconf,
                (*sDaemon.qproto).bname as libc::c_int,
                (*sDaemon.qproto).qcmds,
                (*qdialer).uuconf_qproto_params,
            );
        }
    }
    if qdialer == &mut sdialer as *mut uuconf_dialer {
        uuconf_free_block(sdialer.uuconf_palloc);
    }
    if (Some(((*sDaemon.qproto).pfstart).unwrap())).unwrap()(&mut sDaemon, &mut zlog)
        == 0 || fqueue(&mut sDaemon, 0 as *mut libc::c_void as *mut boolean) == 0
    {
        uclear_queue(&mut sDaemon);
        sstat.ttype = STATUS_FAILED;
        sstat.ilast = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
        fsysdep_set_status(qsys, &mut sstat);
        uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
        return 0 as libc::c_int;
    }
    if zlog.is_null() {
        zlog = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        sprintf(
            zlog,
            b"protocol '%c'\0" as *const u8 as *const libc::c_char,
            (*sDaemon.qproto).bname as libc::c_int,
        );
    }
    zgrade = zbufalc(
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if sDaemon.bgrade as libc::c_int == 'z' as i32 {
        *zgrade = '\0' as i32 as libc::c_char;
    } else {
        sprintf(
            zgrade,
            b"grade %c \0" as *const u8 as *const libc::c_char,
            sDaemon.bgrade as libc::c_int,
        );
    }
    ulog(
        LOG_NORMAL,
        b"Handshake successful (%s%s)\0" as *const u8 as *const libc::c_char,
        zgrade,
        zlog,
    );
    ubuffree(zlog);
    ubuffree(zgrade);
    let mut iend_time: libc::c_long = 0;
    fret = floop(&mut sDaemon);
    if fsend_uucp_cmd(qconn, b"OOOOOOO\0" as *const u8 as *const libc::c_char) != 0
        && fsend_uucp_cmd(qconn, b"OOOOOOO\0" as *const u8 as *const libc::c_char) != 0
    {
        let mut fdone: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < 25 as libc::c_int as libc::c_ulong {
            zstr = zget_uucp_cmd(qconn, 0 as libc::c_int, fstrip);
            if zstr.is_null() {
                break;
            }
            fdone = (strstr(zstr, b"OOOOOO\0" as *const u8 as *const libc::c_char)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
            ubuffree(zstr);
            if fdone != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    iend_time = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    ulog(
        LOG_NORMAL,
        b"Call complete (%ld seconds %ld bytes %ld bps)\0" as *const u8
            as *const libc::c_char,
        iend_time - istart_time,
        sDaemon.csent + sDaemon.creceived,
        if iend_time != istart_time {
            (sDaemon.csent + sDaemon.creceived) / (iend_time - istart_time)
        } else {
            0 as libc::c_int as libc::c_long
        },
    );
    uclear_queue(&mut sDaemon);
    if fret != 0 {
        sstat.ttype = STATUS_COMPLETE;
    } else {
        sstat.ttype = STATUS_FAILED;
    }
    sstat.ilast = iend_time;
    fsysdep_set_status(qsys, &mut sstat);
    if sDaemon.irunuuxqt == -(2 as libc::c_int)
        || sDaemon.irunuuxqt > 0 as libc::c_int
            && sDaemon.cxfiles_received > 0 as libc::c_int as libc::c_long
    {
        fspawn_uuxqt(1 as libc::c_int, (*qsys).uuconf_zname, zconfig);
    }
    uaccept_call_cleanup(puuconf, &mut ssys, qport, &mut sport, zloc);
    return fret;
}
unsafe extern "C" fn uaccept_call_cleanup(
    mut puuconf: pointer,
    mut qfreesys: *mut uuconf_system,
    mut qport: *mut uuconf_port,
    mut qfreeport: *mut uuconf_port,
    mut zloc: *mut libc::c_char,
) {
    if fLocked_system != 0 {
        fsysdep_unlock_system(&mut sLocked_system);
        fLocked_system = 0 as libc::c_int;
    }
    if !qfreesys.is_null() {
        uuconf_free_block((*qfreesys).uuconf_palloc);
    }
    if qport == qfreeport {
        uuconf_free_block((*qfreeport).uuconf_palloc);
    }
    xfree(zloc as pointer);
    ulog_system(0 as *mut libc::c_void as *const libc::c_char);
}
unsafe extern "C" fn uapply_proto_params(
    mut puuconf: pointer,
    mut bproto: libc::c_int,
    mut qcmds: *mut uuconf_cmdtab,
    mut pas: *mut uuconf_proto_param,
) {
    let mut qp: *mut uuconf_proto_param = 0 as *mut uuconf_proto_param;
    qp = pas;
    while (*qp).uuconf_bproto != '\0' as i32 {
        if (*qp).uuconf_bproto == bproto {
            let mut qe: *mut uuconf_proto_param_entry = 0
                as *mut uuconf_proto_param_entry;
            qe = (*qp).uuconf_qentries;
            while (*qe).uuconf_cargs > 0 as libc::c_int {
                let mut iuuconf: libc::c_int = 0;
                iuuconf = uuconf_cmd_args(
                    puuconf,
                    (*qe).uuconf_cargs,
                    (*qe).uuconf_pzargs,
                    qcmds,
                    0 as *mut libc::c_void,
                    ::std::mem::transmute::<
                        *mut libc::c_void,
                        uuconf_cmdtabfn,
                    >(0 as *mut libc::c_void),
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                );
                if iuuconf & 0xff as libc::c_int != 0 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"Error in %c protocol parameters\0" as *const u8
                            as *const libc::c_char,
                        bproto,
                    );
                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                }
                qe = qe.offset(1);
                qe;
            }
            break;
        } else {
            qp = qp.offset(1);
            qp;
        }
    }
}
unsafe extern "C" fn fsend_uucp_cmd(
    mut qconn: *mut sconnection,
    mut z: *const libc::c_char,
) -> boolean {
    let mut cwrite: size_t = 0;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG,
            b"fsend_uucp_cmd: Sending \"%s\"\0" as *const u8 as *const libc::c_char,
            z,
        );
    }
    cwrite = (strlen(z)).wrapping_add(2 as libc::c_int as libc::c_ulong);
    zalc = zbufalc(cwrite);
    *zalc.offset(0 as libc::c_int as isize) = '\u{10}' as i32 as libc::c_char;
    memcpy(
        zalc.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        z as *const libc::c_void,
        cwrite.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    fret = fconn_write(qconn, zalc, cwrite);
    ubuffree(zalc);
    return fret;
}
unsafe extern "C" fn zget_uucp_cmd(
    mut qconn: *mut sconnection,
    mut frequired: boolean,
    mut fstrip: boolean,
) -> *mut libc::c_char {
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut calc: size_t = 0;
    let mut cgot: size_t = 0;
    let mut fintro: boolean = 0;
    let mut iendtime: libc::c_long = 0;
    let mut ctimeout: libc::c_int = 0;
    let mut cchars: libc::c_int = 0;
    let mut iolddebug: libc::c_int = 0;
    iendtime = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    if frequired != 0 {
        iendtime += 120 as libc::c_int as libc::c_long;
    } else {
        iendtime += 10 as libc::c_int as libc::c_long;
    }
    cchars = 0 as libc::c_int;
    iolddebug = iDebug;
    if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG_START,
            b"zget_uucp_cmd: Got \"\0" as *const u8 as *const libc::c_char,
        );
        iDebug &= !(0o1000 as libc::c_int | 0o40 as libc::c_int);
    }
    zalc = 0 as *mut libc::c_char;
    calc = 0 as libc::c_int as size_t;
    cgot = 0 as libc::c_int as size_t;
    fintro = 0 as libc::c_int;
    loop {
        ctimeout = (iendtime
            - ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)) as libc::c_int;
        if !(ctimeout > 0 as libc::c_int) {
            break;
        }
        let mut b: libc::c_int = 0;
        b = breceive_char(qconn, ctimeout, frequired);
        if b < 0 as libc::c_int {
            if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
                ulog(
                    LOG_DEBUG_END,
                    b"\" (%s)\0" as *const u8 as *const libc::c_char,
                    if b == -(1 as libc::c_int) {
                        b"timeout\0" as *const u8 as *const libc::c_char
                    } else {
                        b"error\0" as *const u8 as *const libc::c_char
                    },
                );
                iDebug = iolddebug;
            }
            if b == -(1 as libc::c_int) && frequired != 0 {
                ulog(LOG_ERROR, b"Timeout\0" as *const u8 as *const libc::c_char);
            }
            ubuffree(zalc);
            return 0 as *mut libc::c_char;
        }
        if fstrip != 0 {
            b &= 0x7f as libc::c_int;
        }
        if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
            let mut ab: [libc::c_char; 5] = [0; 5];
            cchars += 1;
            cchars;
            if cchars > 60 as libc::c_int {
                ulog(LOG_DEBUG_END, b"\"\0" as *const u8 as *const libc::c_char);
                ulog(
                    LOG_DEBUG_START,
                    b"zget_uucp_cmd: Got \"\0" as *const u8 as *const libc::c_char,
                );
                cchars = 0 as libc::c_int;
            }
            cdebug_char(ab.as_mut_ptr(), b);
            ulog(
                LOG_DEBUG_CONTINUE,
                b"%s\0" as *const u8 as *const libc::c_char,
                ab.as_mut_ptr(),
            );
        }
        if fintro == 0 {
            if b == '\u{10}' as i32 {
                fintro = 1 as libc::c_int;
            }
        } else if b == '\u{10}' as i32 {
            cgot = 0 as libc::c_int as size_t;
        } else {
            if b == '\r' as i32 || b == '\n' as i32 {
                b = '\0' as i32;
            }
            if cgot >= calc {
                let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
                calc = (calc as libc::c_ulong)
                    .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
                znew = zbufalc(calc);
                if cgot > 0 as libc::c_int as libc::c_ulong {
                    memcpy(znew as *mut libc::c_void, zalc as *const libc::c_void, cgot);
                }
                ubuffree(zalc);
                zalc = znew;
            }
            *zalc.offset(cgot as isize) = b as libc::c_char;
            cgot = cgot.wrapping_add(1);
            cgot;
            if b == '\0' as i32 {
                if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
                    ulog(LOG_DEBUG_END, b"\"\0" as *const u8 as *const libc::c_char);
                    iDebug = iolddebug;
                }
                return zalc;
            }
        }
    }
    if iDebug & 0o4 as libc::c_int != 0 as libc::c_int {
        ulog(LOG_DEBUG_END, b"\" (timeout)\0" as *const u8 as *const libc::c_char);
        iDebug = iolddebug;
    }
    ubuffree(zalc);
    if frequired != 0 {
        ulog(LOG_ERROR, b"Timeout\0" as *const u8 as *const libc::c_char);
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn zget_typed_line(
    mut qconn: *mut sconnection,
    mut fstrip: boolean,
) -> *mut libc::c_char {
    static mut flastcr: boolean = 0;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut calc: size_t = 0;
    let mut cgot: size_t = 0;
    let mut cchars: libc::c_int = 0;
    let mut iolddebug: libc::c_int = 0;
    cchars = 0 as libc::c_int;
    iolddebug = iDebug;
    if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG_START,
            b"zget_typed_line: Got \"\0" as *const u8 as *const libc::c_char,
        );
        iDebug &= !(0o1000 as libc::c_int | 0o40 as libc::c_int);
    }
    zalc = 0 as *mut libc::c_char;
    calc = 0 as libc::c_int as size_t;
    cgot = 0 as libc::c_int as size_t;
    loop {
        let mut b: libc::c_int = 0;
        b = breceive_char(qconn, 120 as libc::c_int, 0 as libc::c_int);
        if b == -(2 as libc::c_int)
            || (afSignal[0 as libc::c_int as usize] != 0
                || afSignal[1 as libc::c_int as usize] != 0
                || afSignal[2 as libc::c_int as usize] != 0
                || afSignal[3 as libc::c_int as usize] != 0
                || afSignal[4 as libc::c_int as usize] != 0)
        {
            if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
                ulog(LOG_DEBUG_END, b"\" (error)\0" as *const u8 as *const libc::c_char);
                iDebug = iolddebug;
            }
            ubuffree(zalc);
            flastcr = 0 as libc::c_int;
            return 0 as *mut libc::c_char;
        }
        if b == -(1 as libc::c_int) {
            flastcr = 0 as libc::c_int;
        } else {
            if fstrip != 0 {
                b &= 0x7f as libc::c_int;
            }
            if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
                let mut ab: [libc::c_char; 5] = [0; 5];
                cchars += 1;
                cchars;
                if cchars > 60 as libc::c_int {
                    ulog(LOG_DEBUG_END, b"\"\0" as *const u8 as *const libc::c_char);
                    ulog(
                        LOG_DEBUG_START,
                        b"zget_typed_line: Got \"\0" as *const u8 as *const libc::c_char,
                    );
                    cchars = 0 as libc::c_int;
                }
                cdebug_char(ab.as_mut_ptr(), b);
                ulog(
                    LOG_DEBUG_CONTINUE,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ab.as_mut_ptr(),
                );
            }
            if b == '\n' as i32 && cgot == 0 as libc::c_int as libc::c_ulong
                && flastcr != 0
            {
                flastcr = 0 as libc::c_int;
            } else {
                flastcr = 0 as libc::c_int;
                if cgot >= calc {
                    let mut znew: *mut libc::c_char = 0 as *mut libc::c_char;
                    calc = (calc as libc::c_ulong)
                        .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    znew = zbufalc(calc);
                    if cgot > 0 as libc::c_int as libc::c_ulong {
                        memcpy(
                            znew as *mut libc::c_void,
                            zalc as *const libc::c_void,
                            cgot,
                        );
                    }
                    ubuffree(zalc);
                    zalc = znew;
                }
                if b == '\n' as i32 {
                    b = '\0' as i32;
                } else if b == '\r' as i32 {
                    flastcr = 1 as libc::c_int;
                    b = '\0' as i32;
                }
                *zalc.offset(cgot as isize) = b as libc::c_char;
                cgot = cgot.wrapping_add(1);
                cgot;
                if b == '\0' as i32 {
                    if iDebug & 0o2 as libc::c_int != 0 as libc::c_int {
                        ulog(LOG_DEBUG_END, b"\"\0" as *const u8 as *const libc::c_char);
                        iDebug = iolddebug;
                    }
                    return zalc;
                }
            }
        }
    };
}
pub unsafe extern "C" fn fspawn_uuxqt(
    mut ffork: boolean,
    mut zsys: *const libc::c_char,
    mut zconfig: *const libc::c_char,
) -> boolean {
    let mut zconfigarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    if zconfig.is_null() {
        zconfigarg = 0 as *mut libc::c_char;
    } else {
        zconfigarg = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_add(strlen(zconfig)),
        );
        sprintf(zconfigarg, b"-I%s\0" as *const u8 as *const libc::c_char, zconfig);
    }
    fret = fsysdep_run(
        ffork,
        b"uuxqt\0" as *const u8 as *const libc::c_char,
        zconfigarg,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    ubuffree(zconfigarg);
    return fret;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
