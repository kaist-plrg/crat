use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_fatal_fn(pfn: Option::<unsafe extern "C" fn() -> ()>);
    fn ulog_close();
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xfree(_: pointer);
    static mut zProgram: *const libc::c_char;
    static mut afSignal: [sig_atomic_t; 5];
    static mut pfLstart: Option::<unsafe extern "C" fn() -> ()>;
    static mut pfLend: Option::<unsafe extern "C" fn() -> ()>;
    fn uuconf_init(
        uuconf_ppglobal: *mut *mut libc::c_void,
        uuconf_zprogram: *const libc::c_char,
        uuconf_zname: *const libc::c_char,
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
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debuglevel(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
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
    fn fconn_break(qconn: *mut sconnection) -> boolean;
    fn fconn_set(
        qconn: *mut sconnection,
        tparity: tparitysetting,
        tstrip: tstripsetting,
        txonxoff: txonxoffsetting,
    ) -> boolean;
    fn fconn_carrier(qconn: *mut sconnection, fcarrier: boolean) -> boolean;
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
    static mut iPrecstart: libc::c_int;
    static mut iPrecend: libc::c_int;
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn zsysdep_localname() -> *const libc::c_char;
    fn usysdep_signal(isig: libc::c_int);
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn esysdep_user_fopen(
        zfile: *const libc::c_char,
        frd: boolean,
        fbinary: boolean,
    ) -> openfile_t;
    fn fsysdep_sync(e: openfile_t, zmsg: *const libc::c_char) -> boolean;
    fn zsysdep_base_name(zfile: *const libc::c_char) -> *mut libc::c_char;
    fn fsysdep_port_access(qport: *mut uuconf_port) -> boolean;
    fn fsysdep_port_is_line(
        qport: *mut uuconf_port,
        zline: *const libc::c_char,
    ) -> boolean;
    fn fsysdep_terminal_raw(flocalecho: boolean) -> boolean;
    fn fsysdep_terminal_restore() -> boolean;
    fn zsysdep_terminal_line(zprompt: *const libc::c_char) -> *mut libc::c_char;
    fn fsysdep_terminal_puts(zline: *const libc::c_char) -> boolean;
    fn fsysdep_terminal_signals(faccept: boolean) -> boolean;
    fn fsysdep_cu_init(qconn: *mut sconnection) -> boolean;
    fn fsysdep_cu(
        qconn: *mut sconnection,
        pbcmd: *mut libc::c_char,
        zlocalname: *const libc::c_char,
    ) -> boolean;
    fn fsysdep_cu_copy(fcopy: boolean) -> boolean;
    fn fsysdep_cu_finish() -> boolean;
    fn fsysdep_shell(
        qconn: *mut sconnection,
        zcmd: *const libc::c_char,
        tcmd: tshell_cmd,
    ) -> boolean;
    fn fsysdep_chdir(zdir: *const libc::c_char) -> boolean;
    fn fsysdep_suspend() -> boolean;
    static mut gnu_optarg: *mut libc::c_char;
    static mut gnu_optind: libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub const _ISprint: C2RustUnnamed_0 = 16384;
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
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type tshell_cmd = libc::c_uint;
pub const SHELL_STDIO_ON_PORT: tshell_cmd = 3;
pub const SHELL_STDIN_FROM_PORT: tshell_cmd = 2;
pub const SHELL_STDOUT_TO_PORT: tshell_cmd = 1;
pub const SHELL_NORMAL: tshell_cmd = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sconninfo {
    pub fmatched: boolean,
    pub flocked: boolean,
    pub fdirect: boolean,
    pub qconn: *mut sconnection,
    pub zline: *const libc::c_char,
}
pub const _ISdigit: C2RustUnnamed_0 = 2048;
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
pub type _argtype = libc::c_uint;
pub const optional_argument: _argtype = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub static mut cu_rcsid: [libc::c_char; 47] = unsafe {
    *::std::mem::transmute::<
        &[u8; 47],
        &[libc::c_char; 47],
    >(b"$Id: cu.c,v 1.47 2002/03/05 19:10:41 ian Rel $\0")
};
pub static mut zCuvar_escape: *const libc::c_char = b"~\0" as *const u8
    as *const libc::c_char;
pub static mut fCuvar_delay: boolean = 1 as libc::c_int;
pub static mut zCuvar_eol: *const libc::c_char = b"\r\x15\x03\x0F\x04\x13\x11\x12\0"
    as *const u8 as *const libc::c_char;
pub static mut fCuvar_binary: boolean = 0 as libc::c_int;
pub static mut zCuvar_binary_prefix: *const libc::c_char = b"\x16\0" as *const u8
    as *const libc::c_char;
pub static mut fCuvar_echocheck: boolean = 0 as libc::c_int;
pub static mut zCuvar_echonl: *const libc::c_char = b"\r\0" as *const u8
    as *const libc::c_char;
pub static mut cCuvar_timeout: libc::c_int = 30 as libc::c_int;
pub static mut zCuvar_kill: *const libc::c_char = b"\x15\0" as *const u8
    as *const libc::c_char;
pub static mut cCuvar_resend: libc::c_int = 10 as libc::c_int;
pub static mut zCuvar_eofwrite: *const libc::c_char = b"\x04\0" as *const u8
    as *const libc::c_char;
pub static mut zCuvar_eofread: *const libc::c_char = b"$\0" as *const u8
    as *const libc::c_char;
pub static mut fCuvar_verbose: boolean = 1 as libc::c_int;
static mut asCuvars: [uuconf_cmdtab; 14] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"escape\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_escape as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"delay\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x12 as libc::c_int,
                uuconf_pvar: &fCuvar_delay as *const boolean as *mut boolean as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"eol\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_eol as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"binary\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x12 as libc::c_int,
                uuconf_pvar: &fCuvar_binary as *const boolean as *mut boolean as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"binary-prefix\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_binary_prefix as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"echocheck\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x12 as libc::c_int,
                uuconf_pvar: &fCuvar_echocheck as *const boolean as *mut boolean
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"echonl\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_echonl as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"timeout\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cCuvar_timeout as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"kill\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_kill as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"resend\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x22 as libc::c_int,
                uuconf_pvar: &cCuvar_resend as *const libc::c_int as *mut libc::c_int
                    as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"eofwrite\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_eofwrite as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"eofread\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zCuvar_eofread as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"verbose\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x12 as libc::c_int,
                uuconf_pvar: &fCuvar_verbose as *const boolean as *mut boolean
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
static mut abCuconnected: [libc::c_char; 13] = unsafe {
    *::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"\x07[connected]\0")
};
static mut pCuuuconf: pointer = 0 as *const libc::c_void as *mut libc::c_void;
static mut qCuconn: *mut sconnection = 0 as *const sconnection as *mut sconnection;
static mut fCuclose_conn: boolean = 0;
static mut qCudialer: *mut uuconf_dialer = 0 as *const uuconf_dialer
    as *mut uuconf_dialer;
static mut fCurestore_terminal: boolean = 0;
static mut fCulocalecho: boolean = 0;
static mut fCustarted: boolean = 0;
static mut fCuconnprinted: boolean = 0 as libc::c_int;
static mut asCulongopts: [option; 17] = [
    {
        let mut init = option {
            name: b"phone\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"escape\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"parity\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"halfduplex\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"line\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
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
            name: b"speed\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"baud\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mapcr\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"nostop\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"system\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
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
    let mut zphone: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feven: boolean = 0 as libc::c_int;
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fprompt: boolean = 0 as libc::c_int;
    let mut fodd: boolean = 0 as libc::c_int;
    let mut zport: *const libc::c_char = 0 as *const libc::c_char;
    let mut ibaud: libc::c_long = 0 as libc::c_long;
    let mut fmapcr: boolean = 0 as libc::c_int;
    let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut txonxoff: txonxoffsetting = XONXOFF_ON;
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut zlocalname: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ssys: uuconf_system = uuconf_system {
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
    let mut qsys: *const uuconf_system = 0 as *const uuconf_system;
    let mut flooped: boolean = 0;
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
    let mut sinfo: sconninfo = sconninfo {
        fmatched: 0,
        flocked: 0,
        fdirect: 0,
        qconn: 0 as *mut sconnection,
        zline: 0 as *const libc::c_char,
    };
    let mut ihighbaud: libc::c_long = 0;
    let mut sdialer: uuconf_dialer = uuconf_dialer {
        uuconf_zname: 0 as *mut libc::c_char,
        uuconf_schat: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_zdialtone: 0 as *mut libc::c_char,
        uuconf_zpause: 0 as *mut libc::c_char,
        uuconf_fcarrier: 0,
        uuconf_ccarrier_wait: 0,
        uuconf_fdtr_toggle: 0,
        uuconf_fdtr_toggle_wait: 0,
        uuconf_scomplete: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_sabort: uuconf_chat {
            uuconf_pzchat: 0 as *mut *mut libc::c_char,
            uuconf_pzprogram: 0 as *mut *mut libc::c_char,
            uuconf_ctimeout: 0,
            uuconf_pzfail: 0 as *mut *mut libc::c_char,
            uuconf_fstrip: 0,
        },
        uuconf_qproto_params: 0 as *mut uuconf_proto_param,
        uuconf_ireliable: 0,
        uuconf_palloc: 0 as *mut libc::c_void,
    };
    let mut qdialer: *mut uuconf_dialer = 0 as *mut uuconf_dialer;
    let mut bcmd: libc::c_char = 0;
    zProgram = *argv.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut clen: size_t = 0;
            let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
            clen = strlen(*argv.offset(i as isize));
            z = zbufalc(clen.wrapping_add(2 as libc::c_int as libc::c_ulong));
            *z.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
            *z.offset(1 as libc::c_int as isize) = 's' as i32 as libc::c_char;
            memcpy(
                z.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                (*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                clen,
            );
            let ref mut fresh0 = *argv.offset(i as isize);
            *fresh0 = z;
        }
        i += 1;
        i;
    }
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"a:c:deE:hnI:l:op:s:tvx:z:\0" as *const u8 as *const libc::c_char,
            asCulongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            99 => {
                zphone = gnu_optarg;
            }
            100 => {
                iDebug = 0o3777 as libc::c_int;
            }
            101 => {
                feven = 1 as libc::c_int;
            }
            69 => {
                zCuvar_escape = gnu_optarg;
            }
            104 => {
                fCulocalecho = 1 as libc::c_int;
            }
            110 => {
                fprompt = 1 as libc::c_int;
            }
            108 => {
                zline = gnu_optarg;
            }
            111 => {
                fodd = 1 as libc::c_int;
            }
            112 | 97 => {
                zport = gnu_optarg;
            }
            115 => {
                ibaud = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
            }
            116 => {
                fmapcr = 1 as libc::c_int;
            }
            122 => {
                zsystem = gnu_optarg;
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
            }
            120 => {
                iDebug |= idebug_parse(gnu_optarg);
            }
            118 => {
                printf(
                    b"cu (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
                    b"1.07\0" as *const u8 as *const libc::c_char,
                );
                printf(
                    b"Copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
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
            2 => {
                if strncmp(
                    gnu_optarg,
                    b"even\0" as *const u8 as *const libc::c_char,
                    strlen(gnu_optarg),
                ) == 0 as libc::c_int
                {
                    feven = 1 as libc::c_int;
                } else if strncmp(
                    gnu_optarg,
                    b"odd\0" as *const u8 as *const libc::c_char,
                    strlen(gnu_optarg),
                ) == 0 as libc::c_int
                {
                    fodd = 1 as libc::c_int;
                } else if strncmp(
                    gnu_optarg,
                    b"none\0" as *const u8 as *const libc::c_char,
                    strlen(gnu_optarg),
                ) == 0 as libc::c_int
                {
                    feven = 1 as libc::c_int;
                    fodd = 1 as libc::c_int;
                } else {
                    fprintf(
                        stderr,
                        b"%s: --parity requires even, odd or none\n\0" as *const u8
                            as *const libc::c_char,
                        zProgram,
                    );
                    ucuusage();
                }
            }
            3 => {
                txonxoff = XONXOFF_OFF;
            }
            1 => {
                ucuhelp();
                exit(0 as libc::c_int);
            }
            0 => {}
            _ => {
                ucuusage();
            }
        }
    }
    if gnu_optind != argc {
        if gnu_optind != argc - 1 as libc::c_int || !zsystem.is_null()
            || !zphone.is_null()
        {
            fprintf(
                stderr,
                b"%s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                zProgram,
            );
            ucuusage();
        }
        if strcmp(
            *argv.offset(gnu_optind as isize),
            b"dir\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            if *(*__ctype_b_loc())
                .offset(
                    *(*argv.offset(gnu_optind as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                zphone = *argv.offset(gnu_optind as isize);
            } else {
                zsystem = *argv.offset(gnu_optind as isize);
            }
        }
    }
    if zsystem.is_null() && zport.is_null() && zline.is_null()
        && ibaud == 0 as libc::c_long
    {
        fprintf(
            stderr,
            b"%s: must specify system, line, port or speed\n\0" as *const u8
                as *const libc::c_char,
            zProgram,
        );
        ucuusage();
    }
    if fprompt != 0 {
        let mut cphone: size_t = 0;
        printf(b"Phone number: \0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        zphone = 0 as *mut libc::c_char;
        cphone = 0 as libc::c_int as size_t;
        if getline(&mut zphone, &mut cphone, stdin) <= 0 as libc::c_int as libc::c_long
            || *zphone as libc::c_int == '\0' as i32
        {
            fprintf(
                stderr,
                b"%s: no phone number entered\n\0" as *const u8 as *const libc::c_char,
                zProgram,
            );
            exit(1 as libc::c_int);
        }
    }
    iuuconf = uuconf_init(
        &mut puuconf,
        b"cu\0" as *const u8 as *const libc::c_char,
        zconfig,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    pCuuuconf = puuconf;
    let mut zdebug: *const libc::c_char = 0 as *const libc::c_char;
    iuuconf = uuconf_debuglevel(puuconf, &mut zdebug);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    if !zdebug.is_null() {
        iDebug |= idebug_parse(zdebug);
    }
    usysdep_initialize(puuconf, 0o2 as libc::c_int | 0o4 as libc::c_int);
    iuuconf = uuconf_localname(puuconf, &mut zlocalname);
    if iuuconf == 1 as libc::c_int {
        zlocalname = zsysdep_localname();
        if zlocalname.is_null() {
            exit(1 as libc::c_int);
        }
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    ulog_fatal_fn(Some(ucuabort as unsafe extern "C" fn() -> ()));
    pfLstart = Some(uculog_start as unsafe extern "C" fn() -> ());
    pfLend = Some(uculog_end as unsafe extern "C" fn() -> ());
    usysdep_signal(2 as libc::c_int);
    usysdep_signal(1 as libc::c_int);
    usysdep_signal(3 as libc::c_int);
    usysdep_signal(15 as libc::c_int);
    usysdep_signal(13 as libc::c_int);
    if !zsystem.is_null() {
        iuuconf = uuconf_system_info(puuconf, zsystem, &mut ssys);
        if iuuconf != 0 as libc::c_int {
            if iuuconf != 1 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            ulog(
                LOG_FATAL,
                b"%s: System not found\0" as *const u8 as *const libc::c_char,
                zsystem,
            );
        }
        qsys = &mut ssys;
    }
    flooped = 0 as libc::c_int;
    loop {
        let mut tparity: tparitysetting = PARITYSETTING_DEFAULT;
        let mut tstrip: tstripsetting = STRIPSETTING_DEFAULT;
        let mut iusebaud: libc::c_long = 0;
        sinfo.fmatched = 0 as libc::c_int;
        sinfo.flocked = 0 as libc::c_int;
        sinfo.fdirect = (qsys.is_null() && zphone.is_null()) as libc::c_int;
        sinfo.qconn = &mut sconn;
        sinfo.zline = zline;
        if !zport.is_null() || !zline.is_null() || ibaud != 0 as libc::c_long {
            iuuconf = uuconf_find_port(
                puuconf,
                zport,
                ibaud,
                0 as libc::c_long,
                Some(
                    icuport_lock
                        as unsafe extern "C" fn(*mut uuconf_port, pointer) -> libc::c_int,
                ),
                &mut sinfo as *mut sconninfo as pointer,
                &mut sport,
            );
            if iuuconf != 0 as libc::c_int {
                if iuuconf != 1 as libc::c_int {
                    if sinfo.flocked != 0 {
                        fconn_unlock(&mut sconn);
                        uconn_free(&mut sconn);
                    }
                    ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
                }
                if zline.is_null() || !zport.is_null() || !zphone.is_null()
                    || !qsys.is_null()
                {
                    if sinfo.fmatched != 0 {
                        ulog(
                            LOG_FATAL,
                            b"All matching ports in use\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        ulog(
                            LOG_FATAL,
                            b"No matching ports\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                sport.uuconf_zname = zline;
                sport.uuconf_ttype = UUCONF_PORTTYPE_DIRECT;
                sport.uuconf_zprotocols = 0 as *mut libc::c_char;
                sport.uuconf_qproto_params = 0 as *mut uuconf_proto_param;
                sport.uuconf_ireliable = 0 as libc::c_int;
                sport.uuconf_zlockname = 0 as *mut libc::c_char;
                sport.uuconf_palloc = 0 as *mut libc::c_void;
                sport.uuconf_u.uuconf_sdirect.uuconf_zdevice = 0 as *mut libc::c_char;
                sport.uuconf_u.uuconf_sdirect.uuconf_ibaud = ibaud;
                if fconn_init(&mut sport, &mut sconn, UUCONF_PORTTYPE_UNKNOWN) == 0 {
                    ucuabort();
                }
                if fconn_lock(&mut sconn, 0 as libc::c_int, 1 as libc::c_int) == 0 {
                    ulog(
                        LOG_FATAL,
                        b"%s: Line in use\0" as *const u8 as *const libc::c_char,
                        zline,
                    );
                }
                qCuconn = &mut sconn;
                if fsysdep_port_access(&mut sport) == 0 {
                    ulog(
                        LOG_FATAL,
                        b"%s: Permission denied\0" as *const u8 as *const libc::c_char,
                        zline,
                    );
                }
            }
            iusebaud = ibaud;
            ihighbaud = 0 as libc::c_long;
        } else {
            while !qsys.is_null() {
                if !((*qsys).uuconf_fcall == 0) {
                    if !((*qsys).uuconf_qport).is_null() {
                        if fconn_init(
                            (*qsys).uuconf_qport,
                            &mut sconn,
                            UUCONF_PORTTYPE_UNKNOWN,
                        ) != 0
                        {
                            if fconn_lock(&mut sconn, 0 as libc::c_int, 0 as libc::c_int)
                                != 0
                            {
                                qCuconn = &mut sconn;
                                break;
                            } else {
                                uconn_free(&mut sconn);
                            }
                        }
                    } else {
                        sinfo.fmatched = 0 as libc::c_int;
                        sinfo.flocked = 0 as libc::c_int;
                        sinfo.qconn = &mut sconn;
                        iuuconf = uuconf_find_port(
                            puuconf,
                            (*qsys).uuconf_zport,
                            (*qsys).uuconf_ibaud,
                            (*qsys).uuconf_ihighbaud,
                            Some(
                                icuport_lock
                                    as unsafe extern "C" fn(
                                        *mut uuconf_port,
                                        pointer,
                                    ) -> libc::c_int,
                            ),
                            &mut sinfo as *mut sconninfo as pointer,
                            &mut sport,
                        );
                        if iuuconf == 0 as libc::c_int {
                            break;
                        }
                        if iuuconf != 1 as libc::c_int {
                            if sinfo.flocked != 0 {
                                fconn_unlock(&mut sconn);
                                uconn_free(&mut sconn);
                            }
                            ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
                        }
                    }
                }
                qsys = (*qsys).uuconf_qalternate;
            }
            if qsys.is_null() {
                let mut zrem: *const libc::c_char = 0 as *const libc::c_char;
                if flooped != 0 {
                    zrem = b"remaining \0" as *const u8 as *const libc::c_char;
                } else {
                    zrem = b"\0" as *const u8 as *const libc::c_char;
                }
                if sinfo.fmatched != 0 {
                    ulog(
                        LOG_FATAL,
                        b"%s: All %smatching ports in use\0" as *const u8
                            as *const libc::c_char,
                        zsystem,
                        zrem,
                    );
                } else {
                    ulog(
                        LOG_FATAL,
                        b"%s: No %smatching ports\0" as *const u8 as *const libc::c_char,
                        zsystem,
                        zrem,
                    );
                }
            }
            iusebaud = (*qsys).uuconf_ibaud;
            ihighbaud = (*qsys).uuconf_ihighbaud;
        }
        if fconn_open(&mut sconn, iusebaud, ihighbaud, 0 as libc::c_int, sinfo.fdirect)
            == 0
        {
            ucuabort();
        }
        fCuclose_conn = 1 as libc::c_int;
        if afSignal[0 as libc::c_int as usize] != 0
            || afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0
        {
            ucuabort();
        }
        if fodd != 0 && feven != 0 {
            tparity = PARITYSETTING_NONE;
            tstrip = STRIPSETTING_SEVENBITS;
        } else if fodd != 0 {
            tparity = PARITYSETTING_ODD;
            tstrip = STRIPSETTING_SEVENBITS;
        } else if feven != 0 {
            tparity = PARITYSETTING_EVEN;
            tstrip = STRIPSETTING_SEVENBITS;
        } else {
            tparity = PARITYSETTING_DEFAULT;
            tstrip = STRIPSETTING_DEFAULT;
        }
        if fconn_set(&mut sconn, tparity, tstrip, txonxoff) == 0 {
            ucuabort();
        }
        if !qsys.is_null() {
            zphone = (*qsys).uuconf_zphone;
        }
        if !qsys.is_null() || !zphone.is_null() {
            let mut tdialer: tdialerfound = DIALERFOUND_FALSE;
            if fconn_dial(&mut sconn, puuconf, qsys, zphone, &mut sdialer, &mut tdialer)
                == 0
            {
                if !zport.is_null() || !zline.is_null() || ibaud != 0 as libc::c_long
                    || qsys.is_null()
                {
                    ucuabort();
                }
                qsys = (*qsys).uuconf_qalternate;
                if qsys.is_null() {
                    ulog(
                        LOG_FATAL,
                        b"%s: No remaining alternates\0" as *const u8
                            as *const libc::c_char,
                        zsystem,
                    );
                }
                fCuclose_conn = 0 as libc::c_int;
                fconn_close(&mut sconn, pCuuuconf, qCudialer, 0 as libc::c_int);
                qCuconn = 0 as *mut sconnection;
                fconn_unlock(&mut sconn);
                uconn_free(&mut sconn);
                flooped = 1 as libc::c_int;
            } else {
                if tdialer as libc::c_uint
                    == DIALERFOUND_FALSE as libc::c_int as libc::c_uint
                {
                    qdialer = 0 as *mut uuconf_dialer;
                } else {
                    qdialer = &mut sdialer;
                }
                break;
            }
        } else {
            if fsysdep_port_access(sconn.qport) == 0 {
                ulog(
                    LOG_FATAL,
                    b"Access to port denied\0" as *const u8 as *const libc::c_char,
                );
            }
            qdialer = 0 as *mut uuconf_dialer;
            if fconn_carrier(&mut sconn, 0 as libc::c_int) == 0 {
                ulog(
                    LOG_FATAL,
                    b"Can't turn off carrier\0" as *const u8 as *const libc::c_char,
                );
            }
            break;
        }
    }
    qCudialer = qdialer;
    if afSignal[0 as libc::c_int as usize] != 0
        || afSignal[1 as libc::c_int as usize] != 0
        || afSignal[2 as libc::c_int as usize] != 0
        || afSignal[3 as libc::c_int as usize] != 0
        || afSignal[4 as libc::c_int as usize] != 0
    {
        ucuabort();
    }
    printf(
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"\x07Connected.\0" as *const u8 as *const libc::c_char,
    );
    fCuconnprinted = 1 as libc::c_int;
    if fsysdep_terminal_raw(fCulocalecho) == 0 {
        ucuabort();
    }
    fCurestore_terminal = 1 as libc::c_int;
    if fsysdep_cu_init(&mut sconn) == 0 {
        ucuabort();
    }
    fCustarted = 1 as libc::c_int;
    while fsysdep_cu(&mut sconn, &mut bcmd, zlocalname) != 0 {
        if fcudo_cmd(puuconf, &mut sconn, bcmd as libc::c_int) == 0 {
            break;
        }
    }
    fCustarted = 0 as libc::c_int;
    if fsysdep_cu_finish() == 0 {
        ucuabort();
    }
    fCurestore_terminal = 0 as libc::c_int;
    fsysdep_terminal_restore();
    fconn_close(&mut sconn, puuconf, qdialer, 1 as libc::c_int);
    fconn_unlock(&mut sconn);
    uconn_free(&mut sconn);
    if fCuconnprinted != 0 {
        printf(
            b"\n%s\n\0" as *const u8 as *const libc::c_char,
            b"\x07Disconnected.\0" as *const u8 as *const libc::c_char,
        );
    }
    ulog_close();
    usysdep_exit(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ucuusage() {
    fprintf(
        stderr,
        b"Usage: %s [options] [system or phone-number]\n\0" as *const u8
            as *const libc::c_char,
        zProgram,
    );
    fprintf(
        stderr,
        b"Use %s --help for help\n\0" as *const u8 as *const libc::c_char,
        zProgram,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn ucuhelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Usage: %s [options] [system or phone-number]\n\0" as *const u8
            as *const libc::c_char,
        zProgram,
    );
    printf(
        b" -a,-p,--port port: Use named port\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -l,--line line: Use named device (e.g. tty0)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -s,--speed,--baud speed, -#: Use given speed\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -c,--phone phone: Phone number to call\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -z,--system system: System to call\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b" -e: Set even parity\n\0" as *const u8 as *const libc::c_char);
    printf(b" -o: Set odd parity\n\0" as *const u8 as *const libc::c_char);
    printf(b" --parity={odd,even}: Set parity\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -E,--escape char: Set escape character\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b" -h,--halfduplex: Echo locally\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" --nostop: Turn off XON/XOFF handling\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -t,--mapcr: Map carriage return to carriage return/linefeed\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -n,--prompt: Prompt for phone number\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b" -d: Set maximum debugging level\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -x,--debug debug: Set debugging type\n\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn ucuabort() {
    if fCustarted != 0 {
        fCustarted = 0 as libc::c_int;
        fsysdep_cu_finish();
    }
    if fCurestore_terminal != 0 {
        fCurestore_terminal = 0 as libc::c_int;
        fsysdep_terminal_restore();
    }
    if !qCuconn.is_null() {
        let mut qconn: *mut sconnection = 0 as *mut sconnection;
        if fCuclose_conn != 0 {
            fCuclose_conn = 0 as libc::c_int;
            fconn_close(qCuconn, pCuuuconf, qCudialer, 0 as libc::c_int);
        }
        qconn = qCuconn;
        qCuconn = 0 as *mut sconnection;
        fconn_unlock(qconn);
        uconn_free(qconn);
    }
    ulog_close();
    if fCuconnprinted != 0 {
        printf(
            b"\n%s\n\0" as *const u8 as *const libc::c_char,
            b"\x07Disconnected.\0" as *const u8 as *const libc::c_char,
        );
    }
    usysdep_exit(0 as libc::c_int);
}
static mut fCulog_restore: boolean = 0;
unsafe extern "C" fn uculog_start() {
    if fCurestore_terminal == 0 {
        fCulog_restore = 0 as libc::c_int;
    } else {
        fCulog_restore = 1 as libc::c_int;
        fCurestore_terminal = 0 as libc::c_int;
        if fsysdep_terminal_restore() == 0 {
            ucuabort();
        }
    };
}
unsafe extern "C" fn uculog_end() {
    if fCulog_restore != 0 {
        if fsysdep_terminal_raw(fCulocalecho) == 0 {
            ucuabort();
        }
        fCurestore_terminal = 1 as libc::c_int;
    }
}
unsafe extern "C" fn icuport_lock(
    mut qport: *mut uuconf_port,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut q: *mut sconninfo = pinfo as *mut sconninfo;
    if !((*q).zline).is_null() && fsysdep_port_is_line(qport, (*q).zline) == 0 {
        return 1 as libc::c_int;
    }
    (*q).fmatched = 1 as libc::c_int;
    if fconn_init(qport, (*q).qconn, UUCONF_PORTTYPE_UNKNOWN) == 0 {
        return 1 as libc::c_int
    } else if fconn_lock((*q).qconn, 0 as libc::c_int, (*q).fdirect) == 0 {
        uconn_free((*q).qconn);
        return 1 as libc::c_int;
    } else {
        qCuconn = (*q).qconn;
        (*q).flocked = 1 as libc::c_int;
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn fcudo_cmd(
    mut puuconf: pointer,
    mut qconn: *mut sconnection,
    mut bcmd: libc::c_int,
) -> boolean {
    let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abescape: [libc::c_char; 5] = [0; 5];
    let mut fret: boolean = 0;
    let mut clen: size_t = 0;
    let mut abbuf: [libc::c_char; 100] = [0; 100];
    match bcmd {
        33 | 36 | 124 | 43 | 37 | 99 | 62 | 60 | 112 | 116 | 115 => {
            zline = zsysdep_terminal_line(0 as *mut libc::c_void as *const libc::c_char);
            if zline.is_null() {
                ucuabort();
            }
            *zline
                .offset(
                    strcspn(zline, b"\n\0" as *const u8 as *const libc::c_char) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        _ => {
            zline = 0 as *mut libc::c_char;
        }
    }
    match bcmd {
        46 => return 0 as libc::c_int,
        33 | 36 | 124 | 43 => {
            if fsysdep_cu_copy(0 as libc::c_int) == 0 || fsysdep_terminal_restore() == 0
            {
                ucuabort();
            }
            fCurestore_terminal = 0 as libc::c_int;
            let mut t: tshell_cmd = SHELL_NORMAL;
            match bcmd {
                36 => {
                    t = SHELL_STDOUT_TO_PORT;
                }
                124 => {
                    t = SHELL_STDIN_FROM_PORT;
                }
                43 => {
                    t = SHELL_STDIO_ON_PORT;
                }
                33 | _ => {
                    t = SHELL_NORMAL;
                }
            }
            fsysdep_shell(qconn, zline, t);
            if fsysdep_cu_copy(1 as libc::c_int) == 0
                || fsysdep_terminal_raw(fCulocalecho) == 0
            {
                ucuabort();
            }
            fCurestore_terminal = 1 as libc::c_int;
            ubuffree(zline);
            return 1 as libc::c_int;
        }
        37 => {
            fret = fcudo_subcmd(puuconf, qconn, zline);
            ubuffree(zline);
            return fret;
        }
        35 => {
            if fconn_break(qconn) == 0 {
                ucuabort();
            }
            return 1 as libc::c_int;
        }
        99 => {
            fsysdep_chdir(zline);
            ubuffree(zline);
            return 1 as libc::c_int;
        }
        62 | 60 | 112 | 116 => {
            clen = strlen(zline);
            z = zbufalc(clen.wrapping_add(3 as libc::c_int as libc::c_ulong));
            *z.offset(0 as libc::c_int as isize) = bcmd as libc::c_char;
            *z.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
            memcpy(
                z.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                zline as *const libc::c_void,
                clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            ubuffree(zline);
            fret = fcudo_subcmd(puuconf, qconn, z);
            ubuffree(z);
            return fret;
        }
        122 => {
            if fsysdep_cu_copy(0 as libc::c_int) == 0 || fsysdep_terminal_restore() == 0
            {
                ucuabort();
            }
            fCurestore_terminal = 0 as libc::c_int;
            if fsysdep_suspend() == 0 {
                ucuabort();
            }
            if fsysdep_cu_copy(1 as libc::c_int) == 0
                || fsysdep_terminal_raw(fCulocalecho) == 0
            {
                ucuabort();
            }
            fCurestore_terminal = 1 as libc::c_int;
            return 1 as libc::c_int;
        }
        115 => {
            fret = fcuset_var(puuconf, zline);
            ubuffree(zline);
            return fret;
        }
        118 => {
            uculist_vars();
            return 1 as libc::c_int;
        }
        63 => {
            if *(*__ctype_b_loc()).offset(*zCuvar_escape as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                sprintf(
                    abescape.as_mut_ptr(),
                    b"\\%03o\0" as *const u8 as *const libc::c_char,
                    *zCuvar_escape as libc::c_uchar as libc::c_int,
                );
            } else {
                abescape[0 as libc::c_int as usize] = *zCuvar_escape;
                abescape[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            }
            if fsysdep_terminal_puts(b"\0" as *const u8 as *const libc::c_char) == 0 {
                ucuabort();
            }
            if fsysdep_terminal_puts(
                b"[Escape sequences]\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%s. hangup]                   [%s!CMD run shell]\0" as *const u8
                    as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%s$CMD stdout to remote]      [%s|CMD stdin from remote]\0"
                    as *const u8 as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%s+CMD stdin and stdout to remote]\0" as *const u8
                    as *const libc::c_char,
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%s# send break]               [%scDIR change directory]\0"
                    as *const u8 as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%s> send file]                [%s< receive file]\0" as *const u8
                    as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%spFROM TO send to Unix]      [%stFROM TO receive from Unix]\0"
                    as *const u8 as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%ssVAR VAL set variable]      [%ssVAR set boolean]\0" as *const u8
                    as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%ss!VAR unset boolean]        [%sv list variables]\0" as *const u8
                    as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[%sz suspend]\0" as *const u8 as *const libc::c_char,
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            uculist_fns(abescape.as_mut_ptr());
            return 1 as libc::c_int;
        }
        _ => {
            if *(*__ctype_b_loc()).offset(*zCuvar_escape as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                sprintf(
                    abescape.as_mut_ptr(),
                    b"\\%03o\0" as *const u8 as *const libc::c_char,
                    *zCuvar_escape as libc::c_uchar as libc::c_int,
                );
            } else {
                abescape[0 as libc::c_int as usize] = *zCuvar_escape;
                abescape[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            }
            sprintf(
                abbuf.as_mut_ptr(),
                b"[Unrecognized.  Use %s%s to send %s]\0" as *const u8
                    as *const libc::c_char,
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
                abescape.as_mut_ptr(),
            );
            if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
                ucuabort();
            }
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn uculist_fns(mut zescape: *const libc::c_char) {
    let mut abbuf: [libc::c_char; 100] = [0; 100];
    sprintf(
        abbuf.as_mut_ptr(),
        b"[%s%%break send break]         [%s%%cd DIR change directory]\0" as *const u8
            as *const libc::c_char,
        zescape,
        zescape,
    );
    if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
        ucuabort();
    }
    sprintf(
        abbuf.as_mut_ptr(),
        b"[%s%%put FROM TO send file]    [%s%%take FROM TO receive file]\0" as *const u8
            as *const libc::c_char,
        zescape,
        zescape,
    );
    if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
        ucuabort();
    }
    sprintf(
        abbuf.as_mut_ptr(),
        b"[%s%%nostop no XON/XOFF]       [%s%%stop use XON/XOFF]\0" as *const u8
            as *const libc::c_char,
        zescape,
        zescape,
    );
    if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
        ucuabort();
    }
}
unsafe extern "C" fn fcuset_var(
    mut puuconf: pointer,
    mut zline: *mut libc::c_char,
) -> boolean {
    let mut zvar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut azargs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut iuuconf: libc::c_int = 0;
    zvar = strtok(zline, b"= \t\0" as *const u8 as *const libc::c_char);
    if zvar.is_null() {
        if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
            ucuabort();
        }
        return 1 as libc::c_int;
    }
    zval = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\0" as *const u8 as *const libc::c_char,
    );
    if zval.is_null() {
        azargs[0 as libc::c_int as usize] = zvar;
        if *(azargs[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
            as libc::c_int != '!' as i32
        {
            azargs[1 as libc::c_int
                as usize] = zbufcpy(b"t\0" as *const u8 as *const libc::c_char);
        } else {
            azargs[0 as libc::c_int
                as usize] = (azargs[0 as libc::c_int as usize]).offset(1);
            azargs[0 as libc::c_int as usize];
            azargs[1 as libc::c_int
                as usize] = zbufcpy(b"f\0" as *const u8 as *const libc::c_char);
        }
    } else {
        azargs[0 as libc::c_int as usize] = zvar;
        azargs[1 as libc::c_int as usize] = zbufcpy(zval);
    }
    iuuconf = uuconf_cmd_args(
        puuconf,
        2 as libc::c_int,
        azargs.as_mut_ptr(),
        asCuvars.as_ptr(),
        0 as *mut libc::c_void,
        Some(
            icuunrecogvar
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if iuuconf & 0x800 as libc::c_int == 0 as libc::c_int {
        ubuffree(azargs[1 as libc::c_int as usize]);
    }
    if iuuconf & !(0x800 as libc::c_int) != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn icuunrecogvar(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut abescape: [libc::c_char; 5] = [0; 5];
    if *(*__ctype_b_loc()).offset(*zCuvar_escape as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        sprintf(
            abescape.as_mut_ptr(),
            b"\\%03o\0" as *const u8 as *const libc::c_char,
            *zCuvar_escape as libc::c_uchar as libc::c_int,
        );
    } else {
        abescape[0 as libc::c_int as usize] = *zCuvar_escape;
        abescape[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    ulog(
        LOG_ERROR,
        b"%s: unknown variable (%sv lists variables)\0" as *const u8
            as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
        abescape.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn uculist_vars() {
    let mut q: *const uuconf_cmdtab = 0 as *const uuconf_cmdtab;
    let mut abbuf: [libc::c_char; 100] = [0; 100];
    if fsysdep_terminal_puts(b"\0" as *const u8 as *const libc::c_char) == 0 {
        ucuabort();
    }
    q = asCuvars.as_ptr();
    while !((*q).uuconf_zcmd).is_null() {
        match (*q).uuconf_itype & 0x70 as libc::c_int {
            16 => {
                if *((*q).uuconf_pvar as *mut boolean) != 0 {
                    sprintf(
                        abbuf.as_mut_ptr(),
                        b"%s true\0" as *const u8 as *const libc::c_char,
                        (*q).uuconf_zcmd,
                    );
                } else {
                    sprintf(
                        abbuf.as_mut_ptr(),
                        b"%s false\0" as *const u8 as *const libc::c_char,
                        (*q).uuconf_zcmd,
                    );
                }
            }
            32 => {
                sprintf(
                    abbuf.as_mut_ptr(),
                    b"%s %d\0" as *const u8 as *const libc::c_char,
                    (*q).uuconf_zcmd,
                    *((*q).uuconf_pvar as *mut libc::c_int),
                );
            }
            48 => {
                sprintf(
                    abbuf.as_mut_ptr(),
                    b"%s %ld\0" as *const u8 as *const libc::c_char,
                    (*q).uuconf_zcmd,
                    *((*q).uuconf_pvar as *mut libc::c_long),
                );
            }
            64 | 80 => {
                let mut z: *const libc::c_char = 0 as *const libc::c_char;
                let mut abchar: [libc::c_char; 5] = [0; 5];
                let mut clen: size_t = 0;
                sprintf(
                    abbuf.as_mut_ptr(),
                    b"%s \0" as *const u8 as *const libc::c_char,
                    (*q).uuconf_zcmd,
                );
                clen = strlen(abbuf.as_mut_ptr());
                z = *((*q).uuconf_pvar as *mut *const libc::c_char);
                while *z as libc::c_int != '\0' as i32 {
                    let mut cchar: libc::c_int = 0;
                    if *(*__ctype_b_loc()).offset(*z as libc::c_int as isize)
                        as libc::c_int
                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        sprintf(
                            abchar.as_mut_ptr(),
                            b"\\%03o\0" as *const u8 as *const libc::c_char,
                            *z as libc::c_uchar as libc::c_int,
                        );
                        cchar = 4 as libc::c_int;
                    } else {
                        abchar[0 as libc::c_int as usize] = *z;
                        abchar[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        cchar = 1 as libc::c_int;
                    }
                    if clen.wrapping_add(cchar as libc::c_ulong)
                        < ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                    {
                        strcat(abbuf.as_mut_ptr(), abchar.as_mut_ptr());
                    }
                    clen = (clen as libc::c_ulong).wrapping_add(cchar as libc::c_ulong)
                        as size_t as size_t;
                    z = z.offset(1);
                    z;
                }
            }
            _ => {
                sprintf(
                    abbuf.as_mut_ptr(),
                    b"%s [unprintable type]\0" as *const u8 as *const libc::c_char,
                    (*q).uuconf_zcmd,
                );
            }
        }
        if fsysdep_terminal_puts(abbuf.as_mut_ptr()) == 0 {
            ucuabort();
        }
        q = q.offset(1);
        q;
    }
}
static mut bCutype: libc::c_char = 0;
static mut asCucmds: [uuconf_cmdtab; 13] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"break\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 1 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icubreak
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"b\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 1 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icubreak
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"cd\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icuchdir
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"d\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 1 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icudebug
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"put\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icuput
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"take\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icutake
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"nostop\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 1 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icunostop
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"stop\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 1 as libc::c_int,
                uuconf_pvar: &bCutype as *const libc::c_char as *mut libc::c_char
                    as UUCONF_POINTER,
                uuconf_pifn: Some(
                    icunostop
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b">\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &bCutype as *const libc::c_char as *mut libc::c_char
                    as UUCONF_POINTER,
                uuconf_pifn: Some(
                    icuput
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"<\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: &bCutype as *const libc::c_char as *mut libc::c_char
                    as UUCONF_POINTER,
                uuconf_pifn: Some(
                    icutake
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"p\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icuput
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"t\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    icutake
                        as unsafe extern "C" fn(
                            pointer,
                            libc::c_int,
                            *mut *mut libc::c_char,
                            pointer,
                            pointer,
                        ) -> libc::c_int,
                ),
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
unsafe extern "C" fn fcudo_subcmd(
    mut puuconf: pointer,
    mut qconn: *mut sconnection,
    mut zline: *mut libc::c_char,
) -> boolean {
    let mut azargs: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut iarg: libc::c_int = 0;
    let mut iuuconf: libc::c_int = 0;
    iarg = 0 as libc::c_int;
    while iarg < 3 as libc::c_int {
        azargs[iarg
            as usize] = strtok(
            if iarg == 0 as libc::c_int {
                zline
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            },
            b" \t\n\0" as *const u8 as *const libc::c_char,
        );
        if (azargs[iarg as usize]).is_null() {
            break;
        }
        iarg += 1;
        iarg;
    }
    if iarg == 0 as libc::c_int {
        if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
            ucuabort();
        }
        return 1 as libc::c_int;
    }
    iuuconf = uuconf_cmd_args(
        puuconf,
        iarg,
        azargs.as_mut_ptr(),
        asCucmds.as_ptr(),
        qconn as pointer,
        Some(
            icuunrecogfn
                as unsafe extern "C" fn(
                    pointer,
                    libc::c_int,
                    *mut *mut libc::c_char,
                    pointer,
                    pointer,
                ) -> libc::c_int,
        ),
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn icuunrecogfn(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut abescape: [libc::c_char; 5] = [0; 5];
    if *(*__ctype_b_loc()).offset(*zCuvar_escape as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        sprintf(
            abescape.as_mut_ptr(),
            b"\\%03o\0" as *const u8 as *const libc::c_char,
            *zCuvar_escape as libc::c_uchar as libc::c_int,
        );
    } else {
        abescape[0 as libc::c_int as usize] = *zCuvar_escape;
        abescape[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int == '?' as i32
    {
        uculist_fns(abescape.as_mut_ptr());
    } else {
        ulog(
            LOG_ERROR,
            b"%s: unknown (%s%%? lists choices)\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            abescape.as_mut_ptr(),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icubreak(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qconn: *mut sconnection = pinfo as *mut sconnection;
    if fconn_break(qconn) == 0 {
        ucuabort();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icuchdir(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut zarg: *const libc::c_char = 0 as *const libc::c_char;
    if argc <= 1 as libc::c_int {
        zarg = 0 as *const libc::c_char;
    } else {
        zarg = *argv.offset(1 as libc::c_int as isize);
    }
    fsysdep_chdir(zarg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn icudebug(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    if iDebug != 0 as libc::c_int {
        iDebug = 0 as libc::c_int;
    } else {
        iDebug = 0o3777 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icunostop(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qconn: *mut sconnection = pinfo as *mut sconnection;
    if fconn_set(
        qconn,
        PARITYSETTING_DEFAULT,
        STRIPSETTING_DEFAULT,
        (if pvar.is_null() {
            XONXOFF_OFF as libc::c_int
        } else {
            XONXOFF_ON as libc::c_int
        }) as txonxoffsetting,
    ) == 0
    {
        ucuabort();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icuput(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qconn: *mut sconnection = pinfo as *mut sconnection;
    let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: openfile_t = 0 as *mut FILE;
    let mut cline: libc::c_int = 0;
    let mut zbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cbuf: size_t = 0;
    if argc > 1 as libc::c_int {
        zfrom = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    } else {
        zfrom = zsysdep_terminal_line(
            b"File to send: \0" as *const u8 as *const libc::c_char,
        );
        if zfrom.is_null() {
            ucuabort();
        }
        *zfrom
            .offset(
                strcspn(zfrom, b" \t\n\0" as *const u8 as *const libc::c_char) as isize,
            ) = '\0' as i32 as libc::c_char;
        if *zfrom as libc::c_int == '\0' as i32 {
            ubuffree(zfrom);
            if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
                ucuabort();
            }
            return 0 as libc::c_int;
        }
    }
    if pvar.is_null() {
        if argc > 2 as libc::c_int {
            zto = zbufcpy(*argv.offset(2 as libc::c_int as isize));
        } else {
            let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zprompt: *mut libc::c_char = 0 as *mut libc::c_char;
            zbase = zsysdep_base_name(zfrom);
            if zbase.is_null() {
                ucuabort();
            }
            zprompt = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_add(strlen(zbase)),
            );
            sprintf(
                zprompt,
                b"Remote file name [%s]: \0" as *const u8 as *const libc::c_char,
                zbase,
            );
            zto = zsysdep_terminal_line(zprompt);
            ubuffree(zprompt);
            if zto.is_null() {
                ucuabort();
            }
            *zto
                .offset(
                    strcspn(zto, b" \t\n\0" as *const u8 as *const libc::c_char) as isize,
                ) = '\0' as i32 as libc::c_char;
            if *zto as libc::c_int != '\0' as i32 {
                ubuffree(zbase);
            } else {
                ubuffree(zto);
                zto = zbase;
            }
        }
    }
    e = esysdep_user_fopen(zfrom, 1 as libc::c_int, fCuvar_binary);
    if e.is_null() {
        let mut zerrstr: *const libc::c_char = 0 as *const libc::c_char;
        if pvar.is_null() {
            ubuffree(zto);
        }
        zerrstr = strerror(*__errno_location());
        zalc = zbufalc(
            (strlen(zfrom))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                )
                .wrapping_add(strlen(zerrstr)),
        );
        sprintf(zalc, b"%s: %s\0" as *const u8 as *const libc::c_char, zfrom, zerrstr);
        ubuffree(zfrom);
        if fsysdep_terminal_puts(zalc) == 0 {
            ucuabort();
        }
        ubuffree(zalc);
        if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
            ucuabort();
        }
        return 0 as libc::c_int;
    }
    ubuffree(zfrom);
    if fsysdep_cu_copy(0 as libc::c_int) == 0
        || fsysdep_terminal_signals(1 as libc::c_int) == 0
    {
        ucuabort();
    }
    if pvar.is_null() {
        let mut fret: boolean = 0;
        zalc = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_add(strlen(zto)),
        );
        sprintf(zalc, b"cat > %s\n\0" as *const u8 as *const libc::c_char, zto);
        ubuffree(zto);
        fret = fcusend_buf(qconn, zalc, strlen(zalc));
        ubuffree(zalc);
        if fret == 0 {
            fclose(e);
            if fsysdep_cu_copy(1 as libc::c_int) == 0
                || fsysdep_terminal_signals(0 as libc::c_int) == 0
            {
                ucuabort();
            }
            if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
                ucuabort();
            }
            return 0 as libc::c_int;
        }
    }
    cline = 0 as libc::c_int;
    zbuf = 0 as *mut libc::c_char;
    cbuf = 0 as libc::c_int as size_t;
    loop {
        let mut abbuf: [libc::c_char; 512] = [0; 512];
        let mut c: size_t = 0;
        if fCuvar_binary != 0 {
            if feof(e) != 0 {
                break;
            }
            c = fread(
                abbuf.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                e,
            );
            if ferror(e) != 0 {
                if fsysdep_terminal_puts(
                    b"[file read error]\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    ucuabort();
                }
                break;
            } else {
                if c == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
                zbuf = abbuf.as_mut_ptr();
            }
        } else if getline(&mut zbuf, &mut cbuf, e) <= 0 as libc::c_int as libc::c_long {
            xfree(zbuf as pointer);
            break;
        } else {
            c = strlen(zbuf);
        }
        if fCuvar_verbose != 0 {
            cline += 1;
            cline;
            printf(b"%d \0" as *const u8 as *const libc::c_char, cline);
            fflush(stdout);
        }
        if fcusend_buf(qconn, zbuf, c) == 0 {
            if fCuvar_binary == 0 {
                xfree(zbuf as pointer);
            }
            fclose(e);
            if fsysdep_cu_copy(1 as libc::c_int) == 0
                || fsysdep_terminal_signals(0 as libc::c_int) == 0
            {
                ucuabort();
            }
            if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
                ucuabort();
            }
            return 0 as libc::c_int;
        }
    }
    fclose(e);
    if pvar.is_null() {
        let mut beof: libc::c_char = 0;
        beof = '\u{4}' as i32 as libc::c_char;
        if fconn_write(qconn, &mut beof, 1 as libc::c_int as size_t) == 0 {
            ucuabort();
        }
    } else if *zCuvar_eofwrite as libc::c_int != '\0' as i32 {
        if fconn_write(qconn, zCuvar_eofwrite, strlen(zCuvar_eofwrite)) == 0 {
            ucuabort();
        }
    }
    if fCuvar_verbose != 0 {
        if fsysdep_terminal_puts(b"\0" as *const u8 as *const libc::c_char) == 0 {
            ucuabort();
        }
    }
    if fsysdep_terminal_puts(
        b"[file transfer complete]\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        ucuabort();
    }
    if fsysdep_cu_copy(1 as libc::c_int) == 0
        || fsysdep_terminal_signals(0 as libc::c_int) == 0
    {
        ucuabort();
    }
    if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
        ucuabort();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icutake(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut qconn: *mut sconnection = pinfo as *mut sconnection;
    let mut zeof: *const libc::c_char = 0 as *const libc::c_char;
    let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: openfile_t = 0 as *mut FILE;
    let mut bcr: libc::c_char = 0;
    let mut ceoflen: size_t = 0;
    let mut zlook: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ceofhave: size_t = 0;
    let mut ferr: boolean = 0;
    if argc > 1 as libc::c_int {
        zfrom = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    } else {
        zfrom = zsysdep_terminal_line(
            b"Remote file to retreive: \0" as *const u8 as *const libc::c_char,
        );
        if zfrom.is_null() {
            ucuabort();
        }
        *zfrom
            .offset(
                strcspn(zfrom, b" \t\n\0" as *const u8 as *const libc::c_char) as isize,
            ) = '\0' as i32 as libc::c_char;
        if *zfrom as libc::c_int == '\0' as i32 {
            ubuffree(zfrom);
            if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
                ucuabort();
            }
            return 0 as libc::c_int;
        }
    }
    if argc > 2 as libc::c_int {
        zto = zbufcpy(*argv.offset(2 as libc::c_int as isize));
    } else {
        let mut zbase: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zprompt: *mut libc::c_char = 0 as *mut libc::c_char;
        zbase = zsysdep_base_name(zfrom);
        if zbase.is_null() {
            ucuabort();
        }
        zprompt = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                .wrapping_add(strlen(zbase)),
        );
        sprintf(
            zprompt,
            b"Local file name [%s]: \0" as *const u8 as *const libc::c_char,
            zbase,
        );
        zto = zsysdep_terminal_line(zprompt);
        ubuffree(zprompt);
        if zto.is_null() {
            ucuabort();
        }
        *zto
            .offset(
                strcspn(zto, b" \t\n\0" as *const u8 as *const libc::c_char) as isize,
            ) = '\0' as i32 as libc::c_char;
        if *zto as libc::c_int != '\0' as i32 {
            ubuffree(zbase);
        } else {
            ubuffree(zto);
            zto = zbase;
        }
    }
    if !pvar.is_null() {
        zcmd = zsysdep_terminal_line(
            b"Remote command to execute: \0" as *const u8 as *const libc::c_char,
        );
        if zcmd.is_null() {
            ucuabort();
        }
        *zcmd
            .offset(
                strcspn(zcmd, b"\n\0" as *const u8 as *const libc::c_char) as isize,
            ) = '\0' as i32 as libc::c_char;
        zeof = zCuvar_eofread;
    } else {
        zcmd = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong)
                .wrapping_add(strlen(zfrom)),
        );
        sprintf(
            zcmd,
            b"cat %s; echo; echo ////cuend////\0" as *const u8 as *const libc::c_char,
            zfrom,
        );
        zeof = b"\n////cuend////\n\0" as *const u8 as *const libc::c_char;
    }
    ubuffree(zfrom);
    e = esysdep_user_fopen(zto, 0 as libc::c_int, fCuvar_binary);
    if e.is_null() {
        let mut zerrstr: *const libc::c_char = 0 as *const libc::c_char;
        ubuffree(zcmd);
        zerrstr = strerror(*__errno_location());
        zalc = zbufalc(
            (strlen(zto))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                )
                .wrapping_add(strlen(zerrstr)),
        );
        sprintf(zalc, b"%s: %s\n\0" as *const u8 as *const libc::c_char, zto, zerrstr);
        if fsysdep_terminal_puts(zalc) == 0 {
            ucuabort();
        }
        ubuffree(zalc);
        if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
            ucuabort();
        }
        ubuffree(zto);
        return 0 as libc::c_int;
    }
    if fsysdep_cu_copy(0 as libc::c_int) == 0
        || fsysdep_terminal_signals(1 as libc::c_int) == 0
    {
        ucuabort();
    }
    if fconn_write(qconn, zcmd, strlen(zcmd)) == 0 {
        ucuabort();
    }
    bcr = '\r' as i32 as libc::c_char;
    if fconn_write(qconn, &mut bcr, 1 as libc::c_int as size_t) == 0 {
        ucuabort();
    }
    ubuffree(zcmd);
    iPrecstart = 0 as libc::c_int;
    iPrecend = 0 as libc::c_int;
    if pvar.is_null() {
        let mut b: libc::c_int = 0;
        loop {
            b = breceive_char(qconn, cCuvar_timeout, 1 as libc::c_int);
            if !(b != '\n' as i32) {
                break;
            }
            if b == -(2 as libc::c_int) {
                ucuabort();
            }
            if b < 0 as libc::c_int {
                if fsysdep_terminal_puts(
                    b"[timed out waiting for newline]\0" as *const u8
                        as *const libc::c_char,
                ) == 0
                {
                    ucuabort();
                }
                if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
                    ucuabort();
                }
                ubuffree(zto);
                return 0 as libc::c_int;
            }
        }
    }
    ceoflen = strlen(zeof);
    zlook = zbufalc(ceoflen);
    ceofhave = 0 as libc::c_int as size_t;
    ferr = 0 as libc::c_int;
    loop {
        let mut b_0: libc::c_int = 0;
        if afSignal[0 as libc::c_int as usize] != 0
            || afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0
        {
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
            if fsysdep_terminal_puts(
                b"[file receive aborted]\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                ucuabort();
            }
            ::std::ptr::write_volatile(
                &mut afSignal[1 as libc::c_int as usize] as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            break;
        } else {
            b_0 = breceive_char(qconn, cCuvar_timeout, 1 as libc::c_int);
            if b_0 == -(2 as libc::c_int) {
                ucuabort();
            }
            if b_0 < 0 as libc::c_int {
                if ceofhave > 0 as libc::c_int as libc::c_ulong {
                    fwrite(
                        zlook as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ceofhave,
                        e,
                    );
                }
                if fsysdep_terminal_puts(
                    b"[timed out]\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    ucuabort();
                }
                break;
            } else {
                if b_0 == '\r' as i32 && fCuvar_binary == 0 {
                    continue;
                }
                if ceoflen == 0 as libc::c_int as libc::c_ulong {
                    if !(fwrite(
                        &mut b_0 as *mut libc::c_int as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        1 as libc::c_int as libc::c_ulong,
                        e,
                    ) != 1 as libc::c_int as libc::c_ulong)
                    {
                        continue;
                    }
                    ferr = 1 as libc::c_int;
                    break;
                } else {
                    *zlook.offset(ceofhave as isize) = b_0 as libc::c_char;
                    ceofhave = ceofhave.wrapping_add(1);
                    ceofhave;
                    if !(ceofhave == ceoflen) {
                        continue;
                    }
                    let mut cmove: size_t = 0;
                    let mut zmove: *mut libc::c_char = 0 as *mut libc::c_char;
                    if memcmp(
                        zeof as *const libc::c_void,
                        zlook as *const libc::c_void,
                        ceoflen,
                    ) == 0 as libc::c_int
                    {
                        if fsysdep_terminal_puts(
                            b"[file transfer complete]\0" as *const u8
                                as *const libc::c_char,
                        ) == 0
                        {
                            ucuabort();
                        }
                        break;
                    } else if fwrite(
                        zlook as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        1 as libc::c_int as libc::c_ulong,
                        e,
                    ) != 1 as libc::c_int as libc::c_ulong
                    {
                        ferr = 1 as libc::c_int;
                        break;
                    } else {
                        zmove = zlook;
                        cmove = ceoflen.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        zmove = zlook;
                        while cmove > 0 as libc::c_int as libc::c_ulong {
                            *zmove
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = *zmove.offset(1 as libc::c_int as isize);
                            cmove = cmove.wrapping_sub(1);
                            cmove;
                            zmove = zmove.offset(1);
                            zmove;
                        }
                        ceofhave = ceofhave.wrapping_sub(1);
                        ceofhave;
                    }
                }
            }
        }
    }
    ubuffree(zlook);
    if fsysdep_sync(e, zto) == 0 {
        fclose(e);
        ferr = 1 as libc::c_int;
    } else if !(fclose(e) == 0 as libc::c_int) {
        ferr = 1 as libc::c_int;
    }
    if ferr != 0 {
        if fsysdep_terminal_puts(
            b"[file write error]\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ucuabort();
        }
    }
    if fsysdep_cu_copy(1 as libc::c_int) == 0
        || fsysdep_terminal_signals(0 as libc::c_int) == 0
    {
        ucuabort();
    }
    if fsysdep_terminal_puts(abCuconnected.as_ptr()) == 0 {
        ucuabort();
    }
    ubuffree(zto);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fcusend_buf(
    mut qconn: *mut sconnection,
    mut zbufarg: *const libc::c_char,
    mut cbufarg: size_t,
) -> boolean {
    let mut zbuf: *const libc::c_char = 0 as *const libc::c_char;
    let mut cbuf: size_t = 0;
    let mut ctries: libc::c_int = 0;
    let mut cbplen: size_t = 0;
    let mut zsendbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    zbuf = zbufarg;
    cbuf = cbufarg;
    ctries = 0 as libc::c_int;
    if fCuvar_binary != 0 {
        cbplen = strlen(zCuvar_binary_prefix);
    } else {
        cbplen = 1 as libc::c_int as size_t;
    }
    zsendbuf = zbufalc(
        (64 as libc::c_int as libc::c_ulong)
            .wrapping_mul(cbplen.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    );
    while cbuf > 0 as libc::c_int as libc::c_ulong {
        let mut csend: libc::c_int = 0;
        let mut zput: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut zget: *const libc::c_char = 0 as *const libc::c_char;
        let mut fnl: boolean = 0;
        let mut i: libc::c_int = 0;
        if afSignal[0 as libc::c_int as usize] != 0
            || afSignal[1 as libc::c_int as usize] != 0
            || afSignal[2 as libc::c_int as usize] != 0
            || afSignal[3 as libc::c_int as usize] != 0
            || afSignal[4 as libc::c_int as usize] != 0
        {
            ubuffree(zsendbuf);
            ulog(LOG_ERROR, 0 as *mut libc::c_void as *const libc::c_char);
            if fsysdep_terminal_puts(
                b"[file send aborted]\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                ucuabort();
            }
            ::std::ptr::write_volatile(
                &mut afSignal[1 as libc::c_int as usize] as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        iPrecstart = 0 as libc::c_int;
        iPrecend = 0 as libc::c_int;
        if *zbuf as libc::c_int == '\n' as i32 {
            csend = 1 as libc::c_int;
        } else {
            let mut znl: *const libc::c_char = 0 as *const libc::c_char;
            znl = memchr(zbuf as *const libc::c_void, '\n' as i32, cbuf)
                as *const libc::c_char;
            if znl.is_null() {
                csend = cbuf as libc::c_int;
            } else {
                csend = znl.offset_from(zbuf) as libc::c_long as libc::c_int;
            }
            if csend > 64 as libc::c_int {
                csend = 64 as libc::c_int;
            }
        }
        zput = zsendbuf;
        fnl = 0 as libc::c_int;
        i = 0 as libc::c_int;
        zget = zbuf;
        while i < csend {
            if *(*__ctype_b_loc()).offset(*zget as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *zget as libc::c_int == '\t' as i32
            {
                let fresh1 = zput;
                zput = zput.offset(1);
                *fresh1 = *zget;
            } else if *zget as libc::c_int == '\n' as i32 {
                if fCuvar_binary != 0 {
                    let fresh2 = zput;
                    zput = zput.offset(1);
                    *fresh2 = '\n' as i32 as libc::c_char;
                } else {
                    let fresh3 = zput;
                    zput = zput.offset(1);
                    *fresh3 = '\r' as i32 as libc::c_char;
                }
                fnl = 1 as libc::c_int;
            } else if fCuvar_binary != 0 {
                strcpy(zput, zCuvar_binary_prefix);
                zput = zput.offset(cbplen as isize);
                let fresh4 = zput;
                zput = zput.offset(1);
                *fresh4 = *zget;
            }
            i += 1;
            i;
            zget = zget.offset(1);
            zget;
        }
        zbuf = zbuf.offset(csend as isize);
        cbuf = (cbuf as libc::c_ulong).wrapping_sub(csend as libc::c_ulong) as size_t
            as size_t;
        if zput == zsendbuf {
            continue;
        }
        if fsend_data(
            qconn,
            zsendbuf,
            zput.offset_from(zsendbuf) as libc::c_long as size_t,
            1 as libc::c_int,
        ) == 0
        {
            ucuabort();
        }
        if fCuvar_echocheck != 0 && fCuvar_binary == 0
            || fnl != 0 && *zCuvar_echonl as libc::c_int != '\0' as i32
        {
            let mut iend: libc::c_long = 0;
            iend = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
                + cCuvar_timeout as libc::c_long;
            let mut current_block_82: u64;
            zget = zsendbuf;
            while zget < zput as *const libc::c_char {
                let mut bread: libc::c_int = 0;
                let mut bwant: libc::c_int = 0;
                if if fCuvar_binary != 0 {
                    (*zget as libc::c_int == '\n' as i32) as libc::c_int
                } else {
                    (*zget as libc::c_int == '\r' as i32) as libc::c_int
                } != 0
                {
                    bwant = *zCuvar_echonl as libc::c_int;
                    if bwant == '\0' as i32 {
                        current_block_82 = 1434579379687443766;
                    } else {
                        current_block_82 = 2290177392965769716;
                    }
                } else if fCuvar_echocheck == 0
                    || *(*__ctype_b_loc()).offset(*zget as libc::c_int as isize)
                        as libc::c_int
                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    current_block_82 = 1434579379687443766;
                } else {
                    bwant = *zget as libc::c_int;
                    current_block_82 = 2290177392965769716;
                }
                match current_block_82 {
                    2290177392965769716 => {
                        loop {
                            if afSignal[0 as libc::c_int as usize] != 0
                                || afSignal[1 as libc::c_int as usize] != 0
                                || afSignal[2 as libc::c_int as usize] != 0
                                || afSignal[3 as libc::c_int as usize] != 0
                                || afSignal[4 as libc::c_int as usize] != 0
                            {
                                ubuffree(zsendbuf);
                                ulog(
                                    LOG_ERROR,
                                    0 as *mut libc::c_void as *const libc::c_char,
                                );
                                if fsysdep_terminal_puts(
                                    b"[file send aborted]\0" as *const u8 as *const libc::c_char,
                                ) == 0
                                {
                                    ucuabort();
                                }
                                ::std::ptr::write_volatile(
                                    &mut afSignal[1 as libc::c_int as usize]
                                        as *mut sig_atomic_t,
                                    0 as libc::c_int,
                                );
                                return 0 as libc::c_int;
                            }
                            bread = breceive_char(
                                qconn,
                                (iend
                                    - ixsysdep_time(
                                        0 as *mut libc::c_void as *mut libc::c_long,
                                    )) as libc::c_int,
                                1 as libc::c_int,
                            );
                            if bread < 0 as libc::c_int {
                                if bread == -(2 as libc::c_int) {
                                    ucuabort();
                                }
                                if fCuvar_binary == 0
                                    && *zCuvar_kill as libc::c_int != '\0' as i32
                                {
                                    ctries += 1;
                                    ctries;
                                    if ctries < cCuvar_resend {
                                        if fCuvar_verbose != 0 {
                                            printf(b"R \0" as *const u8 as *const libc::c_char);
                                            fflush(stdout);
                                        }
                                        if fsend_data(
                                            qconn,
                                            zCuvar_kill,
                                            1 as libc::c_int as size_t,
                                            1 as libc::c_int,
                                        ) == 0
                                        {
                                            ucuabort();
                                        }
                                        zbuf = zbufarg;
                                        cbuf = cbufarg;
                                        break;
                                    }
                                }
                                ubuffree(zsendbuf);
                                if fsysdep_terminal_puts(
                                    b"[timed out looking for echo]\0" as *const u8
                                        as *const libc::c_char,
                                ) == 0
                                {
                                    ucuabort();
                                }
                                return 0 as libc::c_int;
                            } else if !(bread != *zget as libc::c_int) {
                                break;
                            }
                        }
                        if bread < 0 as libc::c_int {
                            break;
                        }
                    }
                    _ => {}
                }
                zget = zget.offset(1);
                zget;
            }
        }
    }
    ubuffree(zsendbuf);
    return 1 as libc::c_int;
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
