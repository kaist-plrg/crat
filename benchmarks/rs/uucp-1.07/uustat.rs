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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut gnu_optarg: *mut libc::c_char;
    static mut gnu_optind: libc::c_int;
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
    fn fspool_file(zfile: *const libc::c_char) -> boolean;
    fn fin_directory_list(
        zfile: *const libc::c_char,
        pzdirs: *mut *mut libc::c_char,
        zpubdir: *const libc::c_char,
        fcheck: boolean,
        freadable: boolean,
        zuser: *const libc::c_char,
    ) -> boolean;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_close();
    fn idebug_parse(_: *const libc::c_char) -> libc::c_int;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    fn xfree(_: pointer);
    static mut zProgram: *const libc::c_char;
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
    fn uuconf_system_local(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_qsys: *mut uuconf_system,
    ) -> libc::c_int;
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_debuglevel(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzdebug: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_cmd_file(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_e: *mut FILE,
        uuconf_qtab: *const uuconf_cmdtab,
        uuconf_pinfo: *mut libc::c_void,
        uuconf_pfiunknownfn: uuconf_cmdtabfn,
        uuconf_iflags: libc::c_int,
        pblock: *mut libc::c_void,
    ) -> libc::c_int;
    fn uuconf_free_block(uuconf_pblock: *mut libc::c_void);
    fn usysdep_initialize(puuconf: pointer, iflags: libc::c_int);
    fn usysdep_exit(fsuccess: boolean);
    fn fsysdep_other_config(_: *const libc::c_char) -> boolean;
    fn zsysdep_localname() -> *const libc::c_char;
    fn zsysdep_login_name() -> *const libc::c_char;
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
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    fn usysdep_localtime(itime: libc::c_long, q: *mut tm);
    fn fsysdep_get_status(
        qsys: *const uuconf_system,
        qret: *mut sstatus,
        pfnone: *mut boolean,
    ) -> boolean;
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
    fn usysdep_get_work_free(qsys: *const uuconf_system);
    fn zsysdep_spool_file_name(
        qsys: *const uuconf_system,
        zfile: *const libc::c_char,
        pseq: pointer,
    ) -> *mut libc::c_char;
    fn fsysdep_get_xqt_init(zsystem: *const libc::c_char) -> boolean;
    fn zsysdep_get_xqt(
        zsystem: *const libc::c_char,
        pzsystem: *mut *mut libc::c_char,
        pferr: *mut boolean,
    ) -> *mut libc::c_char;
    fn usysdep_get_xqt_free(zsystem: *const libc::c_char);
    fn zsysdep_jobid(qsys: *const uuconf_system, pseq: pointer) -> *mut libc::c_char;
    fn fsysdep_privileged() -> boolean;
    fn fsysdep_kill_job(puuconf: pointer, zjobid: *const libc::c_char) -> boolean;
    fn fsysdep_rejuvenate_job(puuconf: pointer, zjobid: *const libc::c_char) -> boolean;
    fn ixsysdep_work_time(qsys: *const uuconf_system, pseq: pointer) -> libc::c_long;
    fn ixsysdep_file_time(zfile: *const libc::c_char) -> libc::c_long;
    fn fsysdep_touch_file(zfile: *const libc::c_char) -> boolean;
    fn csysdep_size(zfile: *const libc::c_char) -> libc::c_long;
    fn fsysdep_all_status_init(phold: *mut pointer) -> boolean;
    fn zsysdep_all_status(
        phold: pointer,
        pferr: *mut boolean,
        qstat: *mut sstatus,
    ) -> *mut libc::c_char;
    fn usysdep_all_status_free(phold: pointer);
    fn fsysdep_lock_status() -> boolean;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sxqtlist {
    pub qnext: *mut sxqtlist,
    pub zsystem: *mut libc::c_char,
    pub cxqts: libc::c_int,
    pub ifirst: libc::c_long,
}
pub type UUCONF_POINTER = *mut libc::c_void;
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
pub struct scmdlist {
    pub qnext: *mut scmdlist,
    pub s: scmd,
    pub itime: libc::c_long,
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
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
pub static mut uustat_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: uustat.c,v 1.61 2002/03/05 19:10:42 ian Rel $\0")
};
static mut asSlongopts: [option; 28] = [
    {
        let mut init = option {
            name: b"all\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mail-lines\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"command\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"not-command\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"executions\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"kill\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"kill-all\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"status\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mail\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"notify\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"older-than\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ps\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-list\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"rejuvenate\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"rejuvenate-all\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
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
            name: b"not-system\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"not-user\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"comment\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"younger-than\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'y' as i32,
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
    let mut fall: boolean = 0 as libc::c_int;
    let mut cstdin: libc::c_int = 100 as libc::c_int;
    let mut ccommands: libc::c_int = 0 as libc::c_int;
    let mut pazcommands: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fnotcommands: boolean = 0 as libc::c_int;
    let mut fexecute: boolean = 0 as libc::c_int;
    let mut ckills: libc::c_int = 0 as libc::c_int;
    let mut pazkills: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fmachine: boolean = 0 as libc::c_int;
    let mut ioldhours: libc::c_int = -(1 as libc::c_int);
    let mut fps: boolean = 0 as libc::c_int;
    let mut fquery: boolean = 0 as libc::c_int;
    let mut crejuvs: libc::c_int = 0 as libc::c_int;
    let mut pazrejuvs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut csystems: libc::c_int = 0 as libc::c_int;
    let mut pazsystems: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fnotsystems: boolean = 0 as libc::c_int;
    let mut cusers: libc::c_int = 0 as libc::c_int;
    let mut pazusers: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fnotusers: boolean = 0 as libc::c_int;
    let mut zcomment: *const libc::c_char = 0 as *const libc::c_char;
    let mut iyounghours: libc::c_int = -(1 as libc::c_int);
    let mut zconfig: *const libc::c_char = 0 as *const libc::c_char;
    let mut icmd: libc::c_int = 0o1 as libc::c_int;
    let mut ccmds: libc::c_int = 0;
    let mut iopt: libc::c_int = 0;
    let mut puuconf: pointer = 0 as *mut libc::c_void;
    let mut iuuconf: libc::c_int = 0;
    let mut iold: libc::c_long = 0;
    let mut iyoung: libc::c_long = 0;
    let mut azoneuser: [*const libc::c_char; 1] = [0 as *const libc::c_char; 1];
    let mut fret: boolean = 0;
    zProgram = *argv.offset(0 as libc::c_int as isize);
    loop {
        iopt = getopt_long(
            argc,
            argv,
            b"aB:c:C:eiI:k:KmMNo:pqQr:Rs:S:u:U:vW:x:y:\0" as *const u8
                as *const libc::c_char,
            asSlongopts.as_ptr(),
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_44: u64;
        match iopt {
            97 => {
                fall = 1 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            66 => {
                cstdin = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            67 => {
                fnotcommands = 1 as libc::c_int;
                current_block_44 = 6820290883636756191;
            }
            99 => {
                current_block_44 = 6820290883636756191;
            }
            101 => {
                fexecute = 1 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            105 => {
                icmd |= 0o2 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            73 => {
                if fsysdep_other_config(gnu_optarg) != 0 {
                    zconfig = gnu_optarg;
                }
                current_block_44 = 4888910987971495881;
            }
            107 => {
                ckills += 1;
                ckills;
                pazkills = xrealloc(
                    pazkills as pointer,
                    (ckills as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                let ref mut fresh1 = *pazkills
                    .offset((ckills - 1 as libc::c_int) as isize);
                *fresh1 = gnu_optarg;
                current_block_44 = 4888910987971495881;
            }
            75 => {
                icmd |= 0o4 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            109 => {
                fmachine = 1 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            77 => {
                icmd |= 0o20 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            78 => {
                icmd |= 0o40 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            111 => {
                ioldhours = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            112 => {
                fps = 1 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            113 => {
                fquery = 1 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            81 => {
                icmd &= !(0o1 as libc::c_int);
                current_block_44 = 4888910987971495881;
            }
            114 => {
                crejuvs += 1;
                crejuvs;
                pazrejuvs = xrealloc(
                    pazrejuvs as pointer,
                    (crejuvs as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                let ref mut fresh2 = *pazrejuvs
                    .offset((crejuvs - 1 as libc::c_int) as isize);
                *fresh2 = gnu_optarg;
                current_block_44 = 4888910987971495881;
            }
            82 => {
                icmd |= 0o10 as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            83 => {
                fnotsystems = 1 as libc::c_int;
                current_block_44 = 8585102411681329058;
            }
            115 => {
                current_block_44 = 8585102411681329058;
            }
            85 => {
                fnotusers = 1 as libc::c_int;
                current_block_44 = 5926169326854709237;
            }
            117 => {
                current_block_44 = 5926169326854709237;
            }
            87 => {
                zcomment = gnu_optarg;
                current_block_44 = 4888910987971495881;
            }
            120 => {
                iDebug |= idebug_parse(gnu_optarg);
                current_block_44 = 4888910987971495881;
            }
            121 => {
                iyounghours = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
                current_block_44 = 4888910987971495881;
            }
            118 => {
                printf(
                    b"uustat (Taylor UUCP) %s\n\0" as *const u8 as *const libc::c_char,
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
            1 => {
                ushelp();
                exit(0 as libc::c_int);
            }
            0 => {
                current_block_44 = 4888910987971495881;
            }
            _ => {
                ususage();
                current_block_44 = 4888910987971495881;
            }
        }
        match current_block_44 {
            5926169326854709237 => {
                cusers += 1;
                cusers;
                pazusers = xrealloc(
                    pazusers as pointer,
                    (cusers as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                let ref mut fresh4 = *pazusers
                    .offset((cusers - 1 as libc::c_int) as isize);
                *fresh4 = gnu_optarg;
            }
            8585102411681329058 => {
                csystems += 1;
                csystems;
                pazsystems = xrealloc(
                    pazsystems as pointer,
                    (csystems as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                let ref mut fresh3 = *pazsystems
                    .offset((csystems - 1 as libc::c_int) as isize);
                *fresh3 = gnu_optarg;
            }
            6820290883636756191 => {
                ccommands += 1;
                ccommands;
                pazcommands = xrealloc(
                    pazcommands as pointer,
                    (ccommands as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                let ref mut fresh0 = *pazcommands
                    .offset((ccommands - 1 as libc::c_int) as isize);
                *fresh0 = gnu_optarg;
            }
            _ => {}
        }
    }
    if gnu_optind != argc {
        ususage();
    }
    ccmds = 0 as libc::c_int;
    if fall != 0 {
        ccmds += 1;
        ccmds;
    }
    if ckills > 0 as libc::c_int || crejuvs > 0 as libc::c_int {
        ccmds += 1;
        ccmds;
    }
    if fmachine != 0 {
        ccmds += 1;
        ccmds;
    }
    if fps != 0 {
        ccmds += 1;
        ccmds;
    }
    if fexecute != 0 || fquery != 0 || csystems > 0 as libc::c_int
        || cusers > 0 as libc::c_int || ioldhours != -(1 as libc::c_int)
        || iyounghours != -(1 as libc::c_int) || ccommands > 0 as libc::c_int
    {
        ccmds += 1;
        ccmds;
    }
    if fexecute != 0 && fquery != 0 {
        ccmds += 1;
        ccmds;
    }
    if ccmds > 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s: too many options\n\0" as *const u8 as *const libc::c_char,
            zProgram,
        );
        ususage();
    }
    if icmd & 0o4 as libc::c_int != 0 as libc::c_int
        && icmd & 0o10 as libc::c_int != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"%s: can not both rejuvenate and kill jobs\n\0" as *const u8
                as *const libc::c_char,
            zProgram,
        );
        ususage();
    }
    iuuconf = uuconf_init(
        &mut puuconf,
        0 as *mut libc::c_void as *const libc::c_char,
        zconfig,
    );
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    let mut zdebug: *const libc::c_char = 0 as *const libc::c_char;
    iuuconf = uuconf_debuglevel(puuconf, &mut zdebug);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    if !zdebug.is_null() {
        iDebug |= idebug_parse(zdebug);
    }
    usysdep_initialize(puuconf, 0o4 as libc::c_int);
    if ccmds == 0 as libc::c_int {
        cusers = 1 as libc::c_int;
        azoneuser[0 as libc::c_int as usize] = zsysdep_login_name();
        pazusers = azoneuser.as_mut_ptr() as *mut *mut libc::c_char;
    }
    if csystems > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < csystems {
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
            iuuconf = uuconf_system_info(
                puuconf,
                *pazsystems.offset(i as isize),
                &mut ssys,
            );
            if iuuconf != 0 as libc::c_int {
                if iuuconf == 1 as libc::c_int {
                    ulog(
                        LOG_FATAL,
                        b"%s: System not found\0" as *const u8 as *const libc::c_char,
                        *pazsystems.offset(i as isize),
                    );
                } else {
                    ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
                }
            }
            if strcmp(*pazsystems.offset(i as isize), ssys.uuconf_zname)
                != 0 as libc::c_int
            {
                let ref mut fresh5 = *pazsystems.offset(i as isize);
                *fresh5 = zbufcpy(ssys.uuconf_zname);
            }
            uuconf_free_block(ssys.uuconf_palloc);
            i += 1;
            i;
        }
    }
    if ioldhours == -(1 as libc::c_int) {
        iold = -(1 as libc::c_int) as libc::c_long;
    } else {
        iold = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
            - ioldhours as libc::c_long * 60 as libc::c_int as libc::c_long
                * 60 as libc::c_int as libc::c_long;
        if iold < 0 as libc::c_long {
            iold = 0 as libc::c_long;
        }
    }
    if iyounghours == -(1 as libc::c_int) {
        iyoung = -(1 as libc::c_int) as libc::c_long;
    } else {
        iyoung = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
            - iyounghours as libc::c_long * 60 as libc::c_int as libc::c_long
                * 60 as libc::c_int as libc::c_long;
        if iyoung < 0 as libc::c_long {
            iyoung = 0 as libc::c_long;
        }
    }
    if fexecute == 0 && fquery == 0
        && (fall != 0 || csystems > 0 as libc::c_int || cusers > 0 as libc::c_int
            || ioldhours != -(1 as libc::c_int) || iyounghours != -(1 as libc::c_int)
            || ccommands > 0 as libc::c_int)
    {
        fret = fsworkfiles(
            puuconf,
            icmd,
            csystems,
            pazsystems,
            fnotsystems,
            cusers,
            pazusers,
            fnotusers,
            iold,
            iyoung,
            ccommands,
            pazcommands,
            fnotcommands,
            zcomment,
            cstdin,
        );
    } else if fexecute != 0 {
        fret = fsexecutions(
            puuconf,
            icmd,
            csystems,
            pazsystems,
            fnotsystems,
            cusers,
            pazusers,
            fnotusers,
            iold,
            iyoung,
            ccommands,
            pazcommands,
            fnotcommands,
            zcomment,
            cstdin,
        );
    } else if icmd != 0o1 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"-i, -K, -M, -N, -Q, -R not supported with -k, -m, -p, -q, -r\0"
                as *const u8 as *const libc::c_char,
        );
        ususage();
        fret = 0 as libc::c_int;
    } else if fquery != 0 {
        if cusers > 0 as libc::c_int || ccommands > 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"-u, -c not supported with -q\0" as *const u8 as *const libc::c_char,
            );
            ususage();
            fret = 0 as libc::c_int;
        } else {
            fret = fsquery(puuconf, csystems, pazsystems, fnotsystems, iold, iyoung);
        }
    } else if fmachine != 0 {
        fret = fsmachines();
    } else if ckills > 0 as libc::c_int || crejuvs > 0 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        fret = 1 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < ckills {
            if fsysdep_kill_job(puuconf, *pazkills.offset(i_0 as isize)) == 0 {
                fret = 0 as libc::c_int;
            }
            i_0 += 1;
            i_0;
        }
        i_0 = 0 as libc::c_int;
        while i_0 < crejuvs {
            if fsysdep_rejuvenate_job(puuconf, *pazrejuvs.offset(i_0 as isize)) == 0 {
                fret = 0 as libc::c_int;
            }
            i_0 += 1;
            i_0;
        }
    } else if fps != 0 {
        fret = fsysdep_lock_status();
    } else {
        ulog(LOG_FATAL, b"Can't happen\0" as *const u8 as *const libc::c_char);
        fret = 0 as libc::c_int;
    }
    ulog_close();
    usysdep_exit(fret);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ususage() {
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
unsafe extern "C" fn ushelp() {
    printf(
        b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995, 2002 Ian Lance Taylor\n\0"
            as *const u8 as *const libc::c_char,
        b"1.07\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Usage: %s [options]\n\0" as *const u8 as *const libc::c_char, zProgram);
    printf(b" -a,--all: list all UUCP jobs\n\0" as *const u8 as *const libc::c_char);
    printf(
        b" -B,--mail-lines num: number of lines to return in -M or -N mail message\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -c,--command command: list requests for named command\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -C,--not-command command: list requests for other than named command\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -e,--executions: list queued executions rather than job requests\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -i,--prompt: prompt for whether to kill each listed job\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -k,--kill job: kill specified UUCP job\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -K,--kill-all: kill each listed job\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -m,--status: report status for all remote machines\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -M,--mail: mail report on each listed job to UUCP administrator\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -N,--notify: mail report on each listed job to requestor\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -o,--older-than hours: list all jobs older than given number of hours\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -p,--ps: show status of all processes holding UUCP locks\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -q,--list: list number of jobs for each system\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -Q,--no-list: don't list jobs, just take actions (-i, -K, -M, -N)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -r,--rejuvenate job: rejuvenate specified UUCP job\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -R,--rejuvenate-all: rejuvenate each listed job\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -s,--system system: list all jobs for specified system\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -S,--not-system system: list all jobs for other than specified system\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -u,--user user: list all jobs for specified user\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -U,--not-user user: list all jobs for other than specified user\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -W,--comment comment: comment to include in mail messages\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" -y,--younger-than hours: list all jobs younger than given number of hours\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" -x,--debug debug: Set debugging level\n\0" as *const u8 as *const libc::c_char,
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
static mut zSxqt_user: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zSxqt_system: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut zSxqt_requestor: *const libc::c_char = 0 as *const libc::c_char;
static mut zSxqt_prog: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut zSxqt_cmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut cSxqt_files: libc::c_int = 0;
static mut pazSxqt_files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut zSxqt_stdin: *const libc::c_char = 0 as *const libc::c_char;
static mut asSxqt_cmds: [uuconf_cmdtab; 6] = unsafe {
    [
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"C\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    isxqt_cmd
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
                uuconf_zcmd: b"I\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zSxqt_stdin as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"F\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 0 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    isxqt_file
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
                uuconf_zcmd: b"R\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x40 as libc::c_int,
                uuconf_pvar: &zSxqt_requestor as *const *const libc::c_char
                    as *mut *const libc::c_char as pointer,
                uuconf_pifn: None,
            };
            init
        },
        {
            let mut init = uuconf_cmdtab {
                uuconf_zcmd: b"U\0" as *const u8 as *const libc::c_char,
                uuconf_itype: 0x60 as libc::c_int | 3 as libc::c_int,
                uuconf_pvar: 0 as *const libc::c_void as *mut libc::c_void,
                uuconf_pifn: Some(
                    isxqt_user
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
unsafe extern "C" fn fsxqt_file_read(mut puuconf: pointer, mut e: *mut FILE) -> boolean {
    let mut iuuconf: libc::c_int = 0;
    let mut fret: boolean = 0;
    zSxqt_user = 0 as *mut libc::c_char;
    zSxqt_system = 0 as *mut libc::c_char;
    zSxqt_stdin = 0 as *const libc::c_char;
    zSxqt_requestor = 0 as *const libc::c_char;
    zSxqt_prog = 0 as *mut libc::c_char;
    zSxqt_cmd = 0 as *mut libc::c_char;
    cSxqt_files = 0 as libc::c_int;
    pazSxqt_files = 0 as *mut *mut libc::c_char;
    iuuconf = uuconf_cmd_file(
        puuconf,
        e,
        asSxqt_cmds.as_ptr(),
        0 as *mut libc::c_void,
        ::std::mem::transmute::<
            *mut libc::c_void,
            uuconf_cmdtabfn,
        >(0 as *mut libc::c_void),
        0x1 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if iuuconf == 0 as libc::c_int {
        fret = 1 as libc::c_int;
    } else {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        fret = 0 as libc::c_int;
    }
    if zSxqt_user.is_null() {
        zSxqt_user = zbufcpy(b"*unknown*\0" as *const u8 as *const libc::c_char);
    }
    if zSxqt_system.is_null() {
        zSxqt_system = zbufcpy(b"*unknown*\0" as *const u8 as *const libc::c_char);
    }
    if zSxqt_prog.is_null() {
        zSxqt_prog = zbufcpy(b"*none*\0" as *const u8 as *const libc::c_char);
        zSxqt_cmd = zbufcpy(b"*none*\0" as *const u8 as *const libc::c_char);
    }
    return fret;
}
unsafe extern "C" fn usxqt_file_free() {
    let mut i: libc::c_int = 0;
    ubuffree(zSxqt_user);
    zSxqt_user = 0 as *mut libc::c_char;
    ubuffree(zSxqt_system);
    zSxqt_system = 0 as *mut libc::c_char;
    ubuffree(zSxqt_prog);
    zSxqt_prog = 0 as *mut libc::c_char;
    ubuffree(zSxqt_cmd);
    zSxqt_cmd = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < cSxqt_files {
        ubuffree(*pazSxqt_files.offset(i as isize));
        i += 1;
        i;
    }
    cSxqt_files = 0 as libc::c_int;
    xfree(pazSxqt_files as pointer);
    pazSxqt_files = 0 as *mut *mut libc::c_char;
    zSxqt_stdin = 0 as *const libc::c_char;
    zSxqt_requestor = 0 as *const libc::c_char;
}
unsafe extern "C" fn isxqt_cmd(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    let mut clen: size_t = 0;
    let mut i: libc::c_int = 0;
    if argc <= 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    zSxqt_prog = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    clen = 0 as libc::c_int as size_t;
    i = 1 as libc::c_int;
    while i < argc {
        clen = (clen as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        i += 1;
        i;
    }
    zSxqt_cmd = zbufalc(clen);
    *zSxqt_cmd.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    i = 1 as libc::c_int;
    while i < argc - 1 as libc::c_int {
        strcat(zSxqt_cmd, *argv.offset(i as isize));
        strcat(zSxqt_cmd, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    strcat(zSxqt_cmd, *argv.offset(i as isize));
    return 0 as libc::c_int;
}
unsafe extern "C" fn isxqt_file(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        return 0 as libc::c_int;
    }
    if fspool_file(*argv.offset(1 as libc::c_int as isize)) == 0 {
        return 0 as libc::c_int;
    }
    cSxqt_files += 1;
    cSxqt_files;
    pazSxqt_files = xrealloc(
        pazSxqt_files as pointer,
        (cSxqt_files as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh6 = *pazSxqt_files
        .offset((cSxqt_files - 1 as libc::c_int) as isize);
    *fresh6 = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    return 0 as libc::c_int;
}
unsafe extern "C" fn isxqt_user(
    mut puuconf: pointer,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pvar: pointer,
    mut pinfo: pointer,
) -> libc::c_int {
    zSxqt_user = zbufcpy(*argv.offset(1 as libc::c_int as isize));
    zSxqt_system = zbufcpy(*argv.offset(2 as libc::c_int as isize));
    return 0 as libc::c_int;
}
unsafe extern "C" fn fsworkfiles(
    mut puuconf: pointer,
    mut icmd: libc::c_int,
    mut csystems: libc::c_int,
    mut pazsystems: *mut *mut libc::c_char,
    mut fnotsystems: boolean,
    mut cusers: libc::c_int,
    mut pazusers: *mut *mut libc::c_char,
    mut fnotusers: boolean,
    mut iold: libc::c_long,
    mut iyoung: libc::c_long,
    mut ccommands: libc::c_int,
    mut pazcommands: *mut *mut libc::c_char,
    mut fnotcommands: boolean,
    mut zcomment: *const libc::c_char,
    mut cstdin: libc::c_int,
) -> boolean {
    let mut fret: boolean = 0;
    let mut i: libc::c_int = 0;
    let mut iuuconf: libc::c_int = 0;
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
    fret = 1 as libc::c_int;
    if csystems > 0 as libc::c_int && fnotsystems == 0 {
        i = 0 as libc::c_int;
        while i < csystems {
            iuuconf = uuconf_system_info(
                puuconf,
                *pazsystems.offset(i as isize),
                &mut ssys,
            );
            if iuuconf != 0 as libc::c_int {
                if iuuconf == 1 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"%s: System not found\0" as *const u8 as *const libc::c_char,
                        *pazsystems.offset(i as isize),
                    );
                } else {
                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                }
                fret = 0 as libc::c_int;
            } else {
                if fsworkfiles_system(
                    puuconf,
                    icmd,
                    &mut ssys,
                    cusers,
                    pazusers,
                    fnotusers,
                    iold,
                    iyoung,
                    ccommands,
                    pazcommands,
                    fnotcommands,
                    zcomment,
                    cstdin,
                ) == 0
                {
                    fret = 0 as libc::c_int;
                }
                uuconf_free_block(ssys.uuconf_palloc);
            }
            i += 1;
            i;
        }
    } else {
        let mut pznames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        iuuconf = uuconf_system_names(puuconf, &mut pznames, 0 as libc::c_int);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            return 0 as libc::c_int;
        }
        let mut current_block_26: u64;
        pz = pznames;
        while !(*pz).is_null() {
            if csystems > 0 as libc::c_int {
                i = 0 as libc::c_int;
                while i < csystems {
                    if strcmp(*pz, *pazsystems.offset(i as isize)) == 0 as libc::c_int {
                        break;
                    }
                    i += 1;
                    i;
                }
                if i < csystems {
                    current_block_26 = 6009453772311597924;
                } else {
                    current_block_26 = 17478428563724192186;
                }
            } else {
                current_block_26 = 17478428563724192186;
            }
            match current_block_26 {
                17478428563724192186 => {
                    iuuconf = uuconf_system_info(puuconf, *pz, &mut ssys);
                    if iuuconf != 0 as libc::c_int {
                        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                        fret = 0 as libc::c_int;
                    } else {
                        if fsworkfiles_system(
                            puuconf,
                            icmd,
                            &mut ssys,
                            cusers,
                            pazusers,
                            fnotusers,
                            iold,
                            iyoung,
                            ccommands,
                            pazcommands,
                            fnotcommands,
                            zcomment,
                            cstdin,
                        ) == 0
                        {
                            fret = 0 as libc::c_int;
                        }
                        uuconf_free_block(ssys.uuconf_palloc);
                        xfree(*pz as pointer);
                    }
                }
                _ => {}
            }
            pz = pz.offset(1);
            pz;
        }
        xfree(pznames as pointer);
    }
    return fret;
}
unsafe extern "C" fn fsworkfiles_system(
    mut puuconf: pointer,
    mut icmd: libc::c_int,
    mut qsys: *const uuconf_system,
    mut cusers: libc::c_int,
    mut pazusers: *mut *mut libc::c_char,
    mut fnotusers: boolean,
    mut iold: libc::c_long,
    mut iyoung: libc::c_long,
    mut ccommands: libc::c_int,
    mut pazcommands: *mut *mut libc::c_char,
    mut fnotcommands: boolean,
    mut zcomment: *const libc::c_char,
    mut cstdin: libc::c_int,
) -> boolean {
    let mut fret: boolean = 0;
    if fsysdep_get_work_init(qsys, 'z' as i32, 0 as libc::c_int as libc::c_uint) == 0 {
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
        let mut itime: libc::c_long = 0;
        if fsysdep_get_work(qsys, 'z' as i32, 0 as libc::c_int as libc::c_uint, &mut s)
            == 0
        {
            usysdep_get_work_free(qsys);
            return 0 as libc::c_int;
        }
        if s.bcmd as libc::c_int == 'H' as i32 {
            break;
        }
        if cusers > 0 as libc::c_int {
            let mut fmatch: boolean = 0;
            let mut i: libc::c_int = 0;
            fmatch = fnotusers;
            i = 0 as libc::c_int;
            while i < cusers {
                if !(s.zuser).is_null()
                    && strcmp(*pazusers.offset(i as isize), s.zuser) == 0 as libc::c_int
                {
                    fmatch = (fmatch == 0) as libc::c_int;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if fmatch == 0 {
                continue;
            }
        }
        itime = ixsysdep_work_time(qsys, s.pseq);
        if iold != -(1 as libc::c_int) as libc::c_long && itime > iold {
            continue;
        }
        if iyoung != -(1 as libc::c_int) as libc::c_long && itime < iyoung {
            continue;
        }
        if fsworkfile_show(
            puuconf,
            icmd,
            qsys,
            &mut s,
            itime,
            ccommands,
            pazcommands,
            fnotcommands,
            zcomment,
            cstdin,
        ) == 0
        {
            usysdep_get_work_free(qsys);
            return 0 as libc::c_int;
        }
    }
    fret = fsworkfile_show(
        puuconf,
        icmd,
        qsys,
        0 as *mut libc::c_void as *const scmd,
        0 as libc::c_long,
        ccommands,
        pazcommands,
        fnotcommands,
        zcomment,
        cstdin,
    );
    usysdep_get_work_free(qsys);
    return fret;
}
unsafe extern "C" fn fsworkfile_show(
    mut puuconf: pointer,
    mut icmd: libc::c_int,
    mut qsys: *const uuconf_system,
    mut qcmd: *const scmd,
    mut itime: libc::c_long,
    mut ccommands: libc::c_int,
    mut pazcommands: *mut *mut libc::c_char,
    mut fnotcommands: boolean,
    mut zcomment: *const libc::c_char,
    mut cstdin: libc::c_int,
) -> boolean {
    static mut qlist: *mut scmdlist = 0 as *const scmdlist as *mut scmdlist;
    static mut zlistid: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut zid: *mut libc::c_char = 0 as *mut libc::c_char;
    if qcmd.is_null() {
        zid = 0 as *mut libc::c_char;
    } else {
        zid = zsysdep_jobid(qsys, (*qcmd).pseq);
        if zid.is_null() {
            return 0 as libc::c_int;
        }
    }
    if !qcmd.is_null() && !qlist.is_null() && strcmp(zlistid, zid) == 0 as libc::c_int {
        let mut qnew: *mut scmdlist = 0 as *mut scmdlist;
        let mut pq: *mut *mut scmdlist = 0 as *mut *mut scmdlist;
        ubuffree(zid);
        qnew = xmalloc(::std::mem::size_of::<scmdlist>() as libc::c_ulong)
            as *mut scmdlist;
        (*qnew).qnext = 0 as *mut scmdlist;
        (*qnew).s = *qcmd;
        (*qnew).itime = itime;
        pq = &mut qlist;
        while !(*pq).is_null() {
            pq = &mut (**pq).qnext;
        }
        *pq = qnew;
        return 1 as libc::c_int;
    }
    if !qlist.is_null() {
        let mut fmatch: boolean = 0;
        let mut zprog: *const libc::c_char = 0 as *const libc::c_char;
        let mut zcmd: *const libc::c_char = 0 as *const libc::c_char;
        let mut zrequestor: *const libc::c_char = 0 as *const libc::c_char;
        let mut zstdin: *const libc::c_char = 0 as *const libc::c_char;
        let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut qxqt: *mut scmdlist = 0 as *mut scmdlist;
        let mut exqt: *mut FILE = 0 as *mut FILE;
        let mut qfree: *mut scmdlist = 0 as *mut scmdlist;
        fmatch = 0 as libc::c_int;
        zstdin = 0 as *const libc::c_char;
        zrequestor = zstdin;
        zcmd = zrequestor;
        zprog = zcmd;
        zfree = 0 as *mut libc::c_char;
        qxqt = qlist;
        while !qxqt.is_null() {
            if (*qxqt).s.bcmd as libc::c_int == 'E' as i32 {
                break;
            }
            if (*qxqt).s.bcmd as libc::c_int == 'S' as i32
                && *((*qxqt).s.zto).offset(0 as libc::c_int as isize) as libc::c_int
                    == 'X' as i32
                && *((*qxqt).s.zto).offset(1 as libc::c_int as isize) as libc::c_int
                    == '.' as i32 && fspool_file((*qxqt).s.zfrom) != 0
            {
                let mut zxqt: *mut libc::c_char = 0 as *mut libc::c_char;
                zxqt = zsysdep_spool_file_name(qsys, (*qxqt).s.zfrom, (*qxqt).s.pseq);
                if zxqt.is_null() {
                    return 0 as libc::c_int;
                }
                exqt = fopen(zxqt, b"r\0" as *const u8 as *const libc::c_char);
                ubuffree(zxqt);
                if !exqt.is_null() {
                    break;
                }
            }
            qxqt = (*qxqt).qnext;
        }
        if qxqt.is_null() {
            if ccommands == 0 as libc::c_int
                || fnotcommands != 0
                    && strcmp(
                        *pazcommands.offset(0 as libc::c_int as isize),
                        b"ALL\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
            {
                fmatch = 1 as libc::c_int;
                if icmd & 0o1 as libc::c_int != 0 as libc::c_int {
                    let mut qshow: *mut scmdlist = 0 as *mut scmdlist;
                    qshow = qlist;
                    while !qshow.is_null() {
                        let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut cbytes: libc::c_long = 0;
                        usworkfile_header(
                            qsys,
                            &mut (*qshow).s,
                            zlistid,
                            (*qshow).itime,
                            (qshow == qlist) as libc::c_int,
                        );
                        match (*qshow).s.bcmd as libc::c_int {
                            83 => {
                                if !(strchr((*qshow).s.zoptions, 'C' as i32)).is_null()
                                    || fspool_file((*qshow).s.zfrom) != 0
                                {
                                    zfile = zsysdep_spool_file_name(
                                        qsys,
                                        (*qshow).s.ztemp,
                                        (*qshow).s.pseq,
                                    );
                                } else {
                                    zfile = zbufcpy((*qshow).s.zfrom);
                                }
                                if zfile.is_null() {
                                    cbytes = -(1 as libc::c_int) as libc::c_long;
                                } else {
                                    cbytes = csysdep_size(zfile);
                                }
                                if cbytes >= 0 as libc::c_int as libc::c_long {
                                    printf(
                                        b"Sending %s (%ld bytes) to %s\0" as *const u8
                                            as *const libc::c_char,
                                        (*qshow).s.zfrom,
                                        cbytes,
                                        (*qshow).s.zto,
                                    );
                                }
                                ubuffree(zfile);
                            }
                            82 => {
                                printf(
                                    b"Requesting %s to %s\0" as *const u8
                                        as *const libc::c_char,
                                    (*qshow).s.zfrom,
                                    (*qshow).s.zto,
                                );
                            }
                            88 => {
                                printf(
                                    b"Requesting %s to %s\0" as *const u8
                                        as *const libc::c_char,
                                    (*qshow).s.zfrom,
                                    (*qshow).s.zto,
                                );
                            }
                            80 => {
                                printf(
                                    b"(poll file)\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            _ => {
                                printf(
                                    b"Bad line %d\0" as *const u8 as *const libc::c_char,
                                    (*qshow).s.bcmd as libc::c_int,
                                );
                            }
                        }
                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                        qshow = (*qshow).qnext;
                    }
                }
            }
        } else {
            let mut csize: libc::c_long = 0;
            let mut qsize: *mut scmdlist = 0 as *mut scmdlist;
            if (*qxqt).s.bcmd as libc::c_int == 'E' as i32 {
                zfree = zbufcpy((*qxqt).s.zcmd);
                *zfree
                    .offset(
                        strcspn(zfree, b" \t\0" as *const u8 as *const libc::c_char)
                            as isize,
                    ) = '\0' as i32 as libc::c_char;
                zprog = zfree;
                zcmd = (*qxqt).s.zcmd;
                if !(strchr((*qxqt).s.zoptions, 'R' as i32)).is_null() {
                    zrequestor = (*qxqt).s.znotify;
                }
            } else {
                if fsxqt_file_read(puuconf, exqt) == 0 {
                    fclose(exqt);
                    return 0 as libc::c_int;
                }
                fclose(exqt);
                zprog = zSxqt_prog;
                zcmd = zSxqt_cmd;
                zrequestor = zSxqt_requestor;
            }
            csize = 0 as libc::c_long;
            qsize = qlist;
            while !qsize.is_null() {
                if (*qsize).s.bcmd as libc::c_int == 'S' as i32
                    || (*qsize).s.bcmd as libc::c_int == 'E' as i32
                {
                    let mut zfile_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    if !(strchr((*qsize).s.zoptions, 'C' as i32)).is_null()
                        || fspool_file((*qsize).s.zfrom) != 0
                    {
                        zfile_0 = zsysdep_spool_file_name(
                            qsys,
                            (*qsize).s.ztemp,
                            (*qsize).s.pseq,
                        );
                    } else {
                        zfile_0 = zbufcpy((*qsize).s.zfrom);
                    }
                    if !zfile_0.is_null() {
                        let mut cbytes_0: libc::c_long = 0;
                        cbytes_0 = csysdep_size(zfile_0);
                        if cbytes_0 > 0 as libc::c_int as libc::c_long {
                            csize += cbytes_0;
                        }
                        ubuffree(zfile_0);
                    }
                }
                qsize = (*qsize).qnext;
            }
            if ccommands == 0 as libc::c_int {
                fmatch = 1 as libc::c_int;
            } else {
                let mut i: libc::c_int = 0;
                fmatch = fnotcommands;
                i = 0 as libc::c_int;
                while i < ccommands {
                    if strcmp(
                        *pazcommands.offset(i as isize),
                        b"ALL\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                        || strcmp(*pazcommands.offset(i as isize), zprog)
                            == 0 as libc::c_int
                    {
                        fmatch = (fmatch == 0) as libc::c_int;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
            }
            if fmatch != 0 {
                let mut qstdin: *mut scmdlist = 0 as *mut scmdlist;
                if (*qxqt).s.bcmd as libc::c_int == 'E' as i32 {
                    qstdin = qxqt;
                } else if !zSxqt_stdin.is_null() {
                    qstdin = qlist;
                    while !qstdin.is_null() {
                        if (*qstdin).s.bcmd as libc::c_int == 'S' as i32
                            && strcmp((*qstdin).s.zto, zSxqt_stdin) == 0 as libc::c_int
                        {
                            break;
                        }
                        qstdin = (*qstdin).qnext;
                    }
                } else {
                    qstdin = 0 as *mut scmdlist;
                }
                if !qstdin.is_null() {
                    if !(strchr((*qstdin).s.zoptions, 'C' as i32)).is_null()
                        || fspool_file((*qstdin).s.zfrom) != 0
                    {
                        zstdin = (*qstdin).s.ztemp;
                    } else {
                        zstdin = (*qstdin).s.zfrom;
                    }
                }
            }
            if fmatch != 0 && icmd & 0o1 as libc::c_int != 0 as libc::c_int {
                usworkfile_header(
                    qsys,
                    &mut (*qxqt).s,
                    zlistid,
                    (*qxqt).itime,
                    1 as libc::c_int,
                );
                printf(
                    b"Executing %s (sending %ld bytes)\n\0" as *const u8
                        as *const libc::c_char,
                    zcmd,
                    csize,
                );
            }
        }
        if fmatch != 0 {
            let mut fkill_or_rejuv: boolean = 0;
            fkill_or_rejuv = 0 as libc::c_int;
            if icmd & 0o2 as libc::c_int != 0 as libc::c_int {
                let mut b: libc::c_int = 0;
                fprintf(
                    stderr,
                    b"%s: %s %s? \0" as *const u8 as *const libc::c_char,
                    zProgram,
                    if icmd & 0o10 as libc::c_int != 0 as libc::c_int {
                        b"Rejuvenate\0" as *const u8 as *const libc::c_char
                    } else {
                        b"Kill\0" as *const u8 as *const libc::c_char
                    },
                    zlistid,
                );
                fflush(stderr);
                b = getchar();
                fkill_or_rejuv = (b == 'y' as i32 || b == 'Y' as i32) as libc::c_int;
                while b != -(1 as libc::c_int) && b != '\n' as i32 {
                    b = getchar();
                }
            } else if icmd & 0o4 as libc::c_int != 0 as libc::c_int
                || icmd & 0o10 as libc::c_int != 0 as libc::c_int
            {
                fkill_or_rejuv = 1 as libc::c_int;
            }
            if fkill_or_rejuv != 0
                && (((*qlist).s.zuser).is_null()
                    || strcmp(zsysdep_login_name(), (*qlist).s.zuser)
                        != 0 as libc::c_int) && fsysdep_privileged() == 0
            {
                ulog(
                    LOG_ERROR,
                    b"%s: Not submitted by you\0" as *const u8 as *const libc::c_char,
                    zlistid,
                );
            } else {
                if icmd & (0o20 as libc::c_int | 0o40 as libc::c_int) != 0 as libc::c_int
                {
                    if fsnotify(
                        puuconf,
                        icmd,
                        zcomment,
                        cstdin,
                        (fkill_or_rejuv != 0
                            && icmd & 0o10 as libc::c_int == 0 as libc::c_int)
                            as libc::c_int,
                        zcmd,
                        qlist,
                        zlistid,
                        (*qlist).itime,
                        (*qlist).s.zuser,
                        qsys,
                        zstdin,
                        (*qlist).s.pseq,
                        zrequestor,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
                if fkill_or_rejuv != 0 {
                    if icmd & 0o10 as libc::c_int == 0 as libc::c_int {
                        if fsysdep_kill_job(puuconf, zlistid) == 0 {
                            return 0 as libc::c_int;
                        }
                    } else if fsysdep_rejuvenate_job(puuconf, zlistid) == 0 {
                        return 0 as libc::c_int
                    }
                }
            }
        }
        if !qxqt.is_null() {
            if (*qxqt).s.bcmd as libc::c_int == 'E' as i32 {
                ubuffree(zfree);
            } else {
                usxqt_file_free();
            }
        }
        qfree = qlist;
        while !qfree.is_null() {
            let mut qnext: *mut scmdlist = 0 as *mut scmdlist;
            qnext = (*qfree).qnext;
            xfree(qfree as pointer);
            qfree = qnext;
        }
        ubuffree(zlistid);
        qlist = 0 as *mut scmdlist;
        zlistid = 0 as *mut libc::c_char;
    }
    if !qcmd.is_null() {
        qlist = xmalloc(::std::mem::size_of::<scmdlist>() as libc::c_ulong)
            as *mut scmdlist;
        (*qlist).qnext = 0 as *mut scmdlist;
        (*qlist).s = *qcmd;
        (*qlist).itime = itime;
        zlistid = zid;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn usworkfile_header(
    mut qsys: *const uuconf_system,
    mut qcmd: *const scmd,
    mut zjobid: *const libc::c_char,
    mut itime: libc::c_long,
    mut ffirst: boolean,
) {
    let mut zshowid: *const libc::c_char = 0 as *const libc::c_char;
    let mut stime: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    if ffirst != 0 {
        zshowid = zjobid;
    } else {
        zshowid = b"-\0" as *const u8 as *const libc::c_char;
    }
    printf(
        b"%s %s %s \0" as *const u8 as *const libc::c_char,
        zshowid,
        (*qsys).uuconf_zname,
        if !((*qcmd).zuser).is_null() {
            (*qcmd).zuser
        } else {
            b"uucp\0" as *const u8 as *const libc::c_char
        },
    );
    usysdep_localtime(itime, &mut stime);
    printf(
        b"%02d-%02d %02d:%02d \0" as *const u8 as *const libc::c_char,
        stime.tm_mon + 1 as libc::c_int,
        stime.tm_mday,
        stime.tm_hour,
        stime.tm_min,
    );
}
unsafe extern "C" fn fsexecutions(
    mut puuconf: pointer,
    mut icmd: libc::c_int,
    mut csystems: libc::c_int,
    mut pazsystems: *mut *mut libc::c_char,
    mut fnotsystems: boolean,
    mut cusers: libc::c_int,
    mut pazusers: *mut *mut libc::c_char,
    mut fnotusers: boolean,
    mut iold: libc::c_long,
    mut iyoung: libc::c_long,
    mut ccommands: libc::c_int,
    mut pazcommands: *mut *mut libc::c_char,
    mut fnotcommands: boolean,
    mut zcomment: *const libc::c_char,
    mut cstdin: libc::c_int,
) -> boolean {
    let mut zlocalname: *const libc::c_char = 0 as *const libc::c_char;
    let mut iuuconf: libc::c_int = 0;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zsystem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ferr: boolean = 0;
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
    if fsysdep_get_xqt_init(0 as *mut libc::c_void as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    loop {
        zfile = zsysdep_get_xqt(
            0 as *mut libc::c_void as *const libc::c_char,
            &mut zsystem,
            &mut ferr,
        );
        if zfile.is_null() {
            break;
        }
        let mut fmatch: boolean = 0;
        let mut i: libc::c_int = 0;
        let mut itime: libc::c_long = 0;
        let mut e: *mut FILE = 0 as *mut FILE;
        if csystems > 0 as libc::c_int {
            fmatch = fnotsystems;
            i = 0 as libc::c_int;
            while i < csystems {
                if strcmp(*pazsystems.offset(i as isize), zsystem) == 0 as libc::c_int {
                    fmatch = (fmatch == 0) as libc::c_int;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if fmatch == 0 {
                ubuffree(zfile);
                ubuffree(zsystem);
                continue;
            }
        }
        itime = ixsysdep_file_time(zfile);
        if iold != -(1 as libc::c_int) as libc::c_long && itime > iold
            || iyoung != -(1 as libc::c_int) as libc::c_long && itime < iyoung
        {
            ubuffree(zfile);
            ubuffree(zsystem);
        } else {
            e = fopen(zfile, b"r\0" as *const u8 as *const libc::c_char);
            if e.is_null() {
                continue;
            }
            if fsxqt_file_read(puuconf, e) == 0 {
                fclose(e);
                ubuffree(zfile);
                ubuffree(zsystem);
            } else {
                fclose(e);
                if cusers == 0 as libc::c_int {
                    fmatch = 1 as libc::c_int;
                } else {
                    fmatch = fnotusers;
                    i = 0 as libc::c_int;
                    while i < cusers {
                        if strcmp(zSxqt_user, *pazusers.offset(i as isize))
                            == 0 as libc::c_int
                            || !zSxqt_requestor.is_null()
                                && strcmp(zSxqt_requestor, *pazusers.offset(i as isize))
                                    == 0 as libc::c_int
                        {
                            fmatch = (fmatch == 0) as libc::c_int;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                }
                if fmatch != 0 && ccommands > 0 as libc::c_int {
                    fmatch = fnotcommands;
                    i = 0 as libc::c_int;
                    while i < ccommands {
                        if strcmp(
                            *pazcommands.offset(i as isize),
                            b"ALL\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                            || strcmp(*pazcommands.offset(i as isize), zSxqt_prog)
                                == 0 as libc::c_int
                        {
                            fmatch = (fmatch == 0) as libc::c_int;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                }
                if fmatch != 0 {
                    let mut fbad: boolean = 0;
                    let mut fkill_or_rejuv: boolean = 0;
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
                    fbad = 0 as libc::c_int;
                    if icmd & 0o1 as libc::c_int != 0 as libc::c_int {
                        let mut stime: tm = tm {
                            tm_sec: 0,
                            tm_min: 0,
                            tm_hour: 0,
                            tm_mday: 0,
                            tm_mon: 0,
                            tm_year: 0,
                            tm_wday: 0,
                            tm_yday: 0,
                            tm_isdst: 0,
                            tm_gmtoff: 0,
                            tm_zone: 0 as *const libc::c_char,
                        };
                        printf(
                            b"%s %s!\0" as *const u8 as *const libc::c_char,
                            zsystem,
                            zSxqt_system,
                        );
                        if !zSxqt_requestor.is_null() {
                            printf(
                                b"%s\0" as *const u8 as *const libc::c_char,
                                zSxqt_requestor,
                            );
                        } else {
                            printf(
                                b"%s\0" as *const u8 as *const libc::c_char,
                                zSxqt_user,
                            );
                        }
                        usysdep_localtime(itime, &mut stime);
                        printf(
                            b" %02d-%02d %02d:%02d \0" as *const u8
                                as *const libc::c_char,
                            stime.tm_mon + 1 as libc::c_int,
                            stime.tm_mday,
                            stime.tm_hour,
                            stime.tm_min,
                        );
                        printf(b"%s\n\0" as *const u8 as *const libc::c_char, zSxqt_cmd);
                    }
                    fkill_or_rejuv = 0 as libc::c_int;
                    if icmd & 0o2 as libc::c_int != 0 as libc::c_int {
                        let mut b: libc::c_int = 0;
                        fprintf(
                            stderr,
                            b"%s: %s %s? \0" as *const u8 as *const libc::c_char,
                            zProgram,
                            if icmd & 0o10 as libc::c_int != 0 as libc::c_int {
                                b"Rejuvenate\0" as *const u8 as *const libc::c_char
                            } else {
                                b"Kill\0" as *const u8 as *const libc::c_char
                            },
                            zSxqt_cmd,
                        );
                        fflush(stderr);
                        b = getchar();
                        fkill_or_rejuv = (b == 'y' as i32 || b == 'Y' as i32)
                            as libc::c_int;
                        while b != -(1 as libc::c_int) && b != '\n' as i32 {
                            b = getchar();
                        }
                    } else if icmd & 0o4 as libc::c_int != 0 as libc::c_int
                        || icmd & 0o10 as libc::c_int != 0 as libc::c_int
                    {
                        fkill_or_rejuv = 1 as libc::c_int;
                    }
                    if fkill_or_rejuv != 0 {
                        if (strcmp(zSxqt_user, zsysdep_login_name()) != 0 as libc::c_int
                            || strcmp(zsystem, zlocalname) != 0 as libc::c_int)
                            && fsysdep_privileged() == 0
                        {
                            ulog(
                                LOG_ERROR,
                                b"Job not submitted by you\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fbad = 1 as libc::c_int;
                        }
                    }
                    if fbad == 0 {
                        iuuconf = uuconf_system_info(puuconf, zsystem, &mut ssys);
                        if iuuconf != 0 as libc::c_int {
                            if iuuconf != 1 as libc::c_int {
                                ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                                fbad = 1 as libc::c_int;
                            } else if strcmp(zsystem, zlocalname) == 0 as libc::c_int {
                                iuuconf = uuconf_system_local(puuconf, &mut ssys);
                                if iuuconf != 0 as libc::c_int {
                                    ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                                    fbad = 1 as libc::c_int;
                                }
                                ssys.uuconf_zname = zlocalname as *mut libc::c_char;
                            } else if funknown_system(puuconf, zsystem, &mut ssys) == 0 {
                                ulog(
                                    LOG_ERROR,
                                    b"Job for unknown system %s\0" as *const u8
                                        as *const libc::c_char,
                                    zsystem,
                                );
                                fbad = 1 as libc::c_int;
                            }
                        }
                    }
                    if fbad == 0
                        && icmd & (0o20 as libc::c_int | 0o40 as libc::c_int)
                            != 0 as libc::c_int
                    {
                        if fsnotify(
                            puuconf,
                            icmd,
                            zcomment,
                            cstdin,
                            (fkill_or_rejuv != 0
                                && icmd & 0o10 as libc::c_int == 0 as libc::c_int)
                                as libc::c_int,
                            zSxqt_cmd,
                            0 as *mut libc::c_void as *mut scmdlist,
                            0 as *mut libc::c_void as *const libc::c_char,
                            itime,
                            zSxqt_user,
                            &mut ssys,
                            zSxqt_stdin,
                            0 as *mut libc::c_void,
                            zSxqt_requestor,
                        ) == 0
                        {
                            ferr = 1 as libc::c_int;
                            usxqt_file_free();
                            ubuffree(zfile);
                            ubuffree(zsystem);
                            break;
                        }
                    }
                    if fbad == 0 && fkill_or_rejuv != 0 {
                        i = 0 as libc::c_int;
                        while i < cSxqt_files {
                            let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                            z = zsysdep_spool_file_name(
                                &mut ssys,
                                *pazSxqt_files.offset(i as isize),
                                0 as *mut libc::c_void,
                            );
                            if !z.is_null() {
                                if icmd & 0o10 as libc::c_int != 0 as libc::c_int {
                                    fsysdep_touch_file(z);
                                } else {
                                    remove(z);
                                }
                                ubuffree(z);
                            }
                            i += 1;
                            i;
                        }
                        if icmd & 0o10 as libc::c_int != 0 as libc::c_int {
                            fsysdep_touch_file(zfile);
                        } else if remove(zfile) != 0 as libc::c_int {
                            ulog(
                                LOG_ERROR,
                                b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
                                zfile,
                                strerror(*__errno_location()),
                            );
                        }
                    }
                    if fbad == 0 {
                        uuconf_free_block(ssys.uuconf_palloc);
                    }
                }
                usxqt_file_free();
                ubuffree(zfile);
                ubuffree(zsystem);
            }
        }
    }
    usysdep_get_xqt_free(0 as *mut libc::c_void as *const libc::c_char);
    return ferr;
}
unsafe extern "C" fn fsnotify(
    mut puuconf: pointer,
    mut icmd: libc::c_int,
    mut zcomment: *const libc::c_char,
    mut cstdin: libc::c_int,
    mut fkilled: boolean,
    mut zcmd: *const libc::c_char,
    mut qcmd: *mut scmdlist,
    mut zid: *const libc::c_char,
    mut itime: libc::c_long,
    mut zuser: *const libc::c_char,
    mut qsys: *const uuconf_system,
    mut zstdin: *const libc::c_char,
    mut pstdinseq: pointer,
    mut zrequestor: *const libc::c_char,
) -> boolean {
    let mut pz: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut cgot: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut istdin: libc::c_int = 0;
    let mut stime: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut ab: [libc::c_char; 20] = [0; 20];
    let mut zsubject: *const libc::c_char = 0 as *const libc::c_char;
    let mut fret: boolean = 0;
    pz = xmalloc(
        (20 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    cgot = 20 as libc::c_int;
    i = 0 as libc::c_int;
    if zid.is_null() {
        let fresh7 = i;
        i = i + 1;
        let ref mut fresh8 = *pz.offset(fresh7 as isize);
        *fresh8 = b"A UUCP execution request\0" as *const u8 as *const libc::c_char;
    } else {
        let fresh9 = i;
        i = i + 1;
        let ref mut fresh10 = *pz.offset(fresh9 as isize);
        *fresh10 = b"UUCP job\n\t\0" as *const u8 as *const libc::c_char;
        let fresh11 = i;
        i = i + 1;
        let ref mut fresh12 = *pz.offset(fresh11 as isize);
        *fresh12 = zid;
        let fresh13 = i;
        i = i + 1;
        let ref mut fresh14 = *pz.offset(fresh13 as isize);
        *fresh14 = b"\nfor system\n\t\0" as *const u8 as *const libc::c_char;
        let fresh15 = i;
        i = i + 1;
        let ref mut fresh16 = *pz.offset(fresh15 as isize);
        *fresh16 = (*qsys).uuconf_zname;
    }
    let fresh17 = i;
    i = i + 1;
    let ref mut fresh18 = *pz.offset(fresh17 as isize);
    *fresh18 = b"\nrequested by\n\t\0" as *const u8 as *const libc::c_char;
    let fresh19 = i;
    i = i + 1;
    let ref mut fresh20 = *pz.offset(fresh19 as isize);
    *fresh20 = if !zuser.is_null() {
        zuser
    } else {
        b"uucp\0" as *const u8 as *const libc::c_char
    };
    if zid.is_null() {
        let fresh21 = i;
        i = i + 1;
        let ref mut fresh22 = *pz.offset(fresh21 as isize);
        *fresh22 = b"\non system\n\t\0" as *const u8 as *const libc::c_char;
        let fresh23 = i;
        i = i + 1;
        let ref mut fresh24 = *pz.offset(fresh23 as isize);
        *fresh24 = (*qsys).uuconf_zname;
    }
    let fresh25 = i;
    i = i + 1;
    let ref mut fresh26 = *pz.offset(fresh25 as isize);
    *fresh26 = b"\n\0" as *const u8 as *const libc::c_char;
    if fkilled != 0 {
        let fresh27 = i;
        i = i + 1;
        let ref mut fresh28 = *pz.offset(fresh27 as isize);
        *fresh28 = b"has been killed.\n\0" as *const u8 as *const libc::c_char;
    }
    if !zcomment.is_null() {
        let fresh29 = i;
        i = i + 1;
        let ref mut fresh30 = *pz.offset(fresh29 as isize);
        *fresh30 = zcomment;
        let fresh31 = i;
        i = i + 1;
        let ref mut fresh32 = *pz.offset(fresh31 as isize);
        *fresh32 = b"\n\0" as *const u8 as *const libc::c_char;
    }
    let fresh33 = i;
    i = i + 1;
    let ref mut fresh34 = *pz.offset(fresh33 as isize);
    *fresh34 = b"The job was queued at \0" as *const u8 as *const libc::c_char;
    usysdep_localtime(itime, &mut stime);
    sprintf(
        ab.as_mut_ptr(),
        b"%04d-%02d-%02d %02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
        stime.tm_year + 1900 as libc::c_int,
        stime.tm_mon + 1 as libc::c_int,
        stime.tm_mday,
        stime.tm_hour,
        stime.tm_min,
        stime.tm_sec,
    );
    let fresh35 = i;
    i = i + 1;
    let ref mut fresh36 = *pz.offset(fresh35 as isize);
    *fresh36 = ab.as_mut_ptr();
    let fresh37 = i;
    i = i + 1;
    let ref mut fresh38 = *pz.offset(fresh37 as isize);
    *fresh38 = b".\nIt \0" as *const u8 as *const libc::c_char;
    if fkilled != 0 {
        let fresh39 = i;
        i = i + 1;
        let ref mut fresh40 = *pz.offset(fresh39 as isize);
        *fresh40 = b"was\n\0" as *const u8 as *const libc::c_char;
    } else {
        let fresh41 = i;
        i = i + 1;
        let ref mut fresh42 = *pz.offset(fresh41 as isize);
        *fresh42 = b"is\n\0" as *const u8 as *const libc::c_char;
    }
    if !zcmd.is_null() {
        let fresh43 = i;
        i = i + 1;
        let ref mut fresh44 = *pz.offset(fresh43 as isize);
        *fresh44 = b"\t\0" as *const u8 as *const libc::c_char;
        let fresh45 = i;
        i = i + 1;
        let ref mut fresh46 = *pz.offset(fresh45 as isize);
        *fresh46 = zcmd;
    } else {
        let mut qshow: *mut scmdlist = 0 as *mut scmdlist;
        qshow = qcmd;
        while !qshow.is_null() {
            if i + 10 as libc::c_int > cgot {
                cgot += 20 as libc::c_int;
                pz = xrealloc(
                    pz as pointer,
                    (cgot as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *const libc::c_char;
            }
            match (*qshow).s.bcmd as libc::c_int {
                83 => {
                    let fresh47 = i;
                    i = i + 1;
                    let ref mut fresh48 = *pz.offset(fresh47 as isize);
                    *fresh48 = b"\tsend \0" as *const u8 as *const libc::c_char;
                }
                80 => {
                    let fresh51 = i;
                    i = i + 1;
                    let ref mut fresh52 = *pz.offset(fresh51 as isize);
                    *fresh52 = b"\tpoll \0" as *const u8 as *const libc::c_char;
                }
                69 => {
                    ulog(
                        LOG_FATAL,
                        b"fsnotify: Can't happen\0" as *const u8 as *const libc::c_char,
                    );
                }
                82 | 88 | _ => {
                    let fresh49 = i;
                    i = i + 1;
                    let ref mut fresh50 = *pz.offset(fresh49 as isize);
                    *fresh50 = b"\trequest \0" as *const u8 as *const libc::c_char;
                }
            }
            if !((*qshow).s.zfrom).is_null() && !((*qshow).s.zto).is_null() {
                let fresh53 = i;
                i = i + 1;
                let ref mut fresh54 = *pz.offset(fresh53 as isize);
                *fresh54 = (*qshow).s.zfrom;
                let fresh55 = i;
                i = i + 1;
                let ref mut fresh56 = *pz.offset(fresh55 as isize);
                *fresh56 = b" to \0" as *const u8 as *const libc::c_char;
                let fresh57 = i;
                i = i + 1;
                let ref mut fresh58 = *pz.offset(fresh57 as isize);
                *fresh58 = (*qshow).s.zto;
            }
            qshow = (*qshow).qnext;
        }
    }
    istdin = i;
    if cstdin > 0 as libc::c_int && !zstdin.is_null() {
        let mut fspool: boolean = 0;
        let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut e: *mut FILE = 0 as *mut FILE;
        fspool = fspool_file(zstdin);
        if fspool != 0 {
            zfile = zsysdep_spool_file_name(qsys, zstdin, pstdinseq);
        } else {
            zfile = zsysdep_local_file(
                zstdin,
                (*qsys).uuconf_zpubdir,
                0 as *mut libc::c_void as *mut boolean,
            );
        }
        if !zfile.is_null()
            && (fspool != 0
                || fin_directory_list(
                    zfile,
                    (*qsys).uuconf_pzremote_send,
                    (*qsys).uuconf_zpubdir,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    0 as *mut libc::c_void as *const libc::c_char,
                ) != 0)
        {
            e = fopen(zfile, b"r\0" as *const u8 as *const libc::c_char);
            if !e.is_null() {
                let mut clines: libc::c_int = 0;
                let mut clen: libc::c_int = 0;
                let mut zline: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cline: size_t = 0;
                let fresh59 = i;
                i = i + 1;
                let ref mut fresh60 = *pz.offset(fresh59 as isize);
                *fresh60 = b"\n\0" as *const u8 as *const libc::c_char;
                istdin = i;
                clines = 0 as libc::c_int;
                zline = 0 as *mut libc::c_char;
                cline = 0 as libc::c_int as size_t;
                loop {
                    clen = getline(&mut zline, &mut cline, e) as libc::c_int;
                    if !(clen > 0 as libc::c_int) {
                        break;
                    }
                    if !(memchr(
                        zline as *const libc::c_void,
                        '\0' as i32,
                        clen as size_t,
                    ))
                        .is_null()
                    {
                        let mut ifree: libc::c_int = 0;
                        ifree = istdin;
                        while ifree < i {
                            ubuffree(*pz.offset(ifree as isize) as *mut libc::c_char);
                            ifree += 1;
                            ifree;
                        }
                        i = istdin - 1 as libc::c_int;
                        break;
                    } else {
                        clines += 1;
                        clines;
                        if clines > cstdin {
                            break;
                        }
                        if i >= cgot {
                            cgot += 20 as libc::c_int;
                            pz = xrealloc(
                                pz as pointer,
                                (cgot as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                    ),
                            ) as *mut *const libc::c_char;
                        }
                        if strncmp(
                            zline,
                            b"From \0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int
                        {
                            let fresh61 = i;
                            i = i + 1;
                            let ref mut fresh62 = *pz.offset(fresh61 as isize);
                            *fresh62 = zbufcpy(zline);
                        } else {
                            let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
                            zalc = zbufalc(
                                (strlen(zline))
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
                            );
                            *zalc
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = '>' as i32 as libc::c_char;
                            strcpy(zalc.offset(1 as libc::c_int as isize), zline);
                            let fresh63 = i;
                            i = i + 1;
                            let ref mut fresh64 = *pz.offset(fresh63 as isize);
                            *fresh64 = zalc;
                        }
                    }
                }
                xfree(zline as pointer);
                fclose(e);
            }
        }
        ubuffree(zfile);
    }
    if fkilled != 0 {
        zsubject = b"UUCP job killed\0" as *const u8 as *const libc::c_char;
    } else {
        zsubject = b"UUCP notification\0" as *const u8 as *const libc::c_char;
    }
    fret = 1 as libc::c_int;
    if icmd & 0o20 as libc::c_int != 0 as libc::c_int {
        if fsysdep_mail(b"uucp\0" as *const u8 as *const libc::c_char, zsubject, i, pz)
            == 0
        {
            fret = 0 as libc::c_int;
        }
    }
    if icmd & 0o40 as libc::c_int != 0 as libc::c_int
        && (!zrequestor.is_null() || !zuser.is_null())
    {
        let mut zmail: *const libc::c_char = 0 as *const libc::c_char;
        let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
        if !zrequestor.is_null() {
            zmail = zrequestor;
        } else {
            zmail = zuser;
        }
        zfree = 0 as *mut libc::c_char;
        if zid.is_null() {
            let mut iuuconf: libc::c_int = 0;
            let mut zloc: *const libc::c_char = 0 as *const libc::c_char;
            iuuconf = uuconf_localname(puuconf, &mut zloc);
            if iuuconf == 1 as libc::c_int {
                zloc = zsysdep_localname();
                if zloc.is_null() {
                    return 0 as libc::c_int;
                }
            } else if iuuconf != 0 as libc::c_int {
                ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
            }
            if strcmp((*qsys).uuconf_zname, zloc) != 0 as libc::c_int
                && (strchr(zmail, '@' as i32)).is_null()
            {
                zfree = zbufalc(
                    (strlen((*qsys).uuconf_zname))
                        .wrapping_add(strlen(zmail))
                        .wrapping_add(
                            ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                        ),
                );
                sprintf(
                    zfree,
                    b"%s!%s\0" as *const u8 as *const libc::c_char,
                    (*qsys).uuconf_zname,
                    zmail,
                );
                zmail = zfree;
            }
        }
        if fsysdep_mail(zmail, zsubject, i, pz) == 0 {
            fret = 0 as libc::c_int;
        }
        ubuffree(zfree);
    }
    while istdin < i {
        ubuffree(*pz.offset(istdin as isize) as *mut libc::c_char);
        istdin += 1;
        istdin;
    }
    xfree(pz as pointer);
    return fret;
}
unsafe extern "C" fn fsquery(
    mut puuconf: pointer,
    mut csystems: libc::c_int,
    mut pazsystems: *mut *mut libc::c_char,
    mut fnotsystems: boolean,
    mut iold: libc::c_long,
    mut iyoung: libc::c_long,
) -> boolean {
    let mut iuuconf: libc::c_int = 0;
    let mut zlocalname: *const libc::c_char = 0 as *const libc::c_char;
    let mut qlist: *mut sxqtlist = 0 as *mut sxqtlist;
    let mut zfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zsystem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ferr: boolean = 0;
    let mut inow: libc::c_long = 0;
    let mut pznames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fret: boolean = 0;
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
    if fsysdep_get_xqt_init(0 as *mut libc::c_void as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    qlist = 0 as *mut sxqtlist;
    loop {
        zfile = zsysdep_get_xqt(
            0 as *mut libc::c_void as *const libc::c_char,
            &mut zsystem,
            &mut ferr,
        );
        if zfile.is_null() {
            break;
        }
        let mut qlook: *mut sxqtlist = 0 as *mut sxqtlist;
        qlook = qlist;
        while !qlook.is_null() {
            if strcmp(zsystem, (*qlook).zsystem) == 0 as libc::c_int {
                break;
            }
            qlook = (*qlook).qnext;
        }
        if !qlook.is_null() {
            let mut itime: libc::c_long = 0;
            ubuffree(zsystem);
            (*qlook).cxqts += 1;
            (*qlook).cxqts;
            itime = ixsysdep_file_time(zfile);
            if itime < (*qlook).ifirst {
                (*qlook).ifirst = itime;
            }
        } else {
            let mut qnew: *mut sxqtlist = 0 as *mut sxqtlist;
            qnew = xmalloc(::std::mem::size_of::<sxqtlist>() as libc::c_ulong)
                as *mut sxqtlist;
            (*qnew).qnext = qlist;
            (*qnew).zsystem = zsystem;
            (*qnew).cxqts = 1 as libc::c_int;
            (*qnew).ifirst = ixsysdep_file_time(zfile);
            qlist = qnew;
        }
        ubuffree(zfile);
    }
    usysdep_get_xqt_free(0 as *mut libc::c_void as *const libc::c_char);
    if ferr != 0 {
        return 0 as libc::c_int;
    }
    inow = ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long);
    iuuconf = uuconf_system_names(puuconf, &mut pznames, 0 as libc::c_int);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
        return 0 as libc::c_int;
    }
    fret = 1 as libc::c_int;
    pz = pznames;
    while !(*pz).is_null() {
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
        iuuconf = uuconf_system_info(puuconf, *pz, &mut ssys);
        if iuuconf != 0 as libc::c_int {
            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
            fret = 0 as libc::c_int;
        } else {
            if fsquery_system(
                &mut ssys,
                &mut qlist,
                inow,
                zlocalname,
                csystems,
                pazsystems,
                fnotsystems,
                iold,
                iyoung,
            ) == 0
            {
                fret = 0 as libc::c_int;
            }
            uuconf_free_block(ssys.uuconf_palloc);
            xfree(*pz as pointer);
        }
        pz = pz.offset(1);
        pz;
    }
    if !qlist.is_null() {
        let mut pq: *mut *mut sxqtlist = 0 as *mut *mut sxqtlist;
        pq = &mut qlist;
        while !(*pq).is_null() {
            if strcmp((**pq).zsystem, zlocalname) == 0 as libc::c_int {
                let mut ssys_0: uuconf_system = uuconf_system {
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
                let mut qfree: *mut sxqtlist = 0 as *mut sxqtlist;
                iuuconf = uuconf_system_info(puuconf, zlocalname, &mut ssys_0);
                if iuuconf != 0 as libc::c_int {
                    if iuuconf != 1 as libc::c_int {
                        ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                        fret = 0 as libc::c_int;
                        break;
                    } else {
                        iuuconf = uuconf_system_local(puuconf, &mut ssys_0);
                        if iuuconf != 0 as libc::c_int {
                            ulog_uuconf(LOG_ERROR, puuconf, iuuconf);
                            fret = 0 as libc::c_int;
                            break;
                        } else {
                            ssys_0.uuconf_zname = zlocalname as *mut libc::c_char;
                        }
                    }
                }
                if fsquery_show(
                    &mut ssys_0,
                    0 as libc::c_int,
                    0 as libc::c_long,
                    *pq,
                    inow,
                    zlocalname,
                    csystems,
                    pazsystems,
                    fnotsystems,
                    iold,
                    iyoung,
                ) == 0
                {
                    fret = 0 as libc::c_int;
                }
                uuconf_free_block(ssys_0.uuconf_palloc);
                qfree = *pq;
                *pq = (*qfree).qnext;
                ubuffree((*qfree).zsystem);
                xfree(qfree as pointer);
                break;
            } else {
                pq = &mut (**pq).qnext;
            }
        }
    }
    while !qlist.is_null() {
        let mut ssys_1: uuconf_system = uuconf_system {
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
        let mut qnext: *mut sxqtlist = 0 as *mut sxqtlist;
        if funknown_system(puuconf, (*qlist).zsystem, &mut ssys_1) == 0 {
            ulog(
                LOG_ERROR,
                b"Executions queued up for unknown systems\0" as *const u8
                    as *const libc::c_char,
            );
            fret = 0 as libc::c_int;
            break;
        } else {
            if fsquery_show(
                &mut ssys_1,
                0 as libc::c_int,
                0 as libc::c_long,
                qlist,
                inow,
                zlocalname,
                csystems,
                pazsystems,
                fnotsystems,
                iold,
                iyoung,
            ) == 0
            {
                fret = 0 as libc::c_int;
            }
            uuconf_free_block(ssys_1.uuconf_palloc);
            qnext = (*qlist).qnext;
            ubuffree((*qlist).zsystem);
            xfree(qlist as pointer);
            qlist = qnext;
        }
    }
    return fret;
}
unsafe extern "C" fn fsquery_system(
    mut qsys: *const uuconf_system,
    mut pq: *mut *mut sxqtlist,
    mut inow: libc::c_long,
    mut zlocalname: *const libc::c_char,
    mut csystems: libc::c_int,
    mut pazsystems: *mut *mut libc::c_char,
    mut fnotsystems: boolean,
    mut iold: libc::c_long,
    mut iyoung: libc::c_long,
) -> boolean {
    let mut cwork: libc::c_int = 0;
    let mut ifirstwork: libc::c_long = 0;
    let mut zid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fret: boolean = 0;
    if fsysdep_get_work_init(qsys, 'z' as i32, 0 as libc::c_int as libc::c_uint) == 0 {
        return 0 as libc::c_int;
    }
    cwork = 0 as libc::c_int;
    ifirstwork = 0 as libc::c_long;
    zid = 0 as *mut libc::c_char;
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
        let mut itime: libc::c_long = 0;
        let mut zthisid: *mut libc::c_char = 0 as *mut libc::c_char;
        if fsysdep_get_work(qsys, 'z' as i32, 0 as libc::c_int as libc::c_uint, &mut s)
            == 0
        {
            return 0 as libc::c_int;
        }
        if s.bcmd as libc::c_int == 'H' as i32 {
            break;
        }
        zthisid = zsysdep_jobid(qsys, s.pseq);
        if !zid.is_null() && strcmp(zid, zthisid) == 0 as libc::c_int {
            ubuffree(zthisid);
        } else {
            cwork += 1;
            cwork;
            ubuffree(zid);
            zid = zthisid;
        }
        itime = ixsysdep_work_time(qsys, s.pseq);
        if ifirstwork == 0 as libc::c_long || ifirstwork > itime {
            ifirstwork = itime;
        }
    }
    usysdep_get_work_free(qsys);
    ubuffree(zid);
    while !(*pq).is_null() {
        if strcmp((**pq).zsystem, (*qsys).uuconf_zname) == 0 as libc::c_int {
            break;
        }
        pq = &mut (**pq).qnext;
    }
    if cwork == 0 as libc::c_int && (*pq).is_null() {
        return 1 as libc::c_int;
    }
    fret = fsquery_show(
        qsys,
        cwork,
        ifirstwork,
        *pq,
        inow,
        zlocalname,
        csystems,
        pazsystems,
        fnotsystems,
        iold,
        iyoung,
    );
    if !(*pq).is_null() {
        let mut qfree: *mut sxqtlist = 0 as *mut sxqtlist;
        qfree = *pq;
        *pq = (*qfree).qnext;
        ubuffree((*qfree).zsystem);
        xfree(qfree as pointer);
    }
    return fret;
}
unsafe extern "C" fn fsquery_show(
    mut qsys: *const uuconf_system,
    mut cwork: libc::c_int,
    mut ifirstwork: libc::c_long,
    mut qxqt: *mut sxqtlist,
    mut inow: libc::c_long,
    mut zlocalname: *const libc::c_char,
    mut csystems: libc::c_int,
    mut pazsystems: *mut *mut libc::c_char,
    mut fnotsystems: boolean,
    mut iold: libc::c_long,
    mut iyoung: libc::c_long,
) -> boolean {
    let mut flocal: boolean = 0;
    let mut sstat: sstatus = sstatus {
        ttype: STATUS_COMPLETE,
        cretries: 0,
        ilast: 0,
        cwait: 0,
        zstring: 0 as *mut libc::c_char,
    };
    let mut fnostatus: boolean = 0;
    let mut stime: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut cpad: libc::c_int = 0;
    if csystems > 0 as libc::c_int {
        let mut fmatch: boolean = 0;
        let mut i: libc::c_int = 0;
        fmatch = fnotsystems;
        i = 0 as libc::c_int;
        while i < csystems {
            if strcmp(*pazsystems.offset(i as isize), (*qsys).uuconf_zname)
                == 0 as libc::c_int
            {
                fmatch = (fmatch == 0) as libc::c_int;
                break;
            } else {
                i += 1;
                i;
            }
        }
        if fmatch == 0 {
            return 1 as libc::c_int;
        }
    }
    if iold != -(1 as libc::c_int) as libc::c_long
        && (cwork == 0 as libc::c_int || ifirstwork > iold)
        && (qxqt.is_null() || (*qxqt).ifirst > iold)
        || iyoung != -(1 as libc::c_int) as libc::c_long
            && (cwork == 0 as libc::c_int || ifirstwork < iyoung)
            && (qxqt.is_null() || (*qxqt).ifirst < iyoung)
    {
        return 1 as libc::c_int;
    }
    flocal = (strcmp((*qsys).uuconf_zname, zlocalname) == 0 as libc::c_int)
        as libc::c_int;
    if flocal == 0 {
        if fsysdep_get_status(qsys, &mut sstat, &mut fnostatus) == 0 {
            return 0 as libc::c_int;
        }
    }
    printf(
        b"%-10s %3dC (\0" as *const u8 as *const libc::c_char,
        (*qsys).uuconf_zname,
        cwork,
    );
    if cwork == 0 as libc::c_int {
        printf(b"0 secs\0" as *const u8 as *const libc::c_char);
        cpad = 3 as libc::c_int;
    } else {
        cpad = csunits_show(inow - ifirstwork);
    }
    printf(b") \0" as *const u8 as *const libc::c_char);
    loop {
        let fresh65 = cpad;
        cpad = cpad - 1;
        if !(fresh65 != 0 as libc::c_int) {
            break;
        }
        printf(b" \0" as *const u8 as *const libc::c_char);
    }
    if qxqt.is_null() {
        printf(b"  0X (0 secs)   \0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"%3dX (\0" as *const u8 as *const libc::c_char, (*qxqt).cxqts);
        cpad = csunits_show(inow - (*qxqt).ifirst);
        printf(b")\0" as *const u8 as *const libc::c_char);
        loop {
            let fresh66 = cpad;
            cpad = cpad - 1;
            if !(fresh66 != 0 as libc::c_int) {
                break;
            }
            printf(b" \0" as *const u8 as *const libc::c_char);
        }
    }
    if flocal != 0 || fnostatus != 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        if flocal == 0 {
            ubuffree(sstat.zstring);
        }
        return 1 as libc::c_int;
    }
    usysdep_localtime(sstat.ilast, &mut stime);
    printf(
        b" %02d-%02d %02d:%02d \0" as *const u8 as *const libc::c_char,
        stime.tm_mon + 1 as libc::c_int,
        stime.tm_mday,
        stime.tm_hour,
        stime.tm_min,
    );
    if (sstat.zstring).is_null() {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            *azStatus.as_mut_ptr().offset(sstat.ttype as libc::c_int as isize),
        );
    } else {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, sstat.zstring);
        ubuffree(sstat.zstring);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn csunits_show(mut idiff: libc::c_long) -> libc::c_int {
    let mut zunit: *const libc::c_char = 0 as *const libc::c_char;
    let mut iunits: libc::c_long = 0;
    let mut cpad: libc::c_int = 0;
    if idiff
        > 24 as libc::c_int as libc::c_long * 60 as libc::c_int as libc::c_long
            * 60 as libc::c_int as libc::c_long
    {
        iunits = idiff
            / (24 as libc::c_int as libc::c_long * 60 as libc::c_int as libc::c_long
                * 60 as libc::c_int as libc::c_long);
        zunit = b"day\0" as *const u8 as *const libc::c_char;
        cpad = 4 as libc::c_int;
    } else if idiff
        > 60 as libc::c_int as libc::c_long * 60 as libc::c_int as libc::c_long
    {
        iunits = idiff / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long;
        zunit = b"hour\0" as *const u8 as *const libc::c_char;
        cpad = 3 as libc::c_int;
    } else if idiff > 60 as libc::c_int as libc::c_long {
        iunits = idiff / 60 as libc::c_int as libc::c_long;
        zunit = b"min\0" as *const u8 as *const libc::c_char;
        cpad = 4 as libc::c_int;
    } else {
        iunits = idiff;
        zunit = b"sec\0" as *const u8 as *const libc::c_char;
        cpad = 4 as libc::c_int;
    }
    printf(
        b"%ld %s%s\0" as *const u8 as *const libc::c_char,
        iunits,
        zunit,
        if iunits == 1 as libc::c_int as libc::c_long {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"s\0" as *const u8 as *const libc::c_char
        },
    );
    if iunits != 1 as libc::c_int as libc::c_long {
        cpad -= 1;
        cpad;
    }
    if iunits > 99 as libc::c_int as libc::c_long {
        cpad -= 1;
        cpad;
    }
    if iunits > 9 as libc::c_int as libc::c_long {
        cpad -= 1;
        cpad;
    }
    return cpad;
}
unsafe extern "C" fn fsmachines() -> boolean {
    let mut phold: pointer = 0 as *mut libc::c_void;
    let mut zsystem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ferr: boolean = 0;
    let mut sstat: sstatus = sstatus {
        ttype: STATUS_COMPLETE,
        cretries: 0,
        ilast: 0,
        cwait: 0,
        zstring: 0 as *mut libc::c_char,
    };
    if fsysdep_all_status_init(&mut phold) == 0 {
        return 0 as libc::c_int;
    }
    loop {
        zsystem = zsysdep_all_status(phold, &mut ferr, &mut sstat);
        if zsystem.is_null() {
            break;
        }
        let mut stime: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        usysdep_localtime(sstat.ilast, &mut stime);
        printf(
            b"%-14s %02d-%02d %02d:%02d \0" as *const u8 as *const libc::c_char,
            zsystem,
            stime.tm_mon + 1 as libc::c_int,
            stime.tm_mday,
            stime.tm_hour,
            stime.tm_min,
        );
        if (sstat.zstring).is_null() {
            printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                *azStatus.as_mut_ptr().offset(sstat.ttype as libc::c_int as isize),
            );
        } else {
            printf(b"%s\0" as *const u8 as *const libc::c_char, sstat.zstring);
            ubuffree(sstat.zstring);
        }
        ubuffree(zsystem);
        if sstat.ttype as libc::c_uint != STATUS_TALKING as libc::c_int as libc::c_uint
            && sstat.cwait > 0 as libc::c_int
        {
            printf(
                b" (%d %s\0" as *const u8 as *const libc::c_char,
                sstat.cretries,
                if sstat.cretries == 1 as libc::c_int {
                    b"try\0" as *const u8 as *const libc::c_char
                } else {
                    b"tries\0" as *const u8 as *const libc::c_char
                },
            );
            if sstat.ilast + sstat.cwait as libc::c_long
                > ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
            {
                usysdep_localtime(sstat.ilast + sstat.cwait as libc::c_long, &mut stime);
                printf(
                    b", next after %02d-%02d %02d:%02d\0" as *const u8
                        as *const libc::c_char,
                    stime.tm_mon + 1 as libc::c_int,
                    stime.tm_mday,
                    stime.tm_hour,
                    stime.tm_min,
                );
            }
            printf(b")\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    usysdep_all_status_free(phold);
    return (ferr == 0) as libc::c_int;
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
